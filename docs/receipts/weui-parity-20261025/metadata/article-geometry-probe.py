from pathlib import Path
import json
from PIL import Image
root=Path('<evidence>');result=[]
for mode in ['light','light-brand','light-care','light-care-brand','dark','dark-brand','dark-care','dark-care-brand']:
 row={'mode':mode}
 for group,label in [('final-captures','before'),('current-captures','after')]:
  for suffix in ['gpui','weui']:
   image=Image.open(root/group/mode/('article-'+suffix+'.png')).convert('RGB')
   ys=[y for y in range(600,image.height-100) if all(image.getpixel((x,y+k))==(235,235,235) for x in [50,60,80] for k in [0,25,50,99])]
   assert ys,(group,mode,suffix)
   row[label+'_'+suffix+'_image_y']=min(ys)/2
 row['after_offset_logical_px']=row['after_gpui_image_y']-row['after_weui_image_y']
 assert abs(row['after_offset_logical_px'])<=2,row
 result.append(row)
(root/'article-geometry.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(result))
