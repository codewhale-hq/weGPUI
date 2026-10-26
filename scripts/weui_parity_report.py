"""Write an offline side-by-side/overlay viewer beside a completed paired capture set."""

import argparse
import json
from pathlib import Path
import re

HTML = r'''<!doctype html>
<html lang="en"><meta charset="utf-8"><meta name="viewport" content="width=device-width">
<title>WeUI native comparisons</title>
<style>
body{font:16px system-ui,sans-serif;margin:24px;background:#f4f4f4;color:#171717}main{max-width:1100px;margin:auto}
h1{font-size:24px}p{max-width:80ch;line-height:1.5}nav{display:flex;flex-wrap:wrap;gap:12px;align-items:end;margin:24px 0}
label{display:grid;gap:6px}select,button,input{font:inherit;min-height:40px}select{max-width:340px}button{padding:0 16px}
#images{display:flex;gap:16px;align-items:start}figure{margin:0;width:var(--width);max-width:100%;flex-shrink:1}
figcaption{height:28px;font-weight:600}img{display:block;width:100%;height:auto;background:white;box-shadow:0 0 0 1px #bbb}
#images.overlay{position:relative;width:var(--width);max-width:100%}#images.overlay figure{width:100%;flex-shrink:0}
#images.overlay #reference{position:absolute;inset:0}#images.overlay #reference figcaption{background:#f4f4f4}
output{display:block;margin:16px 0;line-height:1.5}pre{white-space:pre-wrap;overflow-wrap:anywhere;font-size:12px}
@media(max-width:600px){body{margin:12px}#images:not(.overlay){flex-direction:column}}
</style>
<main><h1>WeUI native comparisons</h1>
<p id="scope"></p><p>Native GPUI is on the left; pinned upstream WeUI is on the right.
These are captured scenes awaiting review. Pixel scores locate differences and do not establish parity.
Use the overlay to inspect alignment. Animation phases and glyph rasterisation can differ.</p>
<nav><label>Appearance<select id="mode"></select></label><label>Scene<select id="scene"></select></label>
<button id="previous" type="button">Previous</button><button id="next" type="button">Next</button>
<label><span>Overlay comparison</span><input id="overlay" type="checkbox"></label>
<label>Reference reveal<input id="reveal" type="range" min="0" max="100" value="50"></label></nav>
<output id="score" aria-live="polite"></output>
<div id="images"><figure><figcaption>Native GPUI</figcaption><img id="native-image" alt=""></figure>
<figure id="reference"><figcaption>Upstream WeUI</figcaption><img id="reference-image" alt=""></figure></div>
<details><summary>Capture provenance and scope</summary><pre id="provenance"></pre></details></main>
<script id="data" type="application/json">__DATA__</script>
<script>
const {records,manifest,summary}=JSON.parse(document.getElementById('data').textContent);
const mode=document.getElementById('mode'),scene=document.getElementById('scene');
const modes=[...new Set(records.map(r=>r.id.split('/')[0]))];
for(const value of modes)mode.add(new Option(value,value));
const names=[...new Set(records.map(r=>r.id.split('/')[1]))].sort();
for(const value of names)scene.add(new Option(value,value));
const lookup=new Map(records.map(r=>[r.id,r]));
const images=document.getElementById('images'),reference=document.getElementById('reference');
images.style.setProperty('--width',manifest.viewport[0]+'px');
document.getElementById('scope').textContent=`${summary.captured} of ${summary.expected} pairs captured. Language ${manifest.language}; viewport ${manifest.viewport.join(' × ')}; scale ${manifest.scale}. ${summary.full_matrix?'Complete appearance/state worklist.':'Selected subset.'}`;
document.getElementById('provenance').textContent=JSON.stringify({manifest,summary},null,2);
function overlay(){const enabled=document.getElementById('overlay').checked;images.classList.toggle('overlay',enabled);
reference.style.clipPath=enabled?`inset(0 ${100-Number(document.getElementById('reveal').value)}% 0 0)`:'';}
function show(){const id=mode.value+'/'+scene.value,r=lookup.get(id);
for(const [element,suffix,label] of [['native-image','gpui','Native GPUI'],['reference-image','weui','Upstream WeUI']]){
const image=document.getElementById(element);image.src=r&&r.status==='captured'?id+'-'+suffix+'.png':'';image.alt=label+' — '+id;}
document.getElementById('score').textContent=r&&r.status==='captured'?`Unreviewed: ${id}. Pixels differing by more than 8: ${r.pixels_over_8_percent}%; mean RGB difference: ${r.mean_rgb_difference}. These are diagnostic scores.`:`No successful pair for ${id}.`;
overlay();}
mode.addEventListener('change',show);scene.addEventListener('change',show);
for(const [id,step] of [['previous',-1],['next',1]])document.getElementById(id).addEventListener('click',()=>{scene.selectedIndex=(scene.selectedIndex+step+names.length)%names.length;show();});
document.getElementById('overlay').addEventListener('change',overlay);document.getElementById('reveal').addEventListener('input',overlay);show();
</script></html>
'''


def build(captures):
    manifest = json.loads((captures / "manifest.json").read_text())
    summary = json.loads((captures / "summary.json").read_text())
    records = [json.loads(line) for line in (captures / "results.jsonl").read_text().splitlines()]
    if len(records) != summary["expected"] or len({r["id"] for r in records}) != len(records):
        raise ValueError("Capture records are incomplete or duplicated")
    for record in records:
        if not re.fullmatch(r"[\w-]+/[\w-]+", record["id"]):
            raise ValueError("Unsafe capture identifier")
        if record["status"] == "captured":
            for suffix in ["gpui", "weui"]:
                if not (captures / (record["id"] + "-" + suffix + ".png")).is_file():
                    raise ValueError("A captured image is missing")
    data = json.dumps({"manifest": manifest, "summary": summary, "records": records}).replace("<", "\\u003c")
    path = captures / "review.html"
    path.write_text(HTML.replace("__DATA__", data))
    return path


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("captures", type=Path)
    args = parser.parse_args()
    try:
        print(build(args.captures))
    except (OSError, ValueError) as error:
        parser.exit(1, f"Cannot create comparison viewer: {error}\n")


if __name__ == "__main__":
    main()
