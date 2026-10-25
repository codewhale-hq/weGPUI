#!/usr/bin/env python3
"""Exercise the local browser WeUI playground; this does not run native Rust/GPUI."""
import argparse
from contextlib import contextmanager
from datetime import datetime, timezone
import hashlib
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
import json
from pathlib import Path
import re
import subprocess
import tempfile
import threading
from urllib.parse import urlencode, urljoin, urlparse

from playwright.sync_api import expect, sync_playwright

ROOT = Path(__file__).resolve().parents[1]
VIEWPORTS = (375, 768, 1280)
PNG = bytes.fromhex(
    '89504e470d0a1a0a0000000d49484452000000010000000108060000001f15c489'
    '0000000b49444154789c636000020000050001a5f645400000000049454e44ae426082'
)


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


@contextmanager
def server(site, project, log):
    with tempfile.TemporaryDirectory(prefix='wegpui-web-check-') as temporary:
        directory = Path(temporary)
        if project:
            (directory / 'weGPUI').symlink_to(site, target_is_directory=True)
        else:
            directory = site

        class Handler(SimpleHTTPRequestHandler):
            def __init__(self, *args, **kwargs):
                super().__init__(*args, directory=str(directory), **kwargs)

            def log_message(self, fmt, *args):
                with log.open('a') as output:
                    output.write(fmt % args + '\n')

        http = ThreadingHTTPServer(('127.0.0.1', 0), Handler)
        thread = threading.Thread(target=http.serve_forever, daemon=True)
        thread.start()
        try:
            yield f'http://127.0.0.1:{http.server_port}/' + ('weGPUI/' if project else '')
        finally:
            http.shutdown()
            http.server_close()
            thread.join(timeout=5)


