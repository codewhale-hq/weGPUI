from pathlib import Path
import json, re, sys, urllib.parse
package = Path(sys.argv[1])
root = Path(__file__).resolve().parent.parent
css = (package / 'dist/style/weui.css').read_text()
light, dark = {}, {}
for block in css.split('{'):
    if '}' not in block:
        selector = block
        continue
    body, next_selector = block.rsplit('}', 1)
    body = body.split('}')[0]
    values = dict(re.findall(r'(?m)^\s*--weui-([\w-]+):\s*([^;]+);', body))
    if values and 'care' not in selector and ':not(' not in selector:
        if "data-weui-theme='dark'" in selector:
            dark.update(values)
        else:
            light.update(values)
    selector = next_selector
for key, value in light.items():
    dark.setdefault(key, value)
(root / 'docs/tokens.json').write_text(json.dumps({'light': light, 'dark': dark}, indent=2)+'\n')
def field(k):
    return re.sub(r'([a-z])([A-Z])',r'\1_\2', k).replace('-', '_').lower()
def value(k, v, palette):
    if v.startswith('var('):
        return value(k, palette[v[11:-1]], palette)
    if v.startswith('#'):
        h = v[1:]
        if len(h) == 3: h = ''.join(c*2 for c in h)
        return f'rgb(0x{h}).into()'
    if v.startswith('rgba('):
        parts = [float(x) for x in v[5:-1].split(',')]
        return f'gpui::Rgba {{ r: {parts[0]/255:.9f}, g: {parts[1]/255:.9f}, b: {parts[2]/255:.9f}, a: {parts[3]} }}.into()'
    if v.startswith('calc('):
        return json.dumps(v)
    return str(float(v.removesuffix('px')))
def typ(v):
    if v.startswith(('#','rgba(','var(')): return 'Hsla'
    if v.startswith('calc('): return "&'static str"
    return 'f32'
lines=['use gpui::{App, Global, Hsla, Window, WindowAppearance, rgb};','', '#[derive(Clone, Debug)]','pub struct Theme {','    pub is_dark: bool,','    pub mode: ThemeMode,']
lines += [f'    pub {field(k)}: {typ(v)},' for k,v in light.items()]
lines += ['}', '', '#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]','pub enum ThemeMode {', '    #[default]', '    System,', '    Light,', '    Dark,', '}', '', 'impl Global for Theme {}', '', 'impl Default for Theme { fn default() -> Self { Self::light() } }', '', 'impl Theme {']
for name, palette in [('light',light),('dark',dark)]:
    lines += [f'    pub fn {name}() -> Self {{', '        Self {', f'            is_dark: {str(name=="dark").lower()},',f'            mode: ThemeMode::{name.title()},']
    lines += [f'            {field(k)}: {value(k,v,palette)},' for k,v in palette.items()]
    lines += ['        }', '    }']
lines += ['''    pub fn with_brand(mut self, brand: impl Into<Hsla>) -> Self { self.brand = brand.into(); self }
    pub fn get(cx: &App) -> &Self { cx.global::<Self>() }
    pub fn set_mode(mode: ThemeMode, cx: &mut App) {
        let brand = Self::get(cx).brand;
        let dark = matches!(cx.window_appearance(), WindowAppearance::Dark | WindowAppearance::VibrantDark);
        let mut theme = match mode { ThemeMode::Dark => Self::dark(), ThemeMode::Light => Self::light(), ThemeMode::System if dark => Self::dark(), _ => Self::light() };
        theme.mode = mode;
        theme.brand = brand;
        cx.set_global(theme);
        cx.refresh_windows();
    }
    pub fn init(cx: &mut App) { cx.set_global(Self::light()); Self::set_mode(ThemeMode::System, cx); }
    pub fn observe(window: &mut Window) {
        window.observe_window_appearance(|_, cx| {
            if Self::get(cx).mode == ThemeMode::System { Self::set_mode(ThemeMode::System, cx); }
        }).detach();
    }
}''']
(root / 'src/theme.rs').write_text('\n'.join(lines)+'\n')
assets = {}
for block in css.split('}'):
    if 'mask-image:' not in block or 'data:image/svg+xml,' not in block: continue
    names = re.findall(r'\.weui-icon-([\w-]+)',block.split('{')[0])
    encoded = None
    for part in block.split('url(')[1:]:
        part=part.lstrip()
        quote=part[0] if part[0] in ('"', "'") else None
        uri=part[1:].split(quote)[0] if quote else part.split(')')[0]
        if uri.startswith('data:image/svg+xml,'):
            encoded=uri.split(',',1)[1]
            break
    if not names or encoded is None: continue
    key = 'icon-'+names[0]+'.svg'
    if key not in assets:
        assets[key]=urllib.parse.unquote(encoded)
for k,v in assets.items(): (root/'assets'/k).write_text(v)
for p in (package/'src/example/images').glob('*'):
    if p.is_file(): (root/'assets'/p.name).write_bytes(p.read_bytes())
paths=sorted(p.name for p in (root/'assets').iterdir() if p.is_file())
rs=['use std::borrow::Cow;','use gpui::{AssetSource, SharedString};','pub struct Assets;','impl AssetSource for Assets {','    fn load(&self, path: &str) -> gpui::Result<Option<Cow<\'static, [u8]>>> {','        Ok(match path {']
rs += [f'            "{p}" => Some(Cow::Borrowed(include_bytes!("../assets/{p}").as_slice())),' for p in paths]
rs += ['            _ => None,','        })','    }','    fn list(&self, _: &str) -> gpui::Result<Vec<SharedString>> { Ok(vec![]) }','}']
(root/'src/assets.rs').write_text('\n'.join(rs)+'\n')
print(f'{len(light)} light variables, {len(dark)} dark variables, {len(assets)} WeUI SVG icons extracted')
