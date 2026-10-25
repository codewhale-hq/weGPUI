"""Compare actual rendered ring pixels with independently calculated WCAG contrast."""
import json
from pathlib import Path
from PIL import Image

ROOT = Path(__file__).parent
MODES = ['light', 'dark', 'care', 'dark-care', 'brand', 'dark-brand', 'care-brand', 'dark-care-brand']
WHITE, BLACK = (255, 255, 255), (0, 0, 0)

def luminance(rgb):
    values = [v/255 for v in rgb]
    values = [v/12.92 if v <= .04045 else ((v+.055)/1.055)**2.4 for v in values]
    return sum(v*k for v,k in zip(values, (.2126,.7152,.0722)))

def contrast(a,b):
    a,b = sorted((luminance(a),luminance(b)))
    return (b+.05)/(a+.05)

def choose(preferred, surface):
    if contrast(preferred,surface) >= 3:
        return preferred
    return max((BLACK,WHITE),key=lambda value: contrast(value,surface))

def close(a,b):
    return max(abs(x-y)for x,y in zip(a,b)) <= 1

checks=[]
for mode in MODES:
    directory=ROOT/mode
    dark='dark'in mode
    care='care'in mode
    brand=(124,58,237)if'brand'in mode else ((1,137,66)if care and not dark else (7,193,96))
    bg2=(25,25,25)if dark else WHITE
    bg3=(32,32,32)if dark else (247,247,247)
    red=(220,54,54)if care and not dark else (250,81,81)
    known={'primary':(WHITE,brand,1,6),'warn':(WHITE,red,1,6),
           'homepage':(brand,bg2,1,None),'camera':(brand,bg2,1,None),
           'search-back':(brand,bg2,1,None),'search-clear':(brand,bg2,1,None),
           'search-submit':(brand,brand,1,6),'search-cancel':(brand,bg2,1,None),
           'picker':(brand,bg3,9,16)}
    for step,(preferred,surface,offset,background_offset)in known.items():
        tree=json.loads((directory/f'{step}-atspi.json').read_text())
        node=next(n for n in tree if'focused'in n['states'])
        x,y,w,h=node['bounds'];im=Image.open(directory/f'{step}.png').convert('RGB')
        point=(int(x+offset),int(y+h/2));actual=im.getpixel(point);expected=choose(preferred,surface)
        assert close(actual,expected),(mode,step,point,actual,expected)
        assert contrast(actual,surface)>=3,(mode,step,actual,surface,contrast(actual,surface))
        row={'mode':mode,'step':step,'point':point,'actual':actual,'expected':expected,
             'surface':surface,'contrast':contrast(actual,surface),'preserved_preferred':expected==preferred}
        if background_offset is not None:
            bgpoint=(int(x+background_offset),int(y+h/2))
            bgactual=im.getpixel(bgpoint)
            assert close(bgactual,surface),(mode,step,'surface',bgpoint,bgactual,surface)
            row.update(surface_point=bgpoint,surface_actual=bgactual,pixel_contrast=contrast(actual,bgactual))
        checks.append(row)
    for step,outer in [('default',False),('checkbox-off',True),('checkbox-on',True),
                       ('switch-on',True),('checkbox-row',False),('radio',False),('image',False)]:
        tree=json.loads((directory/f'{step}-atspi.json').read_text())
        node=next(n for n in tree if'focused'in n['states'])
        x,y,w,h=node['bounds'];im=Image.open(directory/f'{step}.png').convert('RGB')
        a,b=(-1,-3)if outer else(1,3)
        points=[(int(x+offset),int(y+h/2))for offset in(a,b)]
        colors=[im.getpixel(point)for point in points]
        assert colors==[WHITE,BLACK],(mode,step,points,colors)
        checks.append({'mode':mode,'step':step,'points':points,'actual':colors,
                       'band_contrast':contrast(*colors),'two_2px_bands':True})
(ROOT/'pixel-checks.json').write_text(json.dumps(checks,indent=2)+'\n')
print(f'PASS {len(checks)} focus states: actual pixels match helper selection / dual bands across 8 theme combinations')