class Checks:
    def __init__(self, context, page, base, out, report):
        self.context, self.page, self.base, self.out, self.report = context, page, base, out, report
        self.case = 'startup'
        self.problems = []
        self.actions = []
        self.page.on('pageerror', lambda error: self.problem('pageerror', str(error)))
        self.page.on('console', lambda message: self.problem('console', message.text)
                     if message.type == 'error' else None)
        self.page.on('requestfailed', lambda request: self.problem('requestfailed',
                     {'url': request.url, 'failure': request.failure}))
        self.page.on('response', lambda response: self.problem('http',
                     {'url': response.url, 'status': response.status}) if response.status >= 400 else None)
        self.context.route('**/*', self.local_only)
        self.page.add_init_script("""window.__webCheckMessages = [];
            addEventListener('message', event => {
              if (event.data && String(event.data.type).startsWith('wegpui:'))
                window.__webCheckMessages.push(event.data);
            });""")

    def local_only(self, route):
        request = route.request
        parsed, allowed = urlparse(request.url), urlparse(self.base)
        if parsed.scheme in ('blob', 'data', 'about'):
            route.continue_()
        elif (parsed.scheme, parsed.netloc) != (allowed.scheme, allowed.netloc):
            self.problem('outside-network-request', {'url': request.url, 'method': request.method})
            route.abort()
        elif request.method not in ('GET', 'HEAD'):
            self.problem('unexpected-write-request', {'url': request.url, 'method': request.method})
            route.abort()
        else:
            route.continue_()

    def problem(self, kind, detail):
        item = {'case': self.case, 'kind': kind, 'detail': detail}
        self.problems.append(item)
        (self.out / 'browser-problems.json').write_text(json.dumps(self.problems, indent=2) + '\n')

    def record(self, label, **details):
        self.actions.append({'case': self.case, 'label': label, **details})
        (self.out / 'actions.json').write_text(json.dumps(self.actions, ensure_ascii=False, indent=2) + '\n')

    def run(self, name, callback):
        self.case = name
        start = len(self.problems)
        try:
            detail = callback()
            assert len(self.problems) == start, self.problems[start:]
            result = {'name': name, 'result': 'PASS', 'detail': detail}
        except Exception as error:
            result = {'name': name, 'result': 'FAIL', 'error': repr(error),
                      'problems': self.problems[start:]}
            try:
                self.page.screenshot(path=str(self.out / f'failed-{name}.png'), full_page=True)
                (self.out / f'failed-{name}.html').write_text(self.page.content())
            except Exception as capture_error:
                result['capture_error'] = repr(capture_error)
        self.report['checks'].append(result)
        self.save()
        print(json.dumps(result, ensure_ascii=False), flush=True)

    def save(self):
        (self.out / 'browser-checks.json').write_text(json.dumps(self.report, ensure_ascii=False, indent=2) + '\n')

    def route(self, name, *, theme='light', care=False, brand='07c160', state='default'):
        query = urlencode({'theme': theme, 'care': int(care), 'brand': brand, 'state': state})
        target = urljoin(self.base, 'playground/index.html') + '#' + name + '?' + query
        same_route = urlparse(self.page.url).fragment.split('?', 1)[0] == name
        self.page.goto(target, wait_until='networkidle')
        if same_route:
            self.page.reload(wait_until='networkidle')
        expect(self.page.locator('#container .page').first).to_be_visible()
        self.page.wait_for_function("route => document.body.dataset.route === route && document.body.dataset.ready === 'true'", arg=name)
        self.record('route', route=name, theme=theme, care=care, brand=brand, state=state)

    def capture(self, name):
        self.page.screenshot(path=str(self.out / f'{name}.png'), full_page=True)

    def audit(self, routes):
        observations, failures = [], []
        case = self.case
        for width in VIEWPORTS:
            self.page.set_viewport_size({'width': width, 'height': 900})
            for route in routes:
                self.case = f'{case}:{route}:{width}'
                try:
                    self.route(route)
                    metrics = self.page.evaluate("""() => { const page = document.querySelector('#container .page'); return {
                        width: innerWidth, scrollWidth: document.documentElement.scrollWidth,
                        pageWidth: page.clientWidth, pageScrollWidth: page.scrollWidth,
                        text: page.innerText.trim(),
                        graphicalSteps: [...page.querySelectorAll('.weui-steps__item')]
                          .filter(item => item.getBoundingClientRect().width > 0 && item.getBoundingClientRect().height > 0).length,
                        brokenImages: [...document.querySelectorAll('img')]
                          .filter(image => image.getAttribute('src') && (!image.complete || !image.naturalWidth))
                          .map(image => image.getAttribute('src'))
                    }; }""")
                    assert metrics['text'] or (route == 'steps_horizonal' and metrics['graphicalSteps'] >= 4), ('empty route', route, width)
                    assert not metrics['brokenImages'], (route, width, metrics['brokenImages'])
                    assert metrics['scrollWidth'] <= width + 1, (route, metrics['scrollWidth'], width)
                    assert metrics['pageScrollWidth'] <= metrics['pageWidth'] + 1, (route, metrics)
                    observations.append({'route': route, 'width': width, 'result': 'PASS',
                                         'scrollWidth': metrics['scrollWidth'], 'pageScrollWidth': metrics['pageScrollWidth']})
                except Exception as error:
                    item = {'route': route, 'width': width, 'result': 'FAIL', 'error': repr(error)}
                    observations.append(item)
                    failures.append(item)
                    self.capture(f'failed-route-{route}-{width}')
                (self.out / 'route-loads.json').write_text(json.dumps(observations, indent=2) + '\n')
        self.case = case
        self.capture('route-audit-final')
        assert not failures, failures
        return {'routes': len(routes), 'widths': list(VIEWPORTS), 'observations': observations}

    def dialog(self, route, opener, dialog):
        self.route(route)
        trigger = self.page.locator(opener)
        trigger.click()
        active = self.page.locator(dialog)
        content = active.locator('.weui-dialog,.weui-half-screen-dialog').first
        expect(content if content.count() else active).to_be_visible()
        expect(active).to_have_attribute('aria-hidden', 'false')
        for key in ('Tab', 'Shift+Tab') * 5:
            self.page.keyboard.press(key)
            assert active.evaluate('(element) => element.contains(document.activeElement)'), key
        self.capture(route + '-open')
        self.page.keyboard.press('Escape')
        expect(active).to_have_attribute('aria-hidden', 'true')
        self.page.wait_for_function("selector => { const node = document.querySelector(selector); const rect = node.getBoundingClientRect(); const style = getComputedStyle(node); return style.display === 'none' || style.visibility === 'hidden' || rect.top >= innerHeight || rect.bottom <= 0 || Number(style.opacity) === 0; }", arg=dialog)
        expect(trigger).to_be_focused()
        trigger.click()
        action = active.locator('.weui-dialog__btn,.js_close,#iosActionsheetCancel').first
        if action.count():
            action.click()
            expect(active).to_have_attribute('aria-hidden', 'true')
            expect(trigger).to_be_focused()
        else:
            self.page.keyboard.press('Escape')
        return {'route': route, 'tab_trap': True, 'escape_restore': True, 'pointer_close': True}

    def count(self):
        return int(self.page.locator('#container .page').get_attribute('data-action-count') or 0)

    def feedback(self, text):
        expect(self.page.get_by_role('status').filter(has_text=text).last).to_be_visible()

    def buttons(self):
        self.route('button_default')
        active = self.page.locator('.weui-btn_primary:not(.weui-btn_disabled,.weui-btn_loading)').first
        before = self.count()
        active.click()
        assert self.count() == before + 1
        self.feedback(active.inner_text().strip())
        active.focus()
        self.page.keyboard.press('Space')
        assert self.count() == before + 2
        for selector in ('.weui-btn_disabled', '.weui-btn_loading'):
            disabled = self.page.locator(selector).first
            expect(disabled).to_have_attribute('aria-disabled', 'true')
            at = disabled.bounding_box()
            assert at
            self.page.mouse.click(at['x'] + at['width'] / 2, at['y'] + at['height'] / 2)
            assert self.count() == before + 2, selector
        self.capture('button-guards')
        return {'pointer': 1, 'keyboard': 1, 'disabled_loading_callbacks': 0}

    def inputs(self):
        self.route('form_input_status')
        card = self.page.locator('#js_input1')
        submit = self.page.locator('#showTooltips')
        expect(submit).to_have_attribute('aria-disabled', 'true')
        card.fill('1234')
        submit.click()
        expect(card).to_have_attribute('aria-invalid', 'true')
        self.feedback('请输入16位数卡号')
        card.fill('1234567890123456')
        expect(card).not_to_have_attribute('aria-invalid', 'true')
        card.press('End')
        card.press_sequentially('XYZ')
        expect(card).to_have_value('1234567890123456')
        self.page.locator('#js_input_clear').click()
        expect(card).to_have_value('')
        expect(card).to_be_focused()
        readonly = self.page.locator('#js_input3')
        readonly.click()
        readonly.press_sequentially('replacement')
        expect(readonly).to_have_value('1234567')
        disabled = self.page.locator('#js_input4')
        expect(disabled).to_be_disabled()
        disabled.focus()
        expect(disabled).not_to_be_focused()
        expect(disabled).to_have_value('WeUI')
        self.capture('html-input-guards')

        self.route('form_textarea')
        area = self.page.locator('textarea').first
        area.fill('ABCD')
        area.press('Home')
        area.press('ArrowRight')
        session = self.context.new_cdp_session(self.page)
        session.send('Input.imeSetComposition', {'text': 'nihao', 'selectionStart': 5, 'selectionEnd': 5})
        expect(area).to_have_value('AnihaoBCD')
        session.send('Input.insertText', {'text': '你好'})
        expect(area).to_have_value('A你好BCD')
        session.detach()
        area.fill('A😀e\u0301\nsecond line')
        expect(area).to_have_value('A😀e\u0301\nsecond line')
        counter = self.page.locator('.weui-textarea-counter span')
        if counter.count():
            expect(counter).to_have_text('17')
        self.capture('html-unicode-multiline')
        self.route('input')
        text = self.page.locator('input[placeholder="请输入文本"]').first
        text.fill('Input demo')
        expect(text).to_have_value('Input demo')
        return {'maxlength': 16, 'readonly_disabled': True,
                'browser_composition_protocol': 'CDP Input.imeSetComposition/insertText; not an OS IME',
                'unicode_multiline': True}

    def choices(self):
        results = {}
        for route, kind in [('form_checkbox', 'checkbox'), ('form_radio', 'radio'), ('form_switch', 'checkbox')]:
            self.route(route)
            inputs = self.page.locator(f'input[type={kind}]:not(:disabled)')
            assert inputs.count(), route
            target = self.page.locator('input[type=radio]:not(:disabled):not(:checked)').first if kind == 'radio' else inputs.first
            identity = target.get_attribute('id')
            if identity:
                target = self.page.locator('#' + identity)
            before = target.is_checked()
            label = target.locator('xpath=ancestor::label[1]')
            (label if label.count() else target).click()
            self.record('choice-pointer', route=route, before=before, after=target.is_checked(), id=target.get_attribute('id'))
            assert target.is_checked() != before, (route, 'pointer did not change', before)
            if kind == 'radio':
                group = target.get_attribute('name')
                assert self.page.locator(f'input[type=radio][name="{group}"]:checked').count() == 1
            else:
                target.focus()
                self.page.keyboard.press('Space')
                self.record('choice-keyboard', route=route, after=target.is_checked())
                assert target.is_checked() == before, (route, 'Space did not restore', before)
            results[route] = {'pointer': True, 'keyboard': kind != 'radio'}
        return results

    def agreement_vcode(self):
        self.route('form_vcode')
        label = self.page.locator('.weui-agree:visible').first
        agreement = label.locator('input[type=checkbox]')
        before = agreement.is_checked()
        label.locator('a').click()
        assert agreement.is_checked() == before
        self.feedback('相关条款')
        submit = self.page.locator('#showTooltips:visible').last
        submit.click()
        expect(agreement).to_have_attribute('aria-invalid', 'true')
        agreement.focus()
        self.page.keyboard.press('Space')
        assert agreement.is_checked() != before
        others = self.page.locator('.weui-agree:visible input[type=checkbox]').all()
        for consent in others:
            if not consent.is_checked():
                submit.click()
                expect(consent).to_have_attribute('aria-invalid', 'true')
                consent.focus()
                self.page.keyboard.press('Space')
                expect(consent).to_be_checked()
        submit.click()
        expect(self.page.locator('#js_toast .weui-toast')).to_be_visible()
        button = self.page.locator('#js_btn_vcode')
        before = self.count()
        button.click()
        expect(button).to_have_attribute('aria-disabled', 'true')
        expect(button).to_contain_text('重新发送')
        self.feedback('验证码已发送')
        assert self.count() == before + 1
        at = button.bounding_box()
        assert at
        self.page.mouse.click(at['x'] + at['width'] / 2, at['y'] + at['height'] / 2)
        assert self.count() == before + 1
        self.capture('agreement-vcode')
        return {'terms_did_not_toggle': True, 'consent_keyboard': True, 'guarded_send_count': 1}

    def tabs(self):
        result = {}
        for route in ('navbar', 'tabbar'):
            self.route(route)
            tabs = self.page.get_by_role('tab')
            assert tabs.count() > 1
            tabs.nth(1).click()
            expect(tabs.nth(1)).to_have_attribute('aria-selected', 'true')
            tabs.nth(1).focus()
            self.page.keyboard.press('End')
            expect(tabs.last).to_be_focused()
            expect(tabs.last).to_have_attribute('aria-selected', 'true')
            selected_panel = tabs.last.get_attribute('aria-controls')
            expect(self.page.locator('#' + selected_panel)).to_be_visible()
            self.page.keyboard.press('Home')
            expect(tabs.first).to_be_focused()
            expect(tabs.first).to_have_attribute('aria-selected', 'true')
            assert self.page.get_by_role('tabpanel').count() == 1
            result[route] = {'tabs': tabs.count(), 'pointer_and_keys': True}
        self.capture('tab-selection')
        return result

    def search(self):
        self.route('searchbar_filled')
        self.page.locator('#searchText').click()
        field = self.page.locator('#searchInput')
        expect(field).to_be_focused()
        field.fill('WeUI')
        expect(self.page.locator('#searchResult')).to_be_visible()
        field.press('Enter')
        self.feedback('搜索：WeUI')
        self.page.locator('#searchClear').click()
        expect(field).to_have_value('')
        expect(field).to_be_focused()
        field.fill('again')
        self.page.locator('#searchCancel').click()
        expect(field).to_have_value('')
        expect(self.page.locator('#searchText')).to_be_focused()
        self.capture('search-canceled')
        return {'activate_type_submit_clear_cancel': True}

    def picker(self):
        self.route('picker')
        opener = self.page.locator('#showPicker')
        opener.click()
        dialog = self.page.get_by_role('dialog')
        wheel = dialog.get_by_role('listbox').first
        expect(wheel).to_be_focused()
        wheel.press('End')
        expect(wheel.get_by_role('option', selected=True)).to_have_text('其他')
        wheel.press('ArrowUp')
        expect(wheel.get_by_role('option', selected=True)).to_have_text('的士票')
        expect(wheel.get_by_role('option', selected=True)).to_have_attribute('aria-disabled', 'false')
        dialog.get_by_role('button', name='取消', exact=True).click()
        expect(opener).to_be_focused()
        opener.click()
        dialog = self.page.get_by_role('dialog')
        wheel = dialog.get_by_role('listbox').first
        expect(wheel.get_by_role('option', selected=True)).to_have_text('飞机票')
        wheel.press('End')
        dialog.get_by_role('button', name='确定', exact=True).click()
        self.feedback('其他')
        expect(opener).to_be_focused()
        self.page.locator('#showCascadePicker').click()
        dialog = self.page.get_by_role('dialog')
        wheels = dialog.get_by_role('listbox')
        assert wheels.count() == 3
        wheels.first.press('End')
        expect(wheels.nth(1).get_by_role('option').first).to_have_text('南宁')
        wheels.nth(1).press('End')
        expect(wheels.nth(2).get_by_role('option').first).to_have_text('象山')
        dialog.get_by_role('button', name='确定', exact=True).click()
        self.feedback('广西 桂林 象山')
        self.capture('picker-confirmed')
        return {'staging_cancel': True, 'disabled_skip': True, 'confirm': True, 'cascade': True}

    def slider_progress(self):
        self.route('slider')
        slider = self.page.get_by_role('slider', name='可调滑块', exact=True)
        slider.focus()
        self.page.keyboard.press('End')
        expect(slider).to_have_attribute('aria-valuenow', '100')
        self.page.keyboard.press('Home')
        self.page.keyboard.press('ArrowRight')
        expect(slider).to_have_attribute('aria-valuenow', '1')
        track = self.page.locator('#sliderInner').bounding_box()
        assert track
        self.page.mouse.click(track['x'] + track['width'] * .75, track['y'] + track['height'] / 2)
        assert 73 <= int(slider.get_attribute('aria-valuenow')) <= 77
        expect(self.page.get_by_role('slider', name='不可调滑块', exact=True)).to_have_attribute('aria-valuenow', '0')
        self.capture('slider-pointer')
        self.route('progress')
        self.page.locator('#btnUpload').click()
        progress = self.page.get_by_role('progressbar').first
        expect(progress).to_have_attribute('aria-valuenow', re.compile(r'[1-9]\d*'))
        self.page.locator('.weui-progress__opr').first.click()
        self.feedback('已取消上传演示')
        stopped = progress.get_attribute('aria-valuenow')
        self.page.wait_for_timeout(500)
        expect(progress).to_have_attribute('aria-valuenow', stopped)
        expect(self.page.locator('#btnUpload')).to_have_attribute('aria-disabled', 'false')
        return {'slider_keys_pointer': True, 'unavailable_slider': True, 'progress_start_cancel': True}

    def media(self):
        self.route('uploader')
        self.page.locator('#uploaderInputWithDelete').set_input_files(
            {'name': 'local-pixel.png', 'mimeType': 'image/png', 'buffer': PNG})
        preview = self.page.get_by_role('button', name='预览 local-pixel.png', exact=True)
        expect(preview).to_be_visible()
        assert 'blob:' in preview.evaluate('(node) => node.style.backgroundImage')
        preview.focus()
        self.page.keyboard.press('Enter')
        dialog = self.page.get_by_role('dialog', name='图片预览', exact=True)
        expect(dialog).to_be_visible()
        self.capture('local-image-preview')
        self.page.keyboard.press('Escape')
        expect(preview).to_be_focused()
        self.page.get_by_role('button', name='删除 local-pixel.png', exact=True).click()
        expect(preview).to_have_count(0)
        self.feedback('已删除图片')
        self.route('gallery')
        dialog = self.page.get_by_role('dialog', name='图片预览', exact=True)
        expect(dialog).to_be_visible()
        self.page.get_by_role('button', name='关闭图片预览', exact=True).click()
        expect(self.page.locator('#showGallery')).to_be_focused()
        self.page.locator('#showGallery').click()
        self.page.keyboard.press('Escape')
        expect(self.page.locator('#showGallery')).to_be_focused()
        return {'local_blob_preview': True, 'keyboard_open_escape': True, 'delete': True, 'external_uploads': 0}

    def swipe(self):
        self.route('list')
        row = self.page.get_by_role('button', name='滑动列表项，向左显示操作', exact=True)
        row.focus()
        row.press('ArrowRight')
        expect(row).to_have_attribute('aria-expanded', 'false')
        row.press('ArrowLeft')
        expect(row).to_have_attribute('aria-expanded', 'true')
        self.page.locator('.weui-swiped-btn').first.click()
        expect(row).to_have_count(0)
        self.feedback('已删除列表项')
        self.route('list')
        row = self.page.get_by_role('button', name='滑动列表项，向左显示操作', exact=True)
        at = row.bounding_box()
        assert at
        x, y = at['x'] + at['width'] / 2, at['y'] + at['height'] / 2
        self.page.mouse.move(x, y)
        self.page.mouse.down()
        self.page.mouse.move(x + 90, y, steps=8)
        self.page.mouse.up()
        expect(row).to_have_attribute('aria-expanded', 'false')
        self.page.mouse.move(x, y)
        self.page.mouse.down()
        self.page.mouse.move(x - 90, y, steps=8)
        self.page.mouse.up()
        expect(row).to_have_attribute('aria-expanded', 'true')
        self.capture('swipe-pointer-open')
        return {'keyboard_disclose_delete': True, 'real_pointer_drag_both_directions': True}

    def notices(self):
        self.route('information-bar')
        notice = self.page.locator('.weui-information-bar').first
        before = self.count()
        notice.get_by_role('button', name='关闭', exact=True).click()
        expect(notice).to_be_hidden()
        assert self.count() == before + 1
        self.page.get_by_role('button', name='恢复信息栏', exact=True).click()
        expect(notice).to_be_visible()
        self.capture('information-bar-restored')
        return {'actual_notice_dismiss': True, 'restore': True}

    def toast(self):
        self.route('toast')
        trigger = self.page.locator('#showToast')
        trigger.click()
        expect(self.page.locator('#toast .weui-toast')).to_be_visible()
        expect(trigger).to_be_focused()
        expect(self.page.locator('#toast')).to_have_attribute('aria-hidden', 'true', timeout=4000)
        expect(trigger).to_be_focused()
        return {'passive_focus': True, 'visible_then_expires': True}

    def links_and_content(self):
        result = {}
        for route, selector in [('footer', '.weui-footer__link'), ('grid', '.weui-grid'),
                                ('preview', '.weui-form-preview__btn'), ('panel', 'a.weui-media-box'),
                                ('msg_success', '.weui-btn')]:
            self.route(route)
            target = self.page.locator(selector).first
            before = self.count()
            target.click()
            assert self.count() == before + 1, route
            result[route] = 'actual action feedback'
        self.route('article')
        self.page.set_viewport_size({'width': 375, 'height': 700})
        self.page.mouse.move(180, 350)
        self.page.mouse.wheel(0, 1200)
        self.page.wait_for_function("() => scrollY > 0 || document.querySelector('#container .page').scrollTop > 0")
        self.capture('article-narrow-scrolled')
        self.route('home')
        category = self.page.locator('.js_category').first
        category.focus()
        self.page.keyboard.press('Space')
        expect(category).to_have_attribute('aria-expanded', 'true')
        self.page.locator('.js_categoryInner:visible .js_item').first.click()
        self.page.wait_for_function("() => document.body.dataset.route !== 'home' && document.body.dataset.ready === 'true'")
        self.route('layers')
        layer = self.page.locator('.layers__layer').first
        layer.focus()
        self.page.keyboard.press('Space')
        expect(self.page.locator('.j_info:visible')).to_have_count(1)
        result['article'] = 'real scroll/content/images'
        result['home'] = 'keyboard expansion and local navigation'
        result['layers'] = 'keyboard selection'
        result['presentational_families'] = ['badge', 'icons', 'flex', 'loading', 'loadmore', 'steps']
        return result

    def themes(self):
        observations = []
        for width in VIEWPORTS:
            self.page.set_viewport_size({'width': width, 'height': 900})
            for theme, care, brand in [('light', False, ''), ('light', True, ''), ('dark', False, ''),
                                       ('dark', True, '805ad5')]:
                self.route('button_default', theme=theme, care=care, brand=brand)
                actual = self.page.evaluate("""() => {
                    const button = document.querySelector('.weui-btn_primary:not(.weui-btn_disabled,.weui-btn_loading)');
                    const style = getComputedStyle(button);
                    return {config: window.WEGPUI_PLAYGROUND_CONFIG, background: getComputedStyle(document.body).backgroundColor,
                            foreground: getComputedStyle(document.body).color,
                            primary: style.backgroundColor, font: parseFloat(style.fontSize),
                            overflow: document.documentElement.scrollWidth > innerWidth + 1};
                }""")
                assert not actual['overflow'], (width, actual)
                assert actual['config']['theme'] == theme
                assert actual['config']['care'] == care
                if brand == '805ad5':
                    assert actual['primary'] == 'rgb(128, 90, 213)', actual
                elif theme == 'light':
                    assert actual['primary'] == ('rgb(1, 137, 66)' if care else 'rgb(7, 193, 96)'), actual
                    if care:
                        assert actual['foreground'] == 'rgb(0, 0, 0)', actual
                observations.append({'width': width, 'theme': theme, 'care': care, **actual})
                self.capture(f'theme-{width}-{theme}-care{int(care)}')
            light, care, dark, _custom = observations[-4:]
            assert light['background'] != dark['background']
            assert care['primary'] != light['primary'], (light, care)
        return observations

    def appearance_preserves_draft(self):
        self.route('form_primary', brand='')
        field = self.page.locator('input:not([type=checkbox]):not([type=radio]):not(:disabled)').first
        field.fill('Draft 你好 e\u0301😀')
        field.press('Home')
        field.press('Shift+ArrowRight')
        field.evaluate('(node) => { window.__draftField = node; }')
        selection = field.evaluate('(node) => [node.selectionStart, node.selectionEnd]')
        for patch in ({'theme': 'dark'}, {'care': True}, {'brand': '805ad5'}):
            self.page.evaluate("config => postMessage({type: 'wegpui:configure', config}, location.origin)", patch)
            for name, value in patch.items():
                self.page.wait_for_function("([name, value]) => { const actual = WEGPUI_PLAYGROUND_CONFIG[name]; return String(actual).replace(/^#/, '') === String(value); }", arg=[name, value])
            expect(field).to_have_value('Draft 你好 e\u0301😀')
            expect(field).to_be_focused()
            assert field.evaluate('(node) => node === window.__draftField')
            assert field.evaluate('(node) => [node.selectionStart, node.selectionEnd]') == selection
            assert self.page.evaluate('WEGPUI_PLAYGROUND_CONFIG.route') == 'form_primary'
        with self.page.expect_popup() as opened:
            self.page.evaluate("() => window.open('about:blank')")
        sender = opened.value
        try:
            origin = urlparse(self.base)
            sender.evaluate("origin => opener.postMessage({type: 'wegpui:configure', config: {route: 'dialog'}}, origin)",
                            f'{origin.scheme}://{origin.netloc}')
            self.page.wait_for_timeout(150)
            assert self.page.evaluate('WEGPUI_PLAYGROUND_CONFIG.route') == 'form_primary'
            expect(field).to_have_value('Draft 你好 e\u0301😀')
        finally:
            sender.close()
        self.capture('appearance-preserves-draft')
        return {'partial_configurations': 3, 'same_input_dom_value_selection_focus': True,
                'same_origin_wrong_window_message_ignored': True}

    def shell(self):
        results = []
        for width in VIEWPORTS:
            self.page.set_viewport_size({'width': width, 'height': 900})
            self.page.goto(self.base + '#components/dialog?theme=dark&care=1&brand=805ad5', wait_until='networkidle')
            iframe = self.page.locator('iframe[src*="playground/"]').first
            expect(iframe).to_be_visible()
            assert iframe.get_attribute('title'), 'live iframe needs an accessible title'
            frame = iframe.element_handle().content_frame()
            assert frame
            frame.wait_for_function("() => document.body?.dataset.route === 'dialog' && document.body.dataset.ready === 'true'")
            config = frame.evaluate('WEGPUI_PLAYGROUND_CONFIG')
            assert config['route'] == 'dialog'
            assert config['theme'] == 'dark'
            assert config['care'] is True
            assert str(config['brand']).lstrip('#').lower() == '805ad5'
            assert not self.page.evaluate('document.documentElement.scrollWidth > innerWidth + 1'), width
            frame.locator('#showIOSDialog1').click()
            expect(frame.locator('#iosDialog1')).to_have_attribute('aria-hidden', 'false')
            self.page.keyboard.press('Escape')
            expect(frame.locator('#showIOSDialog1')).to_be_focused()
            self.capture(f'shell-{width}-live-dialog')
            self.page.reload(wait_until='networkidle')
            frame = self.page.locator('iframe[src*="playground/"]').first.element_handle().content_frame()
            assert frame
            frame.wait_for_function("() => document.body?.dataset.route === 'dialog' && document.body.dataset.ready === 'true'")
            results.append({'width': width, 'deep_link_config_reload': True, 'live_dialog': True})
        self.page.set_viewport_size({'width': 1280, 'height': 900})
        self.page.goto(self.base + '#components/form_primary?theme=light&care=0&brand=', wait_until='networkidle')
        iframe = self.page.locator('#live-demo')
        expect(iframe).to_be_visible()
        frame = iframe.element_handle().content_frame()
        assert frame
        frame.wait_for_function("() => document.body?.dataset.route === 'form_primary' && document.body.dataset.ready === 'true'")
        field = frame.locator('input:not([type=checkbox]):not([type=radio]):not(:disabled)').first
        field.fill('Shell draft 你好')
        field.evaluate('(node) => { window.__shellDraft = node; }')
        self.page.locator('[data-theme="dark"]').click()
        self.page.locator('#care-toggle').check()
        colour = self.page.locator('#brand-color')
        colour.evaluate("node => { node.value = '#805ad5'; node.dispatchEvent(new Event('input', {bubbles:true})); }")
        frame.wait_for_function("() => WEGPUI_PLAYGROUND_CONFIG.theme === 'dark' && WEGPUI_PLAYGROUND_CONFIG.care && WEGPUI_PLAYGROUND_CONFIG.brand.replace(/^#/, '') === '805ad5'")
        expect(field).to_have_value('Shell draft 你好')
        assert field.evaluate('(node) => node === window.__shellDraft')
        self.page.locator('#viewport-select').select_option('768')
        assert iframe.bounding_box()['width'] >= 760
        assert not self.page.evaluate('document.documentElement.scrollWidth > innerWidth + 1')
        full_page = self.page.locator('#open-demo').get_attribute('href')
        assert full_page.startswith(urljoin(self.base, 'playground/index.html'))
        self.page.locator('.preview-modes').get_by_role('link', name='Native Rust captures', exact=True).click()
        expect(self.page.locator('#native-capture')).to_be_visible()
        expect(self.page.locator('#native-capture')).to_have_js_property('complete', True)
        assert self.page.locator('#native-capture').evaluate('(image) => image.complete && image.naturalWidth > 0')
        self.page.locator('.preview-modes').get_by_role('link', name='Live browser demo', exact=True).click()
        iframe = self.page.locator('#live-demo')
        frame = iframe.element_handle().content_frame()
        assert frame
        frame.wait_for_function("() => document.body?.dataset.ready === 'true'")
        config = frame.evaluate('WEGPUI_PLAYGROUND_CONFIG')
        assert config['theme'] == 'dark' and config['care'] and config['brand'].lstrip('#') == '805ad5'
        search = self.page.get_by_role('searchbox', name='Search components')
        search.fill('dialog')
        self.page.locator('#catalog-list').get_by_role('link', name='Dialogs', exact=True).focus()
        self.page.keyboard.press('Enter')
        frame = self.page.locator('#live-demo').element_handle().content_frame()
        assert frame
        frame.wait_for_function("() => document.body?.dataset.route === 'dialog' && document.body.dataset.ready === 'true'")
        config = frame.evaluate('WEGPUI_PLAYGROUND_CONFIG')
        assert config['theme'] == 'dark' and config['care'] and config['brand'].lstrip('#') == '805ad5'
        self.page.locator('#state-select').select_option('ios1')
        expect(frame.locator('#iosDialog1')).to_have_attribute('aria-hidden', 'false')
        frame.locator('.weui-dialog__btn').first.click()
        search = self.page.get_by_role('searchbox', name='Search components')
        search.fill('picker')
        self.page.locator('#catalog-list').get_by_role('link', name='Pickers', exact=True).focus()
        self.page.keyboard.press('Enter')
        frame = self.page.locator('#live-demo').element_handle().content_frame()
        assert frame
        frame.wait_for_function("() => document.body?.dataset.route === 'picker' && document.body.dataset.ready === 'true'")
        self.page.go_back(wait_until='networkidle')
        frame = self.page.locator('#live-demo').element_handle().content_frame()
        assert frame
        frame.wait_for_function("() => document.body?.dataset.route === 'dialog' && document.body.dataset.ready === 'true'")
        self.capture('shell-prefs-keyboard-native-history')
        results.append({'toolbar_draft_dom_preserved': True, 'colour_control': 'native input event',
                        'native_capture_mode': True, 'keyboard_catalog': True, 'prefs_across_routes': True,
                        'example_state': 'ios1', 'history': True, 'wide_inner_preview': 768})
        return results


