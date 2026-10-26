from pathlib import Path
import json, os, subprocess, time
import pyatspi
from gi.repository import GLib
def pump():
    context=GLib.MainContext.default()
    while context.pending(): context.iteration(False)
out=Path(__file__).resolve().parent
log=(out/'app.log').open('w')
app=subprocess.Popen([os.environ['CARGO_TARGET_DIR']+'/debug/wegpui-preview'],stdout=log,stderr=log)
def nodes(node=None):
    if node is None: node=pyatspi.Registry.getDesktop(0)
    yield node
    for i in range(node.childCount):
        try: yield from nodes(node.getChildAtIndex(i))
        except Exception: pass

def find(name,role=None,timeout=4):
    deadline=time.monotonic()+timeout
    while time.monotonic()<deadline:
        pump()
        for n in nodes():
            try:
                if n.name==name and (role is None or n.getRoleName()==role): return n
            except Exception: pass
        time.sleep(.1)
    raise AssertionError(f'Missing {role}: {name}')
def bounds(n): return tuple(n.queryComponent().getExtents(pyatspi.DESKTOP_COORDS))
def action(name,role=None):
    n=find(name,role)
    for _ in range(24):
        pump()
        if n.getState().contains(pyatspi.STATE_FOCUSED): break
        subprocess.run(['xdotool','key','Tab'],check=True)
        time.sleep(.12)
        n=find(name,role)
    assert n.getState().contains(pyatspi.STATE_FOCUSED),('Could not focus',name)
    a=n.queryAction()
    assert a.nActions>0,name
    names=[a.getName(i) for i in range(a.nActions)]
    for i,action_name in enumerate(names):
        if action_name.lower()=='focus':
            a.doAction(i)
            time.sleep(.6)
    n=find(name,role)
    a=n.queryAction()
    names=[a.getName(i) for i in range(a.nActions)]
    print('ACTIVATE',name,names,bounds(n),flush=True)
    index=next((i for i,n in enumerate(names) if n.lower() in ['click','press','activate']),None)
    assert index is not None,(name,names)
    assert a.doAction(index),name
    time.sleep(.6)
def pointer(name,role=None,edge=False):
    x,y,w,h=bounds(find(name,role))
    subprocess.run(['xdotool','mousemove',str(x+3 if edge else x+w//2),str(y+h//2),'click','1'],check=True)
    time.sleep(.6)
def snapshot(label):
    result=[]
    for n in nodes():
        try: result.append({'role':n.getRoleName(),'name':n.name,'bounds':bounds(n),'checked':n.getState().contains(pyatspi.STATE_CHECKED)})
        except Exception: pass
    (out/(label+'.json')).write_text(json.dumps(result,indent=2))
    ids=subprocess.check_output(['xdotool','search','--onlyvisible','--pid',str(app.pid)],text=True).split()
    subprocess.run(['import','-window',ids[-1],str(out/(label+'.png'))],check=True)
try:
    find('Review draft','button')
    wid=subprocess.check_output(['xdotool','search','--onlyvisible','--pid',str(app.pid)],text=True).split()[-1]
    subprocess.run(['xdotool','windowactivate','--sync',wid],check=True)
    time.sleep(.5)
    pointer('Name','entry')
    print('NATIVE FOCUS',subprocess.check_output(['xdotool','getwindowfocus'],text=True).strip(),wid,flush=True)
    reset=find('Reset draft','link')
    assert bounds(reset)[2]>=44 and bounds(reset)[3]>=44,bounds(reset)
    print('PASS: footer pointer/accessibility target is at least 44 x 44',bounds(reset),flush=True)
    snapshot('initial')
    action('Review draft','button')
    find('Enter your name before reviewing the draft.','status bar')
    assert find('Name','entry').getState().contains(pyatspi.STATE_INVALID_ENTRY)
    print('PASS: required-name validation and invalid entry state',flush=True)
    pointer('Name','entry')
    subprocess.run(['xdotool','type','--clearmodifiers','--delay','12','Hunter'],check=True)
    time.sleep(.5)
    assert find('Name','entry').queryText().getText(0,-1)=='Hunter'
    action('Review draft','button')
    find('Accept the terms before reviewing the draft.','status bar')
    label="I have read the draft and understand that confirming it keeps this example's data in memory. Read the terms"
    action('Read the terms','link')
    assert not find(label,'check box').getState().contains(pyatspi.STATE_CHECKED)
    print('PASS: terms activation leaves consent unchecked',flush=True)
    action(label,'check box')
    assert find(label,'check box').getState().contains(pyatspi.STATE_CHECKED)
    action('Review draft','button')
    find('Confirm this draft?','dialog')
    assert not any(n.name=='Reset draft' for n in nodes())
    action('Cancel')
    find('Review draft','button')
    action('Review draft','button')
    find('Confirm this draft?','dialog')
    action('Confirm')
    find('Confirmed drafts: 1','paragraph')
    print('PASS: modal isolation, Cancel, reopen, Confirm and exactly one count',flush=True)
    snapshot('confirmed')
    action('Dark mode','button')
    find('Light mode','button')
    snapshot('dark')
    action('Light mode','button')
    pointer('Reset draft','link',edge=True)
    assert find('Name','entry').queryText().getText(0,-1)==''
    assert not find(label,'check box').getState().contains(pyatspi.STATE_CHECKED)
    find('Confirmed drafts: 1','paragraph')
    print('PASS: edge-of-target reset clears draft without changing confirmed count',flush=True)
    snapshot('reset')
    pointer('Upload image','button')
    find('Open File','file chooser',timeout=8)
    pointer('Open File','file chooser')
    subprocess.run(['xdotool','key','ctrl+l'],check=True)
    subprocess.run(['xdotool','type','--clearmodifiers','--delay','2',str(out/'chosen.png')],check=True)
    subprocess.run(['xdotool','key','Return'],check=True)
    time.sleep(.7)
    try:
        find('Images, 1/3','list item',timeout=2)
    except AssertionError:
        pointer('Choose images','button')
        find('Images, 1/3','list item',timeout=8)
    print('PASS: native GTK portal selected one image',flush=True)
    snapshot('image-selected')
    pointer('chosen.png','image')
    find('Image preview','dialog')
    snapshot('image-preview')
    subprocess.run(['xdotool','key','Escape'],check=True)
    time.sleep(.6)
    find('Images, 1/3','list item')
    pointer('Delete','button')
    find('Images, 0/3','list item')
    print('PASS: image preview, Escape dismissal and deletion',flush=True)
    snapshot('image-deleted')
    print('ALL APPLICATION CONTRACTS PASSED',flush=True)
except Exception:
    snapshot('failure')
    raise
finally:
    app.terminate()
    app.wait(timeout=5)
