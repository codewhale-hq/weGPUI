import json,re
from pathlib import Path
from playwright.sync_api import sync_playwright
root=Path('/workspace/weGPUI'); upstream=Path('/workspace/.wegpui-cloud/target-198/weui-upstream/weui/package')
routefiles={p.stem:p for p in (upstream/'src/example').rglob('*.html') if p.stem!='index'}
states={}
for path in (root/'scripts/parity').glob('*.json'): states.update(json.loads(path.read_text()))
results=[]
with sync_playwright() as pw:
 browser=pw.chromium.launch(executable_path='/usr/bin/chromium',headless=True,args=['--no-sandbox'])
 page=browser.new_page(viewport={'width':375,'height':812})
 errors=[];page.on('pageerror',lambda err: errors.append(str(err)))
 for route,path in sorted(routefiles.items()):
  markup=re.sub(r'<script\b[^>]*>[\s\S]*?</script>','',path.read_text(),flags=re.I)
  markup=re.sub(r'\son\w+\s*=\s*([\"\']).*?\1','',markup,flags=re.I|re.S)
  markup=re.sub(r'href=([\"\'])javascript:[^\"\']*\1','href="#"',markup,flags=re.I)
  for state in ['']+[key.split(':',1)[1] for key in states if key.startswith(route+':')]:
   errors.clear()
   try:
    page.set_content('<div id="container">'+markup+'</div><p role="status" id="status"></p>')
    page.add_style_tag(path=str(upstream/'dist/style/weui.css'))
    page.add_style_tag(path=str(upstream/'dist/example/example.css'))
    page.add_style_tag(path=str(root/'website/playground/bridge.css'))
    page.add_script_tag(path=str(root/'website/playground/bridge.js'))
    page.evaluate('''({route,state})=>{const p=document.querySelector('#container .page');p.classList.add(route,'js_show');window.WeGPUIPlayground.mount({route,page:p,announce:m=>document.querySelector('#status').textContent=m,navigate:r=>{document.body.dataset.navigate=r}}); window.WeGPUIPlayground.applyState(state)}''',{'route':route,'state':state})
    page.wait_for_timeout(30)
    assert not errors,errors
    results.append({'route':route,'state':state,'pass':True})
   except Exception as exc: results.append({'route':route,'state':state,'pass':False,'error':str(exc)})
 browser.close()
Path('/workspace/.wegpui-cloud/bridge-test/mount-results.json').write_text(json.dumps(results,indent=2)+'\n')
print(json.dumps({'cases':len(results),'failures':[r for r in results if not r['pass']]}))
