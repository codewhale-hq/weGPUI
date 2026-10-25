"""Compare static packaged article CSS geometry with native paint; no external SDK."""
import base64
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import time

from PIL import Image
from playwright.sync_api import sync_playwright

out = Path(sys.argv[1])
binary = sys.argv[2]
root = Path('/workspace/weGPUI')
package = Path('/workspace/.wegpui-cloud/target-198/weui-upstream/weui/package')
inputs = {
    'article.html': package / 'src/example/article/article.html',
    'article.less': package / 'src/style/widget/weui-page/weui-article.less',
    'weui.css': package / 'dist/style/weui.css',
    'example.css': package / 'dist/example/example.css',
    'pic_article.png': package / 'src/example/images/pic_article.png',
}
(out / 'reference-inputs.json').write_text(json.dumps({name: dict(path=str(path), sha256=hashlib.sha256(path.read_bytes()).hexdigest())
                                                     for name, path in inputs.items()}, indent=2))
assert inputs['pic_article.png'].read_bytes() == (root / 'assets/pic_article.png').read_bytes()
asset = Image.open(inputs['pic_article.png']).convert('RGB')
background = asset.getpixel((0, 0))
image_uri = 'data:image/png;base64,' + base64.b64encode(inputs['pic_article.png'].read_bytes()).decode()
article = inputs['article.html'].read_text().replace('class="page"', 'class="page article js_show"', 1)
article = article.replace('./images/pic_article.png', image_uri)
html = '<!doctype html><html><head><meta name="viewport" content="width=device-width,initial-scale=1"><style>'
html += inputs['weui.css'].read_text() + inputs['example.css'].read_text()
theme = os.environ.get('ARTICLE_THEME', 'light')
assert theme in ('light', 'dark'), theme
html += f'</style></head><body data-weui-theme="{theme}">' + article + '</body></html>'
widths = tuple(int(item) for item in os.environ.get('ARTICLE_WIDTHS', '375,480,700,900').split(','))
(out / 'configuration.json').write_text(json.dumps(dict(theme=theme, widths=widths, height=1800), indent=2))
metrics = {}
with sync_playwright() as p:
    browser = p.chromium.launch(executable_path='/usr/bin/chromium')
    for width in widths:
        context = browser.new_context(viewport=dict(width=width, height=1800), device_scale_factor=1, color_scheme=theme)
        context.route('http://**/*', lambda route: route.abort())
        context.route('https://**/*', lambda route: route.abort())
        page = context.new_page()
        page.set_content(html)
        page.locator('article img').first.wait_for()
        page.evaluate('Promise.all(Array.from(document.images).map(image => image.decode()))')
        page.evaluate('document.fonts.ready')
        metrics[str(width)] = page.evaluate('''() => {
          const rect = element => {const r=element.getBoundingClientRect(); return {x:r.x,y:r.y,width:r.width,height:r.height};};
          return {viewport:innerWidth, article:rect(document.querySelector('article')), images:Array.from(document.querySelectorAll('article img'), image=>({bounds:rect(image),naturalWidth:image.naturalWidth,naturalHeight:image.naturalHeight,maxWidth:getComputedStyle(image).maxWidth,height:getComputedStyle(image).height})), following_heading:rect(Array.from(document.querySelectorAll('h4')).find(h=>h.textContent==='列表示例'))};
        }''')
        page.screenshot(path=str(out / f'{width}-weui-static.png'))
        context.close()
    browser.close()
(out / 'weui-static-geometry.json').write_text(json.dumps(metrics, ensure_ascii=False, indent=2))
bus_log = (out / 'atspi-bus.log').open('w')
bus = subprocess.Popen(['/usr/libexec/at-spi-bus-launcher', '--launch-immediately'], stdout=bus_log, stderr=subprocess.STDOUT)
try:
    subprocess.run(['dbus-send', '--session', '--print-reply', '--dest=org.a11y.Bus', '/org/a11y/bus', 'org.freedesktop.DBus.Properties.Set', 'string:org.a11y.Status', 'string:IsEnabled', 'variant:boolean:true'], check=True)
    for width in widths:
        log = (out / f'{width}-native.log').open('w')
        app = subprocess.Popen([binary, '--page', 'article', '--theme', theme, '--size', f'{width}x1800', '--position', '0,0', '--reduce-motion'], stdout=log, stderr=subprocess.STDOUT)
        try:
            time.sleep(6)
            window = subprocess.check_output(['xdotool', 'search', '--onlyvisible', '--pid', str(app.pid), '--name', 'weGPUI'], text=True).splitlines()[0]
            subprocess.run(['import', '-window', window, str(out / f'{width}-native.png')], check=True)
            result = subprocess.check_output(['/usr/bin/python3', '-I', str(out / 'atspi_metrics.py')], text=True)
            (out / f'{width}-native-geometry.json').write_text(result)
            print('captured', width, flush=True)
        finally:
            app.terminate()
            app.wait(timeout=10)
            log.close()
finally:
    bus.terminate()
    bus.wait(timeout=10)
    bus_log.close()


def paint_boxes(path):
    image = Image.open(path).convert('RGB')
    boxes = []
    current = None
    for y in range(180, image.height):
        xs = [x for x in range(image.width) if image.getpixel((x, y)) == background]
        row = (min(xs), max(xs)) if len(xs) >= 100 else None
        if row is not None:
            if current is None or row != (current['x'], current['x'] + current['width'] - 1):
                if current is not None:
                    boxes.append(current)
                current = dict(x=row[0], y=y, width=row[1]-row[0]+1, height=1)
            else:
                current['height'] += 1
        elif current is not None:
            boxes.append(current)
            current = None
    if current is not None:
        boxes.append(current)
    return [box for box in boxes if box['height'] >= 20]


paint = {str(width): {side: paint_boxes(out / f'{width}-{side}.png') for side in ('native', 'weui-static')}
         for width in widths}
(out / 'paint-boxes.json').write_text(json.dumps(dict(background=background, widths=paint), indent=2))
print(json.dumps(paint, indent=2), flush=True)
