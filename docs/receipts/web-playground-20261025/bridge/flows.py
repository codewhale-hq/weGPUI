import base64,json,re
from pathlib import Path
from playwright.sync_api import sync_playwright
root=Path('/workspace/weGPUI'); upstream=Path('/workspace/.wegpui-cloud/target-198/weui-upstream/weui/package')
routefiles={p.stem:p for p in (upstream/'src/example').rglob('*.html') if p.stem!='index'}
results=[]
with sync_playwright() as pw:
 browser=pw.chromium.launch(executable_path='/usr/bin/chromium',headless=True,args=['--no-sandbox'])
 page=browser.new_page(viewport={'width':375,'height':812});page.set_default_timeout(6000)
 def load(route):
  markup=re.sub(r'<script\b[^>]*>[\s\S]*?</script>','',routefiles[route].read_text(),flags=re.I)
  markup=re.sub(r'href=([\"\'])javascript:[^\"\']*\1','href="#"',markup,flags=re.I)
  page.set_content('<div id="container">'+markup+'</div><p role="status" id="status"></p>')
  for path in [upstream/'dist/style/weui.css',upstream/'dist/example/example.css',root/'website/playground/bridge.css']:page.add_style_tag(path=str(path))
  page.add_script_tag(path=str(root/'website/playground/bridge.js'))
  page.evaluate('''route=>{const p=document.querySelector('#container .page');p.classList.add(route,'js_show');window.WeGPUIPlayground.mount({route,page:p,announce:m=>document.querySelector('#status').textContent=m,navigate:r=>document.body.dataset.navigate=r});}''',route)
 def record(name,fn):
  try:fn();results.append({'case':name,'pass':True})
  except Exception as exc:results.append({'case':name,'pass':False,'error':str(exc)})
 def dialog():
  load('dialog');page.locator('#showIOSDialog1').click()
  for i in range(9):
   page.keyboard.press('Shift+Tab' if i%2 else 'Tab');assert page.evaluate("document.querySelector('#iosDialog1').contains(document.activeElement)")
  page.locator('#iosDialog1 .weui-dialog__btn_primary').click();assert page.evaluate("document.activeElement.id==='showIOSDialog1'")
 record('dialog-trap-escape-restore',dialog)
 def picker():
  load('picker');page.locator('#showPicker').click();group=page.get_by_role('listbox');group.press('Home');group.press('ArrowDown');group.press('ArrowDown');group.press('ArrowDown')
  assert group.get_attribute('aria-activedescendant').endswith('-4')
  page.get_by_role('button',name='确定',exact=True).click();assert '其他' in page.locator('#status').inner_text()
  assert page.evaluate("document.activeElement.id==='showPicker'")
  page.locator('#showDatePicker').click();assert page.get_by_role('listbox').count()==3
  page.get_by_role('listbox',name='年',exact=True).press('Home');page.get_by_role('listbox',name='月',exact=True).press('End');page.keyboard.press('Escape')
  assert page.evaluate("document.activeElement.id==='showDatePicker'")
 record('picker-keyboard-disabled-date-cancel',picker)
 def buttons():
  load('button_default');page.locator('.weui-btn_disabled').first.click(force=True);assert not page.locator('#status').inner_text()
  page.locator('.weui-btn_loading').first.click(force=True);assert not page.locator('#status').inner_text()
  a=page.locator('.weui-btn_primary:not(.weui-btn_disabled):not(.weui-btn_loading)').first;a.focus();page.keyboard.press('Space');assert '已触发' in page.locator('#status').inner_text()
 record('buttons-disabled-loading-space',buttons)
 def upload():
  load('uploader');data=base64.b64decode('iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mP8/x8AAwMCAO+jCKsAAAAASUVORK5CYII=')
  page.locator('#uploaderInput').set_input_files({'name':'local.png','mimeType':'image/png','buffer':data})
  preview=page.get_by_role('button',name='预览 local.png',exact=True);assert 'blob:' in preview.get_attribute('style');preview.focus();page.keyboard.press('Enter')
  page.locator('#galleryDel').click();assert page.get_by_role('button',name='预览 local.png',exact=True).count()==0
  assert page.evaluate("document.activeElement.id==='uploaderInput'")
  page.locator('#uploaderInputWithDelete').set_input_files({'name':'cover.png','mimeType':'image/png','buffer':data})
  page.get_by_role('button',name='删除 cover.png',exact=True).click();assert page.get_by_role('button',name='预览 cover.png',exact=True).count()==0
 record('local-file-keyboard-preview-delete-focus',upload)
 def swipe():
  load('list');body=page.locator('.weui-cell_swiped > .weui-cell__bd');body.press('ArrowRight');assert body.get_attribute('aria-expanded')=='false'
  box=body.bounding_box();x=box['x']+box['width']*.75;y=box['y']+box['height']/2
  page.mouse.move(x,y);page.mouse.down();page.mouse.move(x-100,y,steps=8);page.mouse.up();assert body.get_attribute('aria-expanded')=='true'
  page.locator('.weui-swiped-btn').click();assert page.locator('.weui-cell_swiped').count()==0
 record('swipe-real-pointer-reveal-delete',swipe)
 def info():
  load('information-bar');bars=page.locator('.weui-information-bar');assert bars.count()==5
  bars.nth(1).get_by_role('button',name='关闭',exact=True).click();assert not bars.nth(1).is_visible();assert bars.nth(0).is_visible()
  page.locator('#restoreInformationBars').click();assert bars.nth(1).is_visible()
 record('infobar-scoped-close-restore',info)
 browser.close()
Path('/workspace/.wegpui-cloud/bridge-test/flow-results.json').write_text(json.dumps(results,indent=2)+'\n');print(json.dumps(results,ensure_ascii=False))
