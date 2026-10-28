import json,sys,pyatspi
def walk(n):
    yield n
    for i in range(n.childCount):
        child=n.getChildAtIndex(i)
        if child: yield from walk(child)
apps=[n for n in walk(pyatspi.Registry.getDesktop(0)) if n.getRoleName()=='application' and n.name=='date_canada_candidate']
assert len(apps)==1, len(apps)
out=[]
for n in walk(apps[0]):
    try:
        b=n.queryComponent().getExtents(pyatspi.DESKTOP_COORDS)
        bounds={'x':b.x,'y':b.y,'width':b.width,'height':b.height}
    except Exception: bounds=None
    out.append({'role':n.getRoleName(),'name':n.name,'bounds':bounds,
                'states':[pyatspi.stateToString(s) for s in n.getState().getStates()]})
if sys.argv[1]=='picker':
    assert len([n for n in out if n['role']=='list box'])==3, out
else:
    assert any(n['role']=='status bar' and n['name'].startswith('Text ') for n in out),out
print(json.dumps(out,ensure_ascii=False,indent=2))
