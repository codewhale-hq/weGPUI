from http.server import ThreadingHTTPServer,SimpleHTTPRequestHandler
from functools import partial
from threading import Thread
from pathlib import Path
import json
from playwright.sync_api import sync_playwright
class Quiet(SimpleHTTPRequestHandler):
 def log_message(self,*_):pass
server=ThreadingHTTPServer(('127.0.0.1',0),partial(Quiet,directory='<prior-evidence>/reference'))
Thread(target=server.serve_forever,daemon=True).start()
result=[]
with sync_playwright() as p:
 browser=p.chromium.launch(executable_path='/usr/bin/chromium')
 for width,height in [(375,812),(320,812),(480,812),(375,480)]:
  page=browser.new_page(viewport={'width':width,'height':height})
  page.goto(f'http://127.0.0.1:{server.server_port}/dist/example/index.html#layers')
  page.wait_for_timeout(2300)
  client=page.context.new_cdp_session(page);client.send('DOM.enable')
  root=client.send('DOM.getDocument')['root']['nodeId']
  record={'viewport':[width,height],'layers':[]}
  for name in ['content','navigation','mask','popout']:
   selector='.layers__layer_'+name
   node=client.send('DOM.querySelector',{'nodeId':root,'selector':selector})['nodeId']
   element=page.locator(selector)
   style=element.evaluate("e=>{const s=getComputedStyle(e);const r=e.parentElement.parentElement.getBoundingClientRect();return {width:s.width,height:s.height,transform:s.transform,origin:s.transformOrigin,parentY:r.y,parentStyle:getComputedStyle(e.parentElement).transformStyle}}")
   quad=client.send('DOM.getBoxModel',{'nodeId':node})['model']['border']
   record['layers'].append({'name':name,'style':style,'quad':quad})
  result.append(record);page.close()
 browser.close()
server.shutdown()
Path('<evidence>/layer-reference.json').write_text(json.dumps(result,indent=2)+'\n')
for row in result:print(row['viewport'],row['layers'][0])
