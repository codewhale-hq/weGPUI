"""Inspect the one-pixel caret core and its one-pixel side backing in build7 captures."""
from pathlib import Path
from PIL import Image,ImageDraw
import json

ROOT=Path(__file__).parent
MODES=['light','dark','care','dark-care','brand','dark-brand','care-brand','dark-care-brand']
rows=[]
sheet=Image.new('RGB',(1000,320),'#ddd');draw=ImageDraw.Draw(sheet)
for i,mode in enumerate(MODES):
    q=ROOT/f'caret-{mode}';g=json.loads((q/'caret-geometry.json').read_text())
    x,y,w,h=g['last_character'];im=Image.open(q/'caret.png').convert('RGB');end=x+w;ym=int(y+h/2)
    dark='dark'in mode;care='care'in mode
    expected=(124,58,237)if'brand'in mode else((1,137,66)if care and not dark else(7,193,96))
    # AT-SPI character extents round fractional glyph advances; permit its <=2px edge rounding.
    matches=[]
    for xc in range(end-2,end+3):
        core=im.getpixel((xc,ym));sides=[im.getpixel((xc+a,ym))for a in(-1,1)]
        if max(abs(a-b)for a,b in zip(core,expected))<=1 and sides[0]==sides[1] and sides[0]in((0,0,0),(255,255,255)):
            matches.append((xc,core,sides[0]))
    assert len(matches)==1,(mode,end,matches)
    xc,core,backing=matches[0]
    rows.append({'mode':mode,'point':[xc,ym],'core':core,'backing':backing,'a11y_rounded_end':end,
                 'surface':(25,25,25)if dark else(255,255,255)})
    crop=im.crop((xc-12,y-4,xc+12,y+h+4)).resize((96,128));a=(i%4)*250;b=(i//4)*160
    sheet.paste(crop,(a+10,b+25));draw.text((a+10,b+5),mode,fill='black')
(ROOT/'caret-pixels.json').write_text(json.dumps(rows,indent=2)+'\n');sheet.save(ROOT/'review-caret.png')
print('PASS build7 brand-core / solid side-backing pixel checks on8 outlinedBG2 inputs; arbitrarybackground contrast remains limited')
