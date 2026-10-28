import json,re
from pathlib import Path
out=Path('/work/full-parity/date-canada-candidate');base=Path('/work/full-parity/date-canada-probe')
def load(root,label):return json.loads((root/f'{label}.json').read_text())
def status(root,label):
    rows=[n['name'] for n in load(root,label) if n['role']=='status bar']
    assert len(rows)==1,(root,label,rows)
    return rows[0]
def picker(root,label):
    rows=[n for n in load(root,label) if n['role']=='list box']
    assert len(rows)==3,(root,label,rows)
    return [n['name'] for n in sorted(rows,key=lambda n:n['bounds']['x'])]
def commit(root,label):
    values=re.findall(r'^commit=(.*)$',(root/f'{label}.log').read_text(),re.M)
    assert len(values)==1,(root,label,values)
    return values[0]
result={}
for tag in ('en-CA','fr-CA'):
    assert status(base,tag+'-display')=='Text 04/03/2027; ISO 2027-03-04'
    assert status(out,tag+'-display')=='Text 2027-03-04; ISO 2027-03-04'
    assert commit(base,tag+'-entry')==commit(out,tag+'-entry')
    assert status(out,tag+'-entry')=='Text 2027-03-04; ISO 2027-03-04'
    old,new=picker(base,tag+'-picker'),picker(out,tag+'-picker')
    assert old[0].endswith(' 4') and old[2].endswith(' 2027'),old
    assert new[0].endswith(' 2027') and new[2].endswith(' 4') and new[1]==old[1],new
    assert commit(base,tag+'-ambiguous')==commit(out,tag+'-ambiguous')
    assert 'year: 2027, month: 4, day: 3' in commit(out,tag+'-ambiguous')
    assert status(out,tag+'-ambiguous')=='Text 2027-04-03; ISO 2027-04-03'
    assert commit(base,tag+'-invalid')==commit(out,tag+'-invalid')=='Err(Date)'
    result[tag]={'display_before':status(base,tag+'-display'),'display_after':status(out,tag+'-display'),
                 'picker_before':old,'picker_after':new,'legacy_ambiguous_commit':commit(out,tag+'-ambiguous')}
for tag in ('en','en-US','en-GB','zh-CN','de'):
    for mode in ('display','entry'):assert status(base,tag+'-'+mode)==status(out,tag+'-'+mode),(tag,mode)
    assert commit(base,tag+'-entry')==commit(out,tag+'-entry'),tag
    assert picker(base,tag+'-picker')==picker(out,tag+'-picker'),tag
for label,expected in [('en-CA-range','Err(OutOfRange { start: 1900, end: 2100 })'),
                       ('en-CA-invalid-iso','Err(Date)'),('en-CA-year-first-slash','Err(Format)')]:
    assert commit(base,label)==commit(out,label)==expected,(label,commit(out,label))
    assert status(out,label).endswith('ISO 2027-03-04'),label
for label,fragment in [('en-CA-iso','year: 2027, month: 3, day: 4'),
                       ('en-CA-datetime-legacy','year: 2027, month: 4, day: 3, hour: 5, minute: 6'),
                       ('en-CA-datetime-iso','year: 2027, month: 3, day: 4, hour: 5, minute: 6')]:
    assert commit(base,label)==commit(out,label) and fragment in commit(out,label),label
result['controls']={'unchanged_locales':['en','en-US','en-GB','zh-CN','de'],
                    'invalid_and_range':'pass','datetime_legacy_and_iso':'pass'}
(out/'result.json').write_text(json.dumps(result,indent=2,ensure_ascii=False)+'\n')
print('PASS: Canada display/picker order, old and ISO entry, ambiguous input, invalid/range, DateTime and five locale controls')
