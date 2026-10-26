from pathlib import Path
from PIL import Image
from collections import Counter
import json
root=Path('<prior-evidence>/current-captures/light-care')
result={}
for name in ['gpui','weui']:
 im=Image.open(root/('half-screen-dialog-2-'+name+'.png')).convert('RGB')
 histogram=Counter(im.crop((48,580,700,680)).getdata())
 print(name,'colors',histogram.most_common(5))
 rows=[]
 for y in range(570,680):
  count=sum(im.getpixel((x,y))==(148,148,148) for x in range(48,700))
  if count>=8:rows.append(y)
 result[name]=rows
print(json.dumps(result))