def parse():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--site', type=Path, default=ROOT / 'website/dist')
    parser.add_argument('--browser', default='/usr/bin/chromium')
    parser.add_argument('--only', default='', help='Comma-separated development subset; recorded as incomplete')
    parser.add_argument('--hosting', choices=('both', 'root', 'project'), default='both')
    parser.add_argument('--out', type=Path, default=ROOT / 'target/web-interactive' /
                        datetime.now(timezone.utc).strftime('%Y%m%dT%H%M%S%fZ'))
    return parser.parse_args()


def main():
    args = parse()
    args.site = args.site.resolve()
    if args.out.exists() and any(args.out.iterdir()):
        raise SystemExit('Choose a new --out directory; previous evidence is preserved.')
    args.out.mkdir(parents=True, exist_ok=True)
    (args.out / 'validator.py').write_bytes(Path(__file__).read_bytes())
    files = {str(path.relative_to(args.site)): digest(path)
             for path in args.site.rglob('*') if path.is_file()}
    (args.out / 'built-site-sha256.json').write_text(json.dumps(files, indent=2) + '\n')
    routes = sorted(re.findall(r'\("([\w-]+)",\s*\w+::build\)',
                               (ROOT / 'examples/gallery/pages/mod.rs').read_text()))
    assert len(routes) == 60, routes
    provenance = {'native_gpui_test': False,
                  'source_commit': subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip(),
                  'source_status': subprocess.check_output(['git', 'status', '--short'], cwd=ROOT, text=True),
                  'validator_sha256': digest(Path(__file__)), 'routes': routes}
    (args.out / 'provenance.json').write_text(json.dumps(provenance, indent=2) + '\n')
    all_results = []
    with sync_playwright() as playwright:
        browser = playwright.chromium.launch(headless=True, executable_path=args.browser)
        for project in (False, True):
            if args.hosting != 'both' and project != (args.hosting == 'project'):
                continue
            out = args.out / ('project-prefix' if project else 'root')
            out.mkdir(exist_ok=True)
            with server(args.site, project, out / 'http.log') as base:
                context = browser.new_context(viewport={'width': 1280, 'height': 900},
                                              service_workers='block')
                context.tracing.start(screenshots=True, snapshots=True, sources=True)
                page = context.new_page()
                page.set_default_timeout(6000)
                page.set_default_navigation_timeout(20000)
                report = {'browser': browser.version, 'base': base, 'native_gpui_test': False,
                          'complete_suite': not bool(args.only), 'checks': []}
                checks = Checks(context, page, base, out, report)
                cases = [
                    ('all-routes-responsive', lambda: checks.audit(routes)),
                    ('dialog-keyboard', lambda: checks.dialog('dialog', '#showIOSDialog1', '#iosDialog1')),
                    ('half-dialog-keyboard', lambda: checks.dialog('half-screen-dialog', '#showDialog1', '#dialogWrap1')),
                    ('action-sheet-keyboard', lambda: checks.dialog('actionsheet', '#showIOSActionSheet', '#iosActionsheet')),
                    ('button-guards', checks.buttons), ('html-input-editing', checks.inputs),
                    ('choices', checks.choices), ('agreement-vcode', checks.agreement_vcode),
                    ('tabs', checks.tabs), ('search', checks.search), ('pickers', checks.picker),
                    ('slider-progress', checks.slider_progress), ('local-media', checks.media),
                    ('swipe', checks.swipe), ('notices', checks.notices), ('toast', checks.toast),
                    ('links-content', checks.links_and_content), ('themes', checks.themes),
                    ('appearance-preserves-draft', checks.appearance_preserves_draft),
                    ('shell-integration', checks.shell),
                ]
                selected = set(args.only.split(',')) if args.only else {name for name, _ in cases}
                assert selected <= {name for name, _ in cases}, selected
                for name, callback in cases:
                    if name in selected:
                        checks.run(name, callback)
                report['result'] = 'PASS' if all(item['result'] == 'PASS' for item in report['checks']) and not checks.problems else 'FAIL'
                checks.save()
                context.tracing.stop(path=str(out / 'trace.zip'))
                context.close()
                all_results.append(report)
        browser.close()
    after = {str(path.relative_to(args.site)): digest(path)
             for path in args.site.rglob('*') if path.is_file()}
    summary = {'result': 'PASS' if all(report['result'] == 'PASS' for report in all_results) and after == files else 'FAIL',
               'native_gpui_test': False, 'complete_suite': not bool(args.only) and args.hosting == 'both',
               'build_changed_during_run': after != files,
               'reports': [str(Path('project-prefix' if report['base'].endswith('/weGPUI/') else 'root') / 'browser-checks.json')
                           for report in all_results]}
    (args.out / 'summary.json').write_text(json.dumps(summary, indent=2) + '\n')
    print(json.dumps(summary, indent=2))
    return 0 if summary['result'] == 'PASS' else 1


if __name__ == '__main__':
    raise SystemExit(main())
