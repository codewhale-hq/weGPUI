import argparse,json,pathlib
p=argparse.ArgumentParser();p.add_argument('baseline',type=pathlib.Path);p.add_argument('current',type=pathlib.Path);args=p.parse_args()
def read(d,name):return json.loads((d/name).read_text())
bm=read(args.baseline,'summary.json');cm=read(args.current,'summary.json');errors=[]
for key in ['corpus_sha256','browser_version','browser_executable_sha256','browser_locale','lao_font_sha256','reference_asset_hashes']:
 if bm.get(key)!=cm.get(key):errors.append(f'{key} differs')
bs={k:v for k,v in bm['build_inputs'].items() if k.startswith('source/')};cs={k:v for k,v in cm['build_inputs'].items() if k.startswith('source/')}
if bs!=cs:errors.append('product source snapshot differs')
b={(x['id'],x['width']):x for x in read(args.baseline,'comparison.json')};c={(x['id'],x['width']):x for x in read(args.current,'comparison.json')}
if set(b)!=set(c):errors.append('case-width inventory differs')
rows=[]
for key in sorted(set(b)&set(c)):
 a=b[key];z=c[key]
 if a['text']!=z['text'] or a['reference_lines']!=z['reference_lines'] or a['reference_fonts']!=z['reference_fonts'] or a['reference_computed_style']!=z['reference_computed_style']:
  errors.append(f'reference changed for {key}');continue
 rows.append({'id':key[0],'width':key[1], 'baseline_matches':a['match'],'candidate_matches':z['match'],
              'baseline_rows':a['lines'],'candidate_rows':z['lines'],'browser_rows':z['reference_lines'],
              'baseline_breaks':a['candidate_breaks'],'candidate_breaks':z['candidate_breaks'],
              'oracle_breaks':z['exact_icu4c_boundaries'],'baseline_row_widths':a['row_widths'],
              'candidate_row_widths':z['row_widths'],'browser_row_widths':z['reference_row_widths']})
summary={'valid':not errors,'errors':errors,'baseline':str(args.baseline),'current':str(args.current),'pairs':len(rows),
         'baseline_matches':sum(x['baseline_matches'] for x in rows), 'candidate_matches':sum(x['candidate_matches'] for x in rows),
         'improved':[x for x in rows if not x['baseline_matches'] and x['candidate_matches']],
         'worsened':[x for x in rows if x['baseline_matches'] and not x['candidate_matches']],
         'both_mismatch_changed':[x for x in rows if not x['baseline_matches'] and not x['candidate_matches'] and x['baseline_rows']!=x['candidate_rows']]}
out=pathlib.Path(__file__).resolve().parent/'comparison-result.json';out.write_text(json.dumps(summary,ensure_ascii=False,indent=2)+'\n')
print(json.dumps({k:(len(v) if isinstance(v,list) and k not in ['errors'] else v) for k,v in summary.items()},ensure_ascii=False,indent=2))
if errors:raise SystemExit(1)
