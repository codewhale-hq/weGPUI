from pathlib import Path
import json, os, subprocess, time
import pyatspi
from gi.repository import GLib
from PIL import Image
out=Path('<runtime-evidence>/components')
def nodes(node=None):
    if node is None: node=pyatspi.Registry.getDesktop(0)
    yield node
    for i in range(node.childCount):
        try: yield from nodes(node.getChildAtIndex(i))
        except Exception: pass

def find(name,role=None):
    end=time.monotonic()+6
    while time.monotonic()<end:
        ctx=GLib.MainContext.default()
        for _ in range(100):
            if not ctx.pending():break
            ctx.iteration(False)
        for n in nodes():
            try:
                if n.name==name and (role is None or n.getRoleName()==role):return n
            except Exception: pass
        time.sleep(.1)
    raise AssertionError((name,role))
def bounds(node):return tuple(node.queryComponent().getExtents(pyatspi.DESKTOP_COORDS))
def lum(pixel):
    values=[x/255 for x in pixel[:3]]
    return sum(w*(v/12.92 if v<=.04045 else ((v+.055)/1.055)**2.4) for w,v in zip([.2126,.7152,.0722],values))
def contrast(a,b):
    a,b=sorted([lum(a),lum(b)])
    return (b+.05)/(a+.05)
results=[]
for dark in [False,True]:
 for care in [False,True]:
  for brand in [False,True]:
    mode=('-'.join(['dark' if dark else 'light']+(['care'] if care else [])+(['brand'] if brand else [])))
    log=(out/(mode+'.log')).open('w')
    app=subprocess.Popen([os.environ['CARGO_TARGET_DIR']+'/debug/contracts',mode],stdout=log,stderr=log)
    try:
        button=find('Neutral action','button')
        x,y,w,h=bounds(button)
        subprocess.run(['xdotool','mousemove',str(x+w//2),str(y+h//2),'click','1'],check=True)
        time.sleep(.5)
        footer=find('Help','link');fx,fy,fw,fh=bounds(footer)
        assert fw>=44 and fh>=44,(mode,bounds(footer))
        agreement=next(n for n in nodes() if n.getRoleName()=='check box')
        ax,ay,aw,ah=bounds(agreement)
        frame=next(n for n in nodes() if n.getRoleName()=='frame')
        bx,by,bw,bh=bounds(frame)
        tx,ty,tw,th=bounds(find('Terms','link'))
        assert aw<=bw-32 and ax>=bx and ax+aw<=bx+bw,(mode,bounds(agreement))
        assert tx>=ax and tx+tw<=ax+aw and ty+th<=ay+ah,(mode,bounds(agreement),(tx,ty,tw,th))
        wid=subprocess.check_output(['xdotool','search','--onlyvisible','--pid',str(app.pid)],text=True).split()[-1]
        subprocess.run(['xdotool','mousemove','0','0'],check=True)
        subprocess.run(['import','-window',wid,str(out/(mode+'.png'))],check=True)
        im=Image.open(out/(mode+'.png')).convert('RGB')
        box=(int(x-bx+4),int(y-by+4),int(x-bx+w-4),int(y-by+h-4))
        crop=im.crop(box); bg=crop.getpixel((0,0))
        peak=max(contrast(p,bg) for p in crop.getdata())
        assert peak>=4.5,(mode,peak)
        results.append({'mode':mode,'neutral_peak_contrast':round(peak,3),'footer_target':[fw,fh],'agreement_bounds':[aw,ah]})
        print('PASS',results[-1],flush=True)
    finally:
        app.terminate();app.wait(timeout=5);log.close()
(out/'results.json').write_text(json.dumps(results,indent=2)+'\n')
print('ALL EIGHT COMPONENT MODES PASSED',flush=True)
