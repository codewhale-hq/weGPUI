"""Actual final black/white caret and FilledGrey action focus pixels, eight theme combinations."""
from pathlib import Path
from PIL import Image,ImageDraw
import json

ROOT=Path(__file__).parent
MODES=['light','dark','care','dark-care','brand','dark-brand','care-brand','dark-care-brand']
WHITE,BLACK=(255,255,255),(0,0,0)

def luminance(rgb):
    v=[x/255 for x in rgb]
    v=[x/12.92 if x<=.04045 else((x+.055)/1.055)**2.4 for x in v]
    return sum(x*k for x,k in zip(v,(.2126,.7152,.0722)))

def contrast(a,b):
    a,b=sorted((luminance(a),luminance(b)))
    return(b+.05)/(a+.05)

def choose(preferred,surface):
    return preferred if contrast(preferred,surface)>=3 else max((WHITE,BLACK),key=lambda color:contrast(color,surface))

rows=[];actions=[]
sheet=Image.new('RGB',(1200,640),'#ddd');draw=ImageDraw.Draw(sheet)
for i,mode in enumerate(MODES):
    q=ROOT/f'caret-final-{mode}';dark='dark'in mode;care='care'in mode
    bg2=(25,25,25)if dark else WHITE
    grey=(56,56,56)if dark else(237,237,237)
    brand=(124,58,237)if'brand'in mode else((1,137,66)if care and not dark else(7,193,96))
    for j,(step,surface)in enumerate([('caret-outlined',bg2),('caret-filled-grey',grey)]):
        g=json.loads((q/f'{step}-geometry.json').read_text());x,y,w,h=g['last_character'];im=Image.open(q/f'{step}.png').convert('RGB');end=x+w;ym=int(y+h/2)
        matches=[xc for xc in range(end-2,end+3)if [im.getpixel((xc+a,ym))for a in(-1,0,1)]==[WHITE,BLACK,WHITE]]
        assert len(matches)==1,(mode,step,end,matches)
        xc=matches[0];bg=im.getpixel((xc+5,ym));assert bg==surface,(mode,step,bg,surface)
        ratio=max(contrast(BLACK,bg),contrast(WHITE,bg));assert ratio>=3,(mode,step,ratio)
        rows.append({'mode':mode,'step':step,'point':[xc,ym],'core':BLACK,'side_backing':WHITE,'actual_surface':bg,'max_surface_contrast':ratio,'a11y_rounded_end':end})
        crop=im.crop((xc-12,y-4,xc+12,y+h+4)).resize((96,128));a=(i%4)*300+j*140;b=(i//4)*320;sheet.paste(crop,(a+8,b+30));draw.text((a+8,b+5),f'{mode}/{"grey"if j else"outlined"}',fill='black')
    for step,surface in [('grey-clear',grey),('grey-cancel',bg2),('grey-back',bg2)]:
        tree=json.loads((q/f'{step}-atspi.json').read_text());node=next(n for n in tree if 'focused'in n['states']);x,y,w,h=node['bounds'];im=Image.open(q/f'{step}.png').convert('RGB');point=(int(x+1),int(y+h/2));actual=im.getpixel(point);expected=choose(brand,surface)
        assert max(abs(a-b)for a,b in zip(actual,expected))<=1,(mode,step,actual,expected)
        assert contrast(actual,surface)>=3,(mode,step,actual,surface)
        actions.append({'mode':mode,'step':step,'point':point,'actual':actual,'surface':surface,'contrast':contrast(actual,surface)})
(ROOT/'caret-final-pixels.json').write_text(json.dumps(rows,indent=2)+'\n');(ROOT/'grey-action-pixels.json').write_text(json.dumps(actions,indent=2)+'\n');sheet.save(ROOT/'review-caret-final.png')
print(f'PASS {len(rows)} final caret surfaces and {len(actions)} FilledGrey action focus states')
