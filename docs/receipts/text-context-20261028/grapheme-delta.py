import collections,json,pathlib,regex
root=pathlib.Path(__file__).resolve().parent
rows=json.loads((root.parent/'candidate-fuzz.json').read_text())['cases'];counts=collections.Counter();examples=[]
for row in rows:
 text=row['text'];graphemes={len(text[:m.start()].encode()) for m in regex.finditer(r'\X',text)}|{len(text.encode())}
 base=set(row['baseline']);candidate=set(row['candidate']);base_bad=base-graphemes;candidate_bad=candidate-graphemes;added_bad=candidate_bad-base_bad;removed_bad=base_bad-candidate_bad
 counts['total']+=1;counts['baseline_cases_with_non_grapheme']+=bool(base_bad);counts['candidate_cases_with_non_grapheme']+=bool(candidate_bad);counts['cases_with_new_non_grapheme']+=bool(added_bad);counts['cases_with_removed_non_grapheme']+=bool(removed_bad);counts['new_non_grapheme_offsets']+=len(added_bad);counts['removed_non_grapheme_offsets']+=len(removed_bad)
 if added_bad and len(examples)<10:examples.append({'id':row['id'],'text':text,'added':sorted(added_bad)})
report={'description':'Host Python regex grapheme boundaries; may differ from ICU4X or product editor and do not prove input safety.','counts':dict(counts),'new_offset_examples':examples}
(root/'grapheme-delta-result.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n');print(json.dumps(report,ensure_ascii=False,indent=2))
