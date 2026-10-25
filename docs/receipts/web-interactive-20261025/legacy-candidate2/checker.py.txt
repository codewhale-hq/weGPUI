#!/usr/bin/env python3
"""Exercise historical native-capture views and the surrounding website, not native GPUI."""
import argparse
from datetime import datetime, timezone
import json
from pathlib import Path
from playwright.sync_api import sync_playwright, expect

parser=argparse.ArgumentParser()
parser.add_argument('--url',default='http://127.0.0.1:4173/')
parser.add_argument('--browser',default='/usr/bin/chromium')
parser.add_argument('--out',type=Path,default=Path('target/website-browser') / datetime.now(timezone.utc).strftime('%Y%m%dT%H%M%S%fZ'))
args=parser.parse_args();args.out.mkdir(parents=True,exist_ok=True)
with sync_playwright() as p:
    options={'headless':True}
    if args.browser:
        options['executable_path']=args.browser
    browser=p.chromium.launch(**options)
    context=browser.new_context(viewport={'width':1440,'height':1040},permissions=['clipboard-read','clipboard-write'])
    page=context.new_page()
    errors=[]
    page.on('pageerror',lambda e: errors.append(str(e)))
    page.goto(args.url,wait_until='networkidle')
    expect(page.get_by_role('heading',level=1)).to_contain_text('Familiar design')
    page.screenshot(path=str(args.out/'overview-desktop.png'),full_page=True)
    routes=page.evaluate('WEGPUI.routes')
    api_links=set()
    state_themes=0
    capture_paths=set()
    for route in routes:
        for state in route['states']:
            for theme in ('light','dark'):
                page.goto(args.url+'#components/'+route['id']+'?state='+state+'&theme='+theme+'&view=native')
                image=page.locator('#native-capture')
                expected=route['images'][state][theme]
                expect(image).to_be_visible()
                expect(image).to_have_attribute('src',expected['gpui'])
                expect(image).to_have_js_property('complete',True)
                assert image.evaluate('(img)=>img.naturalWidth')>0,(route['id'],state,theme)
                expect(page.locator('#state-select')).to_have_value(state)
                capture_paths.update(expected.values())
                state_themes+=1
        api_links.update(page.locator('a[href^="api/"]').evaluate_all('(links)=>links.map(a=>a.getAttribute("href"))'))
    for path in sorted(capture_paths):
        decoded=page.evaluate("""src => new Promise(resolve => {
            const image = new Image();
            image.onload = () => resolve(image.naturalWidth > 0 && image.naturalHeight > 0);
            image.onerror = () => resolve(false);
            image.src = src;
        })""",path)
        assert decoded,path
    for path in api_links:
        assert context.request.get(args.url+path).status==200,path
    page.goto(args.url+'#components/button_default?view=native')
    search=page.get_by_role('searchbox',name='Search components')
    search.fill('not-a-real-component')
    expect(page.locator('.empty-results')).to_be_visible()
    search.fill('dialog')
    page.locator('#catalog-list').get_by_role('link',name='Dialogs',exact=True).click()
    page.locator('.preview-modes').get_by_role('link',name='Native Rust captures',exact=True).click()
    page.locator('#state-select').select_option('ios1')
    page.get_by_role('button',name='Dark',exact=True).click()
    expect(page.locator('#native-capture')).to_have_attribute('src','captures/dialog-ios1-dark-gpui.png')
    expect(page.get_by_role('button',name='Dark',exact=True)).to_be_focused()
    page.get_by_role('button',name='Compare WeUI',exact=True).click()
    expect(page.locator('.preview-stage img')).to_have_count(2)
    expect(page.locator('.preview-stage img').nth(1)).to_have_attribute('src','captures/dialog-ios1-dark-weui.png')
    page.get_by_role('button',name='Copy code').click()
    expect(page.get_by_role('button',name='Copy code')).to_have_text('Copied')
    copied=page.evaluate('navigator.clipboard.readText()')
    assert '--state ios1 --theme dark' in copied,copied
    page.reload(wait_until='networkidle')
    expect(page.locator('#native-capture')).to_have_attribute('src','captures/dialog-ios1-dark-gpui.png')
    page.goto(args.url+'#components/button_default?view=native')
    page.screenshot(path=str(args.out/'component-desktop.png'),full_page=True)
    page.get_by_role('link',name='Skip to content').focus()
    page.keyboard.press('Enter')
    expect(page.locator('#main')).to_be_focused()
    assert 'components/button_default' in page.url
    checked=[]
    for width in (390,768,1440):
        page.set_viewport_size({'width':width,'height':900})
        for path in ('overview','components/form_select?view=native','get-started','themes','native','native-guide','native-themes','platforms','roadmap'):
            page.goto(args.url+'#'+path)
            expect(page.get_by_role('heading',level=1)).to_be_visible()
            overflow=page.evaluate('document.documentElement.scrollWidth > innerWidth + 1')
            assert not overflow,(width,path)
            checked.append([width,path])
    page.set_viewport_size({'width':390,'height':844})
    page.goto(args.url+'#components/form_select?view=native')
    page.get_by_role('button',name='Browse all components').click()
    expect(page.get_by_role('button',name='Browse all components')).to_have_attribute('aria-expanded','true')
    page.locator('#catalog-list').get_by_role('link',name='Buttons',exact=True).click()
    expect(page.get_by_role('heading',level=1)).to_have_text('Buttons')
    page.goto(args.url+'#overview');page.screenshot(path=str(args.out/'overview-mobile.png'),full_page=True)
    page.goto(args.url+'#native-themes')
    palettes=page.evaluate('Object.keys(WEGPUI.tokens)')
    for palette in palettes:
        page.get_by_label('Palette',exact=True).select_option(palette)
        expect(page).to_have_url(args.url+'#native-themes?palette='+palette)
        expect(page.locator('.swatches>div')).to_have_count(8)
    page.goto(args.url+'#themes')
    for palette in palettes:
        page.get_by_label('Palette',exact=True).select_option(palette)
        expect(page).to_have_url(args.url+'#themes?palette='+palette)
        expect(page.locator('.swatches>div')).to_have_count(8)
    page.goto(args.url+'#components/does-not-exist')
    expect(page.get_by_role('heading',level=1)).to_contain_text('isn’t in this gallery')
    assert errors==[],errors
    result={'browser':browser.version,'native_application_test':False,'source_commit':page.evaluate('WEGPUI.sourceCommit'),'source_dirty':page.evaluate('WEGPUI.sourceDirty'),'capture_commit':page.evaluate('WEGPUI.captureCommit'),'gpui_version':page.evaluate('WEGPUI.gpuiVersion'),'routes_checked':len(routes),'state_theme_routes_checked':state_themes,'captures_decoded':len(capture_paths),'api_links_checked':len(api_links),'viewport_routes':checked,'palettes':palettes,'page_errors':errors,'passed':['catalog search/empty state','state/theme deep links','native/reference comparison','clipboard','keyboard skip/focus','mobile navigation','responsive overflow','missing route']}
    (args.out/'browser-checks.json').write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps(result,indent=2))
    browser.close()
