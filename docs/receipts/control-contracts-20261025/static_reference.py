"""Static CSS fixture from the verified package; never the upstream JS example shell."""
import argparse
import hashlib
import json
from pathlib import Path

from playwright.sync_api import sync_playwright

parser = argparse.ArgumentParser()
parser.add_argument('out', type=Path)
parser.add_argument('--css', type=Path, required=True)
parser.add_argument('--size', choices=['large', 'medium', 'mini', 'xmini'], default='large')
parser.add_argument('--dark', action='store_true')
parser.add_argument('--care', action='store_true')
parser.add_argument('--brand')
args = parser.parse_args()
args.out.mkdir(parents=True, exist_ok=True)

rows = []
for overlay in [False, True]:
    cards = []
    for kind_index, kind in enumerate(['default', 'primary', 'warn']):
        buttons = []
        for state in range(3):
            classes = ['weui-btn', f'weui-btn_{kind}']
            if args.size != 'large':
                classes.append(f'weui-btn_{args.size}')
            if overlay:
                classes.append('weui-btn_overlay')
            if state == 1:
                classes.append('weui-btn_disabled')
            if state == 2:
                classes.append('weui-btn_loading')
            name = f'button-{kind_index}-{str(overlay).lower()}-{state}'
            spinner = '<i class="weui-mask-loading"></i>' if state == 2 else ''
            disabled = ' disabled' if state == 1 and not overlay else ''
            label = ['Active', 'Disabled', 'Loading'][state]
            buttons.append(f'<a role="button" href="javascript:" id="{name}" class="{" ".join(classes)}"{disabled}>'
                           f'{spinner}{label}</a>')
        cards.append(f'<div class="card {"overlay" if overlay else "normal"}">'
                     f'<p>Kind {kind_index}; overlay {str(overlay).lower()}</p>' + ''.join(buttons) + '</div>')
    rows.append('<section class="row">' + ''.join(cards) + '</section>')

html = f'''<!doctype html><meta charset="utf-8">
<link rel="stylesheet" href="{args.css.resolve().as_uri()}">
<style>
html,body {{ margin:0; min-height:100%; }}
body {{ box-sizing:border-box; padding:24px; font-size:14px; color:var(--weui-FG-0); background:var(--weui-BG-0); }}
.row {{ display:flex; gap:16px; margin:16px 0; }}
.card {{ flex:1; padding:16px; border-radius:8px; min-width:0; }}
.card p {{ margin:0 0 16px; }}
.normal {{ background:var(--weui-BG-2); }}
.overlay {{ background:var(--weui-BRAND); }}
.card .weui-btn {{ margin-bottom:16px; }}
</style><body {'data-weui-mode="care"' if args.care else ''}>
<p>STATIC VERIFIED CSS REFERENCE — not the upstream JS example shell</p>
{''.join(rows)}
'''
page_path = args.out / 'fixture.html'
page_path.write_text(html)
with sync_playwright() as playwright:
    browser = playwright.chromium.launch(executable_path='/usr/bin/chromium')
    context = browser.new_context(viewport={'width': 1100, 'height': 900},
                                  color_scheme='dark' if args.dark else 'light')
    page = context.new_page()
    page.goto(page_path.resolve().as_uri())
    if args.brand:
        page.evaluate('(brand) => document.body.style.setProperty("--weui-BRAND", "#" + brand)', args.brand)
    page.screenshot(path=str(args.out / 'static-css.png'))
    values = page.locator('.weui-btn').evaluate_all('''buttons => Object.fromEntries(buttons.map(button => {
      const style = getComputedStyle(button);
      const spinner = button.querySelector('.weui-mask-loading');
      return [button.id, Object.fromEntries([
        ...['fontFamily','fontSize','fontWeight','lineHeight','color','backgroundColor','borderRadius'].map(key => [key,style[key]]),
        ['width',button.getBoundingClientRect().width],['height',button.getBoundingClientRect().height],
        ['spinnerColor',spinner ? getComputedStyle(spinner).color : null]
      ])];
    }))''')
    (args.out / 'computed-styles.json').write_text(json.dumps(values, indent=2) + '\n')
    (args.out / 'provenance.json').write_text(json.dumps({
        'kind': 'static verified CSS reference, not upstream JavaScript example',
        'css': str(args.css), 'css_sha256': hashlib.sha256(args.css.read_bytes()).hexdigest(),
        'fixture_sha256': hashlib.sha256(page_path.read_bytes()).hexdigest(),
        'chromium_version': browser.version, 'size': args.size, 'dark': args.dark,
        'care': args.care, 'brand': args.brand,
    }, indent=2) + '\n')
    browser.close()
