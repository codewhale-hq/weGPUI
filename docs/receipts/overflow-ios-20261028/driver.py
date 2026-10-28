import json,os,subprocess,time,threading,hashlib
from pathlib import Path
from http.server import SimpleHTTPRequestHandler,ThreadingHTTPServer
from functools import partial
from playwright.sync_api import sync_playwright
from PIL import Image
root=Path('/work/full-parity');out=root/'dictionary-overflow-normal';out.mkdir(exist_ok=True)
env=dict(os.environ,DISPLAY=':216',GPUI_X11_SCALE_FACTOR='2',LIBGL_ALWAYS_SOFTWARE='1',VK_ICD_FILENAMES='/usr/share/vulkan/icd.d/lvp_icd.json',XDG_RUNTIME_DIR='/work/runtime')
log=(out/'native.txt').open('w');xvfb=subprocess.Popen(['Xvfb',':216','-screen','0','2400x2100x24','-nolisten','tcp'],env=env,stdout=log,stderr=log)
for _ in range(100):
 if subprocess.run(['xdpyinfo','-display',':216'],stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL).returncode==0:break
 time.sleep(.05)
wm=subprocess.Popen(['openbox','--sm-disable'],env=env,stdout=log,stderr=log)
(root/'dictionary-native.json').unlink(missing_ok=True)
process=subprocess.Popen(['/target/debug/wegpui-dictionary-render-probe'],env=env,stdout=log,stderr=log)
try:
 for _ in range(200):
  found=subprocess.run(['xdotool','search','--onlyvisible','--pid',str(process.pid)],env=env,capture_output=True,text=True)
  if found.stdout.strip() and (root/'dictionary-native.json').exists():break
  if process.poll() is not None:raise RuntimeError('Native fixture exited')
  time.sleep(.05)
 else:raise RuntimeError('Native fixture not ready')
 window=found.stdout.splitlines()[0]
 for _ in range(20):
  subprocess.run(['import','-window',window,str(out/'native.png')],env=env,check=True,capture_output=True)
  image=Image.open(out/'native.png').convert('RGB')
  if any(a!=b for a,b in image.getextrema()):break
  time.sleep(.25)
 else:raise RuntimeError('Native frame blank')
 (out/'native.json').write_bytes((root/'dictionary-native.json').read_bytes())
finally:
 process.terminate();process.wait(timeout=5);wm.terminate();xvfb.terminate();log.close()
class Quiet(SimpleHTTPRequestHandler):
 def log_message(self,*args):pass
server=ThreadingHTTPServer(('127.0.0.1',0),partial(Quiet,directory='/work/code-parity/reference'));threading.Thread(target=server.serve_forever,daemon=True).start()
native=json.loads((out/'native.json').read_text());results=[]
(out/'binary-sha256.txt').write_text(hashlib.sha256(Path('/target/debug/wegpui-dictionary-render-probe').read_bytes()).hexdigest()+'\n')
with sync_playwright() as p:
 browser=p.chromium.launch(executable_path='/usr/bin/chromium');page=browser.new_page(viewport={'width':1024,'height':900},locale='zh-CN')
 page.goto(f'http://127.0.0.1:{server.server_port}/dist/example/index.html#form_page');page.wait_for_function("Array.from(document.querySelectorAll('#container > .page')).at(-1)?.classList.contains('js_show')")
 for case in native:
  desc=page.locator('.weui-form__desc');desc.evaluate("(e,c)=>{e.textContent=c.text;e.style.width=c.width+'px';e.style.fontFamily=c.language==='lo'?\"'Noto Sans Lao',system-ui\":''}",case)
  measured_data=desc.evaluate('''e=>{let t=e.firstChild,rows={};for(let i=0;i<t.length;i++){let r=new Range();r.setStart(t,i);r.setEnd(t,i+1);let b=r.getBoundingClientRect();let row=rows[b.y]??={text:'',left:Infinity,right:-Infinity};row.text+=t.textContent[i];row.left=Math.min(row.left,b.left);row.right=Math.max(row.right,b.right)}return {lines:Object.values(rows).map(r=>r.text),widths:Object.values(rows).map(r=>r.right-r.left)}}''')
  measured=measured_data['lines']
  desc.evaluate("e=>e.style.whiteSpace='nowrap'")
  full_width=desc.evaluate("e=>{const r=new Range();r.selectNodeContents(e);return r.getBoundingClientRect().width}")
  session=page.context.new_cdp_session(page);session.send('DOM.enable');session.send('CSS.enable');node=session.send('DOM.querySelector',{'nodeId':session.send('DOM.getDocument')['root']['nodeId'],'selector':'.weui-form__desc'})['nodeId'];fonts=session.send('CSS.getPlatformFontsForNode',{'nodeId':node});session.detach()
  desc.evaluate("e=>e.style.whiteSpace='normal'")
  native_lines=[' '.join(s.split()) for s in case['lines']];reference_lines=[' '.join(s.split()) for s in measured]
  result=dict(case,reference_lines=measured,reference_row_widths=measured_data['widths'],reference_full_width=full_width,reference_fonts=fonts,reference_words=desc.evaluate("(e,lang)=>Array.from(new Intl.Segmenter(lang,{granularity:'word'}).segment(e.textContent),x=>({index:new TextEncoder().encode(e.textContent.slice(0,x.index)).length,segment:x.segment}))",case['language']),match=native_lines==reference_lines);results.append(result)
 browser.close()
server.shutdown();(out/'comparison.json').write_text(json.dumps(results,indent=2,ensure_ascii=False)+'\n')
print(json.dumps([{'language':r['language'],'width':r['width'],'native_rows':len(r['lines']),'reference_rows':len(r['reference_lines']),'match':r['match']} for r in results],indent=2))
