#!/usr/bin/env python3
"""Capture local browser adapters, not native GPUI or upstream parity acceptance."""
import argparse,base64,hashlib,json,tempfile,threading
from datetime import datetime,timezone
from functools import partial
from http.server import ThreadingHTTPServer,SimpleHTTPRequestHandler
from pathlib import Path
from urllib.parse import urlencode,urlparse
from playwright.sync_api import sync_playwright
from PIL import Image,ImageDraw
ROOT=Path(__file__).resolve().parents[4]
CASES=[('button_default','default'),('dialog','ios1'),('half-screen-dialog','4'),('picker','single'),('picker','date'),('picker','cascade'),('searchbar_filled','typed'),('uploader','viewer')]
MODES={'light':{},'dark':{'theme':'dark'},'care':{'care':'1'},'brand':{'brand':'7C3AED'}}
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def main():
 parser=argparse.ArgumentParser();parser.add_argument('--site',type=Path,required=True);parser.add_argument('--out',type=Path,required=True);args=parser.parse_args();args.out.mkdir(parents=True,exist_ok=True)
 metadata={'utc':datetime.now(timezone.utc).isoformat(),'scope':'Local browser adapters; no native GPUI execution or complete parity/AT acceptance','cases':[],'errors':[],'external_requests':[], 'source':{str(path.relative_to(args.site)):sha(path) for path in sorted((args.site/'playground').rglob('*')) if path.is_file()}}
 class Quiet(SimpleHTTPRequestHandler):
  def log_message(self,*args):pass
 with tempfile.TemporaryDirectory() as temp:
  (Path(temp)/'weGPUI').symlink_to(args.site.resolve(),target_is_directory=True)
  server=ThreadingHTTPServer(('127.0.0.1',0),partial(Quiet,directory=temp));thread=threading.Thread(target=server.serve_forever,daemon=True);thread.start();base=f'http://127.0.0.1:{server.server_port}/weGPUI/playground/index.html'
  try:
   with sync_playwright() as pw:
    browser=pw.chromium.launch(executable_path='/usr/bin/chromium',headless=True,args=['--no-sandbox']);metadata['browser']=browser.version
    context=browser.new_context(viewport={'width':375,'height':812},device_scale_factor=1)
    def local_only(route):
     if urlparse(route.request.url).scheme in ('data','blob','about') or urlparse(route.request.url).netloc==f'127.0.0.1:{server.server_port}':route.continue_()
     else:metadata['external_requests'].append(route.request.url);route.abort()
    context.route('**/*',local_only);page=context.new_page();page.set_default_timeout(6000);page.on('pageerror',lambda e:metadata['errors'].append(str(e)))
    for mode,overrides in MODES.items():
     for route,state in CASES:
      params={'state':state,'theme':'light',**overrides};url=base+'#'+route+'?'+urlencode(params);page.goto(url);page.wait_for_function("document.body.dataset.ready==='true'");page.wait_for_timeout(220)
      target=args.out/f'{mode}-{route}-{state}.png';page.screenshot(path=str(target));metadata['cases'].append({'mode':mode,'route':route,'state':state,'capture':target.name,'ready':page.locator('body').get_attribute('data-ready'),'focus':page.evaluate("document.activeElement.outerHTML.slice(0,500)")})
    browser.close()
  finally:server.shutdown();server.server_close();thread.join()
 for mode in MODES:
  sheet=Image.new('RGB',(1000,1128),'#ddd');draw=ImageDraw.Draw(sheet)
  for index,(route,state) in enumerate(CASES):
   im=Image.open(args.out/f'{mode}-{route}-{state}.png').convert('RGB');im.thumbnail((250,542));x=(index%4)*250;y=(index//4)*564;draw.text((x+5,y+3),f'{mode} {route}:{state}',fill='black');sheet.paste(im,(x,y+22))
  sheet.save(args.out/f'sheet-{mode}.jpg',quality=90)
 (args.out/'metadata.json').write_text(json.dumps(metadata,indent=2,ensure_ascii=False)+'\n');print(json.dumps({'captures':len(metadata['cases']),'errors':metadata['errors'],'outside_requests':metadata['external_requests']}))
if __name__=='__main__':main()
