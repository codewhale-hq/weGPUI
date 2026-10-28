'use strict';
const data = window.WEGPUI;
const repo = 'https://github.com/codewhale-hq/weGPUI';
const app = document.querySelector('#app');
const escapeHTML = value => String(value).replace(/[&<>"']/g, c => ({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]));
const link = path => `${repo}/blob/${data.sourceCommit}/${path}`;
const code = text => `<div class="code-block"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>${escapeHTML(text)}</code></pre></div>`;
const eyebrow = text => `<p class="eyebrow">${text}</p>`;
const docsLink = (path, label) => `<a href="${link(path)}">${label} <span aria-hidden="true">↗</span></a>`;
const apiLink = (module, label) => `<a class="module-link" href="${data.apiIncluded ? `api/wegpui/${module}/index.html` : link('src/' + module + '.rs')}">${escapeHTML(label || module.replaceAll('/', '::'))} <span aria-hidden="true">↗</span></a>`;
const titlePage = (kicker, title, intro) => `${eyebrow(kicker)}<h1>${title}</h1><p class="page-intro">${intro}</p>`;
let catalogQuery = '';
let compare = false;
let restoreFocus = null;
let lastPage = '';
let demoPreferences = {theme: 'light', care: false, brand: '', width: '375'};
try {
  const saved = JSON.parse(localStorage.getItem('wegpui.demo') || 'null');
  if (saved && typeof saved === 'object') demoPreferences = {...demoPreferences,
    theme: saved.theme === 'dark' ? 'dark' : 'light', care: saved.care === true,
    brand: typeof saved.brand === 'string' && /^[0-9a-f]{6}$/i.test(saved.brand) ? saved.brand : '',
    width: saved.width === '768' ? '768' : '375'};
} catch {}

function rememberDemo(config, width) {
  demoPreferences = {theme: config.theme, care: config.care, brand: config.brand,
    width: width === '768' ? '768' : '375'};
  try { localStorage.setItem('wegpui.demo', JSON.stringify(demoPreferences)); } catch {}
}

function demoConfig(info = current()) {
  const brand = (info.params.has('brand') ? info.params.get('brand') : demoPreferences.brand).replace(/^#/, '');
  const route = info.route || 'form_primary';
  const states = data.playground?.states?.[route] || data.routes.find(item => item.id === route)?.states || ['default'];
  const requestedState = info.params.get('state') || 'default';
  return {
    route,
    state: states.includes(requestedState) ? requestedState : 'default',
    theme: info.params.has('theme') ? (info.params.get('theme') === 'dark' ? 'dark' : 'light') : demoPreferences.theme,
    care: info.params.has('care') ? info.params.get('care') === '1' : demoPreferences.care,
    brand: /^[0-9a-f]{6}$/i.test(brand) ? brand : '',
  };
}

function demoURL(config) {
  const url = new URL('playground/index.html', document.baseURI);
  const params = new URLSearchParams({theme: config.theme, state: config.state,
    care: config.care ? '1' : '0', brand: config.brand});
  url.hash = `${config.route}?${params}`;
  return url.href;
}

function liveFrame(config, id, className = '') {
  return `<iframe id="${id}" class="live-frame ${className}" data-playground data-config="${escapeHTML(JSON.stringify(config))}" src="${escapeHTML(demoURL(config))}" title="${escapeHTML(config.route.replaceAll('_', ' '))}: interactive WeUI components" loading="${id === 'hero-demo' ? 'eager' : 'lazy'}" allow="clipboard-write"></iframe>`;
}

function viewTabs(info, selected) {
  const params = new URLSearchParams(info.params);
  params.set('theme', demoConfig(info).theme);
  const url = view => { params.set('view', view); return `#components/${info.route}?${params}`; };
  return `<nav class="preview-modes" aria-label="Component preview"><a href="${escapeHTML(url('live'))}" ${selected === 'live' ? 'aria-current="page"' : ''}>Live browser demo</a><a href="${escapeHTML(url('native'))}" ${selected === 'native' ? 'aria-current="page"' : ''}>Native Rust captures</a></nav>`;
}

function catalogSidebar() {
  return `<aside class="catalog" aria-label="Component catalog"><div class="catalog-top"><h2>Components <span>${data.routes.length}</span></h2><label class="search-box"><span class="sr-only">Search components</span><span aria-hidden="true">⌕</span><input id="catalog-search" type="search" placeholder="Search components…" value="${escapeHTML(catalogQuery)}" autocomplete="off"></label><button class="mobile-catalog-toggle" aria-expanded="false" aria-controls="catalog-list">Browse all components <span aria-hidden="true">⌄</span></button></div><div id="catalog-list">${catalogList(catalogQuery)}</div></aside>`;
}

function webExample(family) {
  const examples = {
    button: '<button type="button" class="weui-btn weui-btn_primary">Continue</button>',
    input: '<input class="weui-input" aria-label="Your name" placeholder="Your name">',
    form: '<label class="weui-cell">\n  <span class="weui-cell__hd">Name</span>\n  <input class="weui-input" placeholder="Your name">\n</label>',
    dialog: "weui.confirm('Save these changes?', {\n  onConfirm() { /* Save your data here. */ }\n});",
    toast: "weui.toast('Saved', { duration: 1500 });",
    picker: "weui.picker([\n  { label: 'First choice', value: 1 },\n  { label: 'Second choice', value: 2 }\n], { onConfirm(items) { console.log(items); } });",
    loading: "const loading = weui.loading('Loading');\n// Call loading.hide() when your task finishes.",
    slider: '<input type="range" min="0" max="100" value="50" aria-label="Volume">',
  };
  return examples[family] || '<link rel="stylesheet" href="weui.css">\n<script src="weui.min.js"><\/script>';
}

function componentPage(info) {
  if (info.params.get('view') === 'native') return nativeComponentPage(info);
  const r = data.routes.find(route => route.id === info.route);
  if (!r) return nativeComponentPage(info);
  const config = demoConfig(info);
  const states = data.playground?.states?.[r.id] || r.states;
  if (!states.includes(config.state)) config.state = 'default';
  const width = info.params.get('width') || demoPreferences.width;
  const wide = width === '768';
  rememberDemo(config, width);
  const color = config.brand ? `#${config.brand}` : data.tokens[config.care ? `care_${config.theme}` : config.theme]['BRAND-100'];
  return `<div class="explorer">${catalogSidebar()}<main id="main" tabindex="-1" class="component-main live-component">
    <div class="component-header">${eyebrow(`${r.category} / ${r.familyLabel}`)}<h1>${escapeHTML(r.title)}</h1><p class="page-intro">${escapeHTML(r.description.replace(/^Native /, ''))}</p></div>
    ${viewTabs(info, 'live')}
    <div class="preview-toolbar live-toolbar"><span class="preview-label"><span class="status-dot"></span>Live in your browser</span>
      <label>Example <select id="state-select">${states.map(state => `<option value="${state}" ${state === config.state ? 'selected' : ''}>${state === 'default' ? 'Default' : escapeHTML(state)}</option>`).join('')}</select></label>
      <fieldset class="segmented"><legend class="sr-only">Demo theme</legend>${['light', 'dark'].map(theme => `<button type="button" data-theme="${theme}" aria-pressed="${theme === config.theme}">${theme === 'light' ? 'Light' : 'Dark'}</button>`).join('')}</fieldset>
      <label class="care-control"><input id="care-toggle" type="checkbox" ${config.care ? 'checked' : ''}> Care mode</label>
      <label class="brand-control">Brand <input id="brand-color" type="color" value="${escapeHTML(color)}" aria-label="Demo brand colour"></label><button class="quiet-button" id="brand-reset" type="button">Reset</button>
    </div>
    <div class="live-workspace"><section class="live-stage" aria-label="Interactive preview"><div class="live-stage-bar"><label>Viewport <select id="viewport-select"><option value="375" ${!wide ? 'selected' : ''}>Phone</option><option value="768" ${wide ? 'selected' : ''}>Wide · 768px</option></select></label><a id="open-demo" href="${escapeHTML(demoURL(config))}" target="_blank" rel="noopener">Open full page ↗</a></div><div class="live-viewport"><div class="live-frame-shell ${wide ? 'is-wide' : ''}" style="--demo-width:${wide ? 768 : 375}px">${liveFrame(config, 'live-demo')}</div></div><p id="demo-status" class="demo-status" role="status">Loading interactive example…</p></section>
    <aside class="component-details"><div><h2>Try the real controls</h2><p>Click, type, change a value or open an overlay. These are browser-rendered WeUI components.</p><p class="version-note">WeUI 2.6.26 · JavaScript 1.2.17</p></div><div><h2>Use it on the web</h2>${code(webExample(r.family))}<a class="source-link" href="playground/source/${r.id}.html.txt" target="_blank" rel="noopener">View complete HTML source ↗</a></div><div><h2>Native Rust companion</h2><p>The repository also implements these component families with GPUI for desktop applications.</p><a class="source-link" href="${link(r.source)}">View native implementation ↗</a><div class="module-list">${r.modules.map(module => apiLink(module)).join('')}</div></div></aside></div>
    <div class="component-bottom"><span>Keyboard-friendly · No sign-in required</span><a href="#get-started">Build with WeUI →</a></div>
  </main></div>`;
}

function overview() {
  const hero = {route: 'form_primary', state: 'default', theme: 'light', care: false, brand: ''};
  const cards = [
    ['button_default', 'Actions, with intention.', 'Buttons, choices and feedback.', 'icon_nav_form.png'],
    ['form_primary', 'Inputs that belong.', 'Real fields, validation and local previews.', 'icon_nav_layout.png'],
    ['picker', 'Choices, made clear.', 'Pickers, dialogs and focused decisions.', 'icon_nav_feedback.png'],
  ];
  return `<main id="main" tabindex="-1" class="overview web-overview">
    <section class="hero"><div class="hero-copy">${eyebrow('WeUI · Live in your browser')}<h1>Familiar design.<br><em>Ready to try.</em></h1><p>Buttons, forms, navigation and overlays. Explore the WeUI component gallery, make it your own, and open the code behind every example.</p><div class="hero-actions"><a class="button primary" href="#components/button_default">Explore components <span aria-hidden="true">→</span></a><a class="button secondary" href="#get-started">Start building</a></div><p class="hero-footnote">Open source · WeUI 2.6.26 · No sign-in</p></div><div class="hero-visual live-hero"><div class="visual-caption"><span class="status-dot"></span>Try the form <span class="mono">LIVE / BROWSER</span></div><div class="hero-live-window">${liveFrame(hero, 'hero-demo')}</div><div class="visual-tag"><span class="mini-code">↗</span><span>Click. Type. Explore.<small>These controls are interactive.</small></span></div></div></section>
    <section class="feature-row" aria-label="About the gallery"><div><h2>Live controls</h2><p>Open a dialog, change a choice, or try a complete form. The examples run right here.</p></div><div><h2>Make it yours</h2><p>Switch light and dark, try care mode, and explore your own brand colour.</p></div><div><h2>Open by design</h2><p>Browse the HTML and JavaScript, or explore the companion native Rust library.</p></div></section>
    <section class="home-catalog"><div class="section-heading"><div>${eyebrow(`${data.routes.length} examples to explore`)}<h2>Small pieces.<br>Whole experiences.</h2></div><p>From a single action to a complete flow. Browse the catalog and try each component on phone and wide layouts.</p></div><div class="showcase-grid live-showcase">${cards.map(([route, title, description, icon]) => `<a class="showcase-card live-showcase-card" href="#components/${route}"><div class="live-card-art"><img src="playground/vendor/images/${icon}" alt="" width="80" height="80"><span aria-hidden="true">↗</span></div><div>${title}</div><p>${description}</p><small>Open live example →</small></a>`).join('')}</div></section>
    <section class="build-banner"><div>${eyebrow('A small starting point')}<h2>Your next screen<br>starts here.</h2><p>Use WeUI styles with familiar HTML and add the interactions your application needs.</p><a class="text-link" href="#get-started">Read the web guide →</a></div>${code('<link rel="stylesheet" href="weui.css">\n\n<button type="button" class="weui-btn weui-btn_primary">\n  Continue\n</button>')}</section>
    <section class="honest-note"><span class="outlined-icon" aria-hidden="true">◎</span><div><h2>Web today. Native alongside.</h2><p>This gallery runs WeUI in your browser. The repository also contains reusable native Rust components built with GPUI.</p></div><a href="#native">Explore the native library →</a></section>
  </main>`;
}

function nativeProgress() {
  const evidence = data.nativeEvidence;
  return `<section class="honest-note"><div><h2>Native progress, with evidence.</h2><p>${evidence.capturedPairs.toLocaleString()} recorded native/reference pairs cover ${evidence.scenes} scenes in eight appearances. Shared bidi geometry, styled RTL text, locale-aware dates and shaped ellipsis clamping are implemented. Dictionary line breaking covers Thai, Lao, Khmer and Myanmar; an expanded four-language comparison matches 189 of 192 browser row sequences; three Khmer cases remain different. The iOS simulator app links and launches; a digit keypad builds but still needs runtime verification. Full desktop parity is still in progress.</p><p>Complete recorded batch: <code>${evidence.sourceCommit.slice(0,7)}</code>. Each image identifies its own source. Windows verification is limited to build/tests.</p>${docsLink('docs/receipts/chromium-dictionary-tel-20261028/README.md', 'Latest dictionary and telephone input work')}</div>${docsLink(evidence.receipt, 'Read the complete comparison report')}</section>`;
}

function nativeOverview() {
  const features = [['Native by construction','Rust components rendered by GPUI. No embedded browser is required for the desktop gallery.'],['A familiar design language','WeUI 2.6.26 layouts, tokens and assets, adapted to native application composition.'],['Built to inspect','Explore the source, reproduce a gallery state, and see exactly what has been verified.']];
  return `<main id="main" tabindex="-1" class="overview">
    <section class="hero"><div class="hero-copy">${eyebrow('WeUI × Rust × GPUI')}<h1>Familiar design.<br><em>Native feel.</em></h1><p>A WeUI component library for native Rust applications. Forms, navigation, overlays and the details in between—ready to explore in GPUI.</p><div class="hero-actions"><a class="button primary" href="#components/button_default">Explore the components <span aria-hidden="true">→</span></a><a class="button secondary" href="#get-started">Build your first screen</a></div><p class="hero-footnote">Open source · Desktop-first · Developer preview</p></div><div class="hero-visual" aria-label="Native gallery screenshot"><div class="visual-caption"><span class="status-dot"></span>Captured from the native gallery <span class="mono">01 / FORM</span></div><div class="hero-capture"><img src="captures/form_primary-light-gpui.png" alt="Native WeUI form with title, fields and a green primary action" width="750" height="1624"></div><div class="visual-tag"><span class="mini-code">&lt;/&gt;</span><span>Rust all the way down.<small>Composed with GPUI elements.</small></span></div></div></section>
    <section class="feature-row" aria-label="About the library">${features.map(([title,text])=>`<div><h2>${title}</h2><p>${text}</p></div>`).join('')}</section>
    <section class="home-catalog"><div class="section-heading"><div>${eyebrow('A complete gallery inventory')}<h2>Small pieces.<br>Whole experiences.</h2></div><p>Browse ${data.routes.length} example routes, from a single button to a complete form. Route coverage is implemented; release acceptance is still in progress.</p></div><div class="showcase-grid">${[['button_default','Actions, with intention.'],['form_select','Inputs that belong.'],['picker','Choices, made clear.']].map(([id,caption])=>{const r=data.routes.find(x=>x.id===id);const state=id==='picker'?'single':'default';return `<a class="showcase-card" href="#components/${id}${state==='default'?'':'?state='+state}"><div class="showcase-image"><img src="${r.images[state].light.gpui}" width="750" height="1624" loading="lazy" alt="${escapeHTML(r.title)} native gallery capture"></div><div><span>${caption}</span><span aria-hidden="true">↗</span></div><small>${r.familyLabel}</small></a>`;}).join('')}</div></section>
    <section class="build-banner"><div>${eyebrow('From gallery to application')}<h2>Your next screen<br>starts here.</h2><p>Run the native gallery, inspect its source, and adapt a complete example to your app.</p><a class="text-link" href="#get-started">Read the getting-started guide →</a></div>${code('git clone https://github.com/codewhale-hq/weGPUI.git\ncd weGPUI\ncargo run --example gallery')}</section>
    ${nativeProgress()}
    <section class="honest-note"><span class="outlined-icon" aria-hidden="true">◎</span><div><h2>Open source. Open about the gaps.</h2><p>We’re preparing a verified desktop release. Mobile is experimental, and no platform has completed every acceptance gate yet.</p></div><a href="#platforms">See platform status →</a></section>
  </main>`;
}
function catalogList(query) {
  const filtered = data.routes.filter(r=>`${r.title} ${r.id} ${r.category} ${r.description}`.toLowerCase().includes(query.toLowerCase()));
  const selected = current().route;
  const mode = current().params.get('view') === 'native' ? '?view=native' : '';
  const categories = [...new Set(filtered.map(r=>r.category))];
  return `<p class="result-count" aria-live="polite">${filtered.length} of ${data.routes.length} routes</p>${filtered.length ? categories.map(category=>`<section class="nav-group"><h3>${category}</h3>${filtered.filter(r=>r.category===category).map(r=>`<a href="#components/${r.id}${mode}" ${selected===r.id?'aria-current="page"':''}>${escapeHTML(r.title)}<span aria-hidden="true">${r.states.length>1?r.states.length:''}</span></a>`).join('')}</section>`).join('') : '<p class="empty-results">No matching components. Try “form”, “dialog” or “navigation”.</p>'}`;
}
function current() {
  const [path, query=''] = location.hash.slice(1).split('?');
  const parts=(path || 'overview').split('/');
  return {page:parts[0],route:parts[1],params:new URLSearchParams(query)};
}
function nativeComponentPage(info) {
  const r = data.routes.find(r=>r.id===info.route);
  if (!r) return `<main id="main" tabindex="-1" class="document">${titlePage('Not found','That component isn’t in this gallery.','Try the component catalog or return to the overview.')}<a class="button primary" href="#components/button_default">Browse components</a></main>`;
  const theme=info.params.get('theme')==='dark'?'dark':'light';
  const state=r.states.includes(info.params.get('state'))?info.params.get('state'):'default';
  const capture = r.captureEvidence?.[state]?.[theme] || {commit:data.captureCommit, platform:'macOS', viewport:[375,812], receipt:'docs/PARITY.md'};
  const captureLabel = `${capture.platform} capture from ${capture.commit.slice(0,7)}`;
  const command=`cargo run --example gallery -- --page ${r.id}${state==='default'?'':' --state '+state} --theme ${theme}`;
  return `<div class="explorer"><aside class="catalog" aria-label="Component catalog"><div class="catalog-top"><h2>Components <span>${data.routes.length}</span></h2><label class="search-box"><span class="sr-only">Search components</span><span aria-hidden="true">⌕</span><input id="catalog-search" type="search" placeholder="Search components…" value="${escapeHTML(catalogQuery)}" autocomplete="off"></label><button class="mobile-catalog-toggle" aria-expanded="false" aria-controls="catalog-list">Browse all components <span aria-hidden="true">⌄</span></button></div><div id="catalog-list">${catalogList(catalogQuery)}</div></aside>
    <main id="main" tabindex="-1" class="component-main"><div class="component-header">${eyebrow(r.category+' / '+r.familyLabel)}<h1>${escapeHTML(r.title)}</h1><p class="page-intro">${escapeHTML(r.description)}</p></div>
    <div class="preview-toolbar"><div class="preview-label"><span class="status-dot"></span>Native capture</div><label>State <select id="state-select">${r.states.map(s=>`<option value="${s}" ${s===state?'selected':''}>${s==='default'?'Default':s}</option>`).join('')}</select></label><fieldset class="segmented"><legend class="sr-only">Capture theme</legend>${['light','dark'].map(t=>`<button type="button" data-theme="${t}" aria-pressed="${t===theme}">${t==='light'?'Light':'Dark'}</button>`).join('')}</fieldset><button type="button" class="compare-toggle" aria-pressed="${compare}">${compare?'Hide comparison':'Compare WeUI'}</button></div>
    <div class="component-workspace ${compare?'comparing':''}"><div class="preview-stage"><figure><figcaption><span>weGPUI</span><span class="mono">RECORDED / ${escapeHTML(capture.platform)}</span></figcaption><div class="capture-window"><img id="native-capture" src="${r.images[state][theme].gpui}" width="750" height="1624" alt="${escapeHTML(r.title)}, ${state} state, ${theme} theme: ${escapeHTML(captureLabel)}"></div></figure>${compare?`<figure><figcaption><span>WeUI reference</span><span class="mono">BROWSER</span></figcaption><div class="capture-window"><img src="${r.images[state][theme].weui}" width="750" height="1624" alt="Upstream WeUI reference for ${escapeHTML(r.title)}, ${state}, ${theme}"></div></figure>`:''}</div>
    <aside class="component-details"><div><h2>Try it natively</h2><p>The preview is a recorded screenshot. Run this command for the interactive desktop example.</p>${code(command)}</div><div><h2>Go to the source</h2><a class="source-link" href="${link(r.source)}">Gallery implementation <span aria-hidden="true">↗</span></a><div class="module-list">${r.modules.length?r.modules.map(m=>apiLink(m)).join(''):docsLink(r.source,'Gallery-specific implementation')}</div></div><div class="provenance"><h2>About this capture</h2><p>${escapeHTML(capture.platform)} capture from <code>${capture.commit.slice(0,7)}</code>, at a ${capture.viewport.join(' × ')} logical-pixel viewport. It is a recorded comparison, not a live control or accessibility acceptance.</p>${docsLink(capture.receipt,'Capture method and differences')}<br>${docsLink('docs/STATUS.md','Known component gaps')}</div></aside></div>
    <div class="component-bottom"><span class="mono">${escapeHTML(r.id)}</span><a href="#get-started">Set up your application →</a></div></main></div>`;
}
function gettingStarted() {
  return `<main id="main" tabindex="-1" class="document">${titlePage('Get started on the web', 'A familiar starting point.', 'Use the same WeUI styles and browser components you can try in this gallery.')}<div class="doc-layout"><article><section><h2><span class="step-number">01</span>Add the styles</h2><p>Download the pinned stylesheet and link it from your page.</p>${code('<link rel="stylesheet" href="weui.css">')}<a class="text-link" href="playground/vendor/weui.css" download>Download WeUI 2.6.26 CSS ↓</a></section><section><h2><span class="step-number">02</span>Start with HTML</h2><p>Use semantic controls and give every input a visible label. Browse an example for its complete markup.</p>${code('<button type="button" class="weui-btn weui-btn_primary">\n  Continue\n</button>\n\n<label>\n  Your name\n  <input class="weui-input" name="name" autocomplete="name">\n</label>')}<a class="text-link" href="#components/form_primary">Try a complete form →</a></section><section><h2><span class="step-number">03</span>Add interactions</h2><p>The optional JavaScript helpers provide dialogs, pickers, toasts and more. Your application owns its data and callbacks.</p>${code('<script src="weui.min.js"></script>\n<script>\n  weui.confirm("Save these changes?", {\n    onConfirm() { console.log("Confirmed"); }\n  });\n</script>')}<a class="text-link" href="playground/vendor/weui.min.js" download>Download weui.js 1.2.17 ↓</a></section><section><h2><span class="step-number">04</span>Run this gallery locally</h2>${code('git clone https://github.com/codewhale-hq/weGPUI.git\ncd weGPUI\npython3 website/build.py\npython3 -m http.server 4173 --directory website/dist')}<p>The website is a static build and works on GitHub Pages or Cloudflare Pages.</p></section></article><aside class="reading-note"><h2>Explore first</h2><p>Every example opens in an isolated live preview. Change the theme, try care mode, and inspect the full source.</p><a href="#components/button_default">Open the component catalog →</a><hr><h2>Local file previews</h2><p>Upload examples preview selected files in your browser. They do not send files to a server.</p><hr><h2>Native Rust</h2><p>For desktop applications, explore the GPUI implementation and its current platform evidence.</p><a href="#native-guide">Native setup guide →</a><hr><h2>Attribution</h2><p>WeUI is designed by Tencent. This independent gallery preserves its open-source license and notices.</p><a href="licenses/LICENSE-WEUI">Read the WeUI license →</a></aside></div></main>`;
}

function nativeGettingStarted() {
  const install=`[dependencies]\nwegpui = { git = "${repo}", rev = "${data.sourceCommit}" }\ngpui = { package = "gpui-pre", version = "=${data.gpuiVersion}" }\ngpui_platform = { package = "gpui-pre-platform", version = "=${data.gpuiVersion}", features = ["font-kit", "wayland", "x11"] }`;
  const patches=Object.entries(data.compatibilityPatches || {}).map(([name,path])=>`${name} = { path = "../weGPUI/${path}" }`).join('\n');
  const patchGuide=patches?`<h3>Match the approved compatibility sources</h3><p>The current native library requires the patched GPUI API to compile. Cargo patches in a dependency are not inherited. Keep a checkout at the same revision alongside your application and add these entries to the application's workspace-root Cargo.toml. Adjust the paths to your checkout.</p>${code('[patch.crates-io]\n'+patches)}<p>The package versions stay unchanged. These approved local copies carry the sampled Linux accessibility fixes.</p>${docsLink('vendor/README.md','Compatibility changes and limits')}`:'';
  const previewNote=data.sourceDirty?'<p class="provenance">This local preview includes uncommitted changes. Source links and the Git dependency revision identify the last commit; rebuild from committed source before sharing this setup.</p>':'';
  return `<main id="main" tabindex="-1" class="document">${titlePage('Getting started','From clone to native window.','Start with a complete example, then bring the components into your own GPUI application.')}<div class="doc-layout"><article><section><h2><span class="step-number">01</span>Set up your environment</h2><p>The manifest declares Rust ${data.rustVersion} or newer; Linux stable and the declared minimum version pass the recorded build checks. macOS requires Xcode tooling; Windows needs the MSVC build tools and Windows SDK. Linux needs the system libraries below.</p><details><summary>Debian / Ubuntu system dependencies</summary>${code('sudo apt-get install libxkbcommon-dev libxkbcommon-x11-dev \\\n  libwayland-dev libvulkan-dev libfontconfig-dev libfreetype-dev \\\n  libx11-xcb-dev libxcb1-dev libssl-dev pkg-config clang')}</details></section><section><h2><span class="step-number">02</span>Run the gallery</h2>${code('git clone '+repo+'.git\ncd weGPUI\ncargo run --example gallery')}<p>Or run <code>cargo run --example minimal</code> for a small application with a dialog, state updates and focus handling.</p></section><section><h2><span class="step-number">03</span>Add it to your app</h2><p>There is no crates.io release yet. Use a Git revision with matching exact GPUI pins. The example below includes Linux windowing features.</p>${code(install)}${previewNote}${patchGuide}<p>Call <code>wegpui::init(cx)</code>, attach <code>wegpui::Assets</code>, and use <code>Theme</code> and the components inside your GPUI views.</p><a class="button secondary" href="${link('examples/minimal.rs')}">Read the complete minimal application ↗</a></section><section><h2><span class="step-number">04</span>Build with the reference nearby</h2><div class="doc-links">${data.apiIncluded?'<a href="api/wegpui/index.html">API reference ↗</a>':docsLink('src/lib.rs','Library documentation')}${docsLink('CONTRIBUTING.md','Contributor guide')}${docsLink('docs/UPSTREAM-COVERAGE.md','Upstream example mapping')}${docsLink('docs/STATUS.md','Known differences')}</div></section></article><aside class="reading-note"><h2>Before you ship</h2><p>The project is a developer preview. Desktop runtime and screen-reader acceptance are not complete. Mobile remains experimental.</p><a href="#platforms">Review platform status →</a><hr><h2>Why exact GPUI pins?</h2><p>These components depend on GPUI APIs that evolve. Keep the application and library on the same GPUI snapshot.</p><hr><h2>A library, not a backend</h2><p>Your application owns business logic, data, persistence and upload transport. The gallery demonstrates component composition.</p></aside></div></main>`;
}
function themes(info) {
  const palette = Object.hasOwn(data.tokens, info.params.get('palette')) ? info.params.get('palette') : 'light';
  const tokens = data.tokens[palette];
  const config = {route: 'form_checkbox', state: 'default', theme: palette.endsWith('dark') ? 'dark' : 'light', care: palette.startsWith('care_'), brand: ''};
  rememberDemo(config, demoPreferences.width);
  const swatches = ['BRAND-100', 'BG-0', 'BG-1', 'BG-2', 'FG-0', 'FG-1', 'RED-100', 'BLUE-100'].filter(key => tokens[key]);
  return `<main id="main" tabindex="-1" class="document">${titlePage('Themes and tokens', 'Make it feel like yours.', 'Explore the pinned WeUI palettes, then try light, dark, care contrast and your own brand in any component.')}<div class="theme-panel"><label for="palette-select">Palette</label><select id="palette-select">${Object.keys(data.tokens).map(key => `<option value="${key}" ${key === palette ? 'selected' : ''}>${key.replaceAll('_', ' ')}</option>`).join('')}</select><p>Original colour values from WeUI 2.6.26. Care mode increases contrast.</p><div class="swatches">${swatches.map(key => `<div><div class="swatch" style="background:${escapeHTML(tokens[key])}"></div><strong>${key}</strong><code>${escapeHTML(tokens[key])}</code></div>`).join('')}</div></div><div class="two-column"><section><h2>Use the palette</h2>${code(`document.body.setAttribute('data-weui-theme', '${config.theme}');\ndocument.body.setAttribute('data-weui-mode', '${config.care ? 'care' : 'normal'}');`)}<p>Use the controls above any live example to change its brand colour. Reset returns to the selected palette.</p><a class="text-link" href="#components/button_default">Try a themed component →</a><h3>Keep the source nearby</h3><p>All ${Object.keys(tokens).length} generated tokens and their original CSS are available to inspect.</p>${docsLink('docs/tokens.json', 'Browse the token source')}<p><a href="playground/vendor/weui.css">View the pinned stylesheet ↗</a></p></section><section class="theme-live-preview"><h2>Live palette preview</h2><div class="live-frame-shell">${liveFrame(config, 'theme-demo')}</div></section></div></main>`;
}

function nativeThemes(info) {
  const palette=Object.hasOwn(data.tokens,info.params.get('palette'))?info.params.get('palette'):'light';
  const tokens=data.tokens[palette];
  const swatches=['BRAND-100','BG-0','BG-1','BG-2','FG-0','FG-1','RED-100','BLUE-100'].filter(k=>tokens[k]);
  return `<main id="main" tabindex="-1" class="document">${titlePage('Design tokens','A shared visual language.','WeUI-derived palettes, application-wide theme control, and room for your own brand.')}<div class="theme-panel"><label for="palette-select">Palette</label><select id="palette-select">${Object.keys(data.tokens).map(k=>`<option value="${k}" ${k===palette?'selected':''}>${k}</option>`).join('')}</select><p>Values are generated from the pinned WeUI 2.6.26 CSS.</p><div class="swatches">${swatches.map(k=>`<div><div class="swatch" style="background:${escapeHTML(tokens[k])}"></div><strong>${k}</strong><code>${escapeHTML(tokens[k])}</code></div>`).join('')}</div></div><div class="two-column"><section><h2>Set the application theme</h2>${code('use wegpui::{Theme, ThemeMode};\n\nTheme::set_mode(ThemeMode::Dark, cx);\nTheme::set_care(true, cx);\nTheme::set_brand(Some(gpui::rgb(0x07c160).into()), cx);')}<p>Care mode and brand overrides are implemented; full visual and accessibility acceptance for every combination is still outstanding.</p>${docsLink('src/theme.rs','Theme API source')}</section><section><h2>Explore every token</h2><p>Colors, typography and component dimensions come from the same generated source. The raw values include CSS expressions where appropriate.</p><a class="source-link" href="${link('docs/tokens.json')}">Browse all ${Object.keys(tokens).length} tokens ↗</a>${apiLink('theme','Theme API reference')}<h3>Keep regeneration reproducible</h3><p>The extraction script preserves provenance and hashes for the upstream package.</p>${docsLink('docs/SOURCES.md','Sources and attribution')}</section></div></main>`;
}
function platforms() {
  const rows=[['macOS','Build and native samples','Build/tests/Clippy pass; current Arabic selection/undo and US date-picker keyboard samples have AX readback.','Current input/IME, resizing, screen-reader and distributable app acceptance.','desktop'],['Windows','Build/tests verified','Windows MSVC all-target build, library/gallery tests and Clippy pass. Visible Windows UI is outside the owner-selected scope.','Visible native UI, scaling, IME, Narrator/NVDA and packaging remain unqualified.','desktop'],['Linux','Runtime samples verified','Linux builds/tests pass. ARM64 X11 Arabic input and styled RTL probes pass; historical Wayland/Orca samples keep their own source identity.','Complete Orca acceptance, hardware desktop, IME/scaling matrix and packaging.','desktop'],['iOS','Experimental · simulator samples','The approved core bridge links a simulator bundle. Local iPhone 17 / iOS 26.2 navigation, safe areas, software-keyboard input and Pinyin samples are observed.','Complete keyboard/IME, insets, lifecycle, accessibility and device qualification; secure native password entry is missing.','experimental'],['Android','Experimental · no compatible backend','Historical library-only type-checks pass. A separate local backend targets GPUI 0.3.7 and needs adaptation to 0.3.9.','Backend integration and all application/runtime acceptance.','experimental']];
  return `<main id="main" tabindex="-1" class="document">${titlePage('Platform readiness','Evidence before promises.','The first release targets macOS, Windows and Linux. Mobile stays experimental. No platform has completed all release gates yet.')}<div class="status-key"><span><i class="status-dot"></i>Verified describes the named check only</span><span>Updated from recorded receipts</span></div><div class="platform-table" role="region" aria-label="Platform readiness" tabindex="0"><table><thead><tr><th>Platform</th><th>What we know</th><th>Still needed for release</th></tr></thead><tbody>${rows.map(([os,status,known,missing,kind])=>`<tr><th scope="row">${os}<span class="platform-badge ${kind}">${status}</span></th><td>${known}</td><td>${missing}</td></tr>`).join('')}</tbody></table></div><div class="two-column status-notes"><section><h2>What “supported” will mean</h2><p>A versioned build, real launch and rendering, input/focus/IME checks, assistive-technology acceptance and verified distribution. Every claim points to its tested source and host.</p><a href="#roadmap">Read the release roadmap →</a></section><section><h2>Inspect the evidence</h2><div class="doc-links">${docsLink('docs/PLATFORMS.md','Full acceptance matrix')}${docsLink(data.nativeEvidence.receipt,'Latest build and native evidence')}${docsLink('docs/receipts/wayland-20261025/README.md','Native Wayland samples')}${docsLink('docs/receipts/orca-20261025/README.md','Orca samples and remaining scope')}${docsLink('vendor/README.md','Approved accessibility compatibility sources')}${docsLink('docs/receipts/dictionary-mobile-20261026/README.md','Dictionary and iOS continuation')}</div></section></div></main>`;
}
function roadmap() {
  const stages=[['01','Correctness','Shared bidi geometry, styled text, locale dates and shaped clamping are implemented. Continue the measured typography, interaction and accessibility parity work.'],['02','Desktop acceptance','Verify native macOS, Windows and Linux runtimes, IME, scaling and screen readers on identified hosts.'],['03','Developer experience','Prove fresh-project setup, finalize examples and versioned API docs, and verify distributable packages.'],['04','Public release','Review the version, package name, signing, support policy and publication details. Publish the website, crate and desktop examples.']];
  return `<main id="main" tabindex="-1" class="document">${titlePage('Desktop-first release','A useful project.<br>A dependable release.','The website makes the work discoverable. Verification, clear contracts and reliable installation make it something people can depend on.')}<div class="roadmap-list">${stages.map(([n,t,d])=>`<section><span>${n}</span><div><h2>${t}</h2><p>${d}</p></div><span class="roadmap-status">In progress</span></section>`).join('')}</div><div class="honest-note"><div><h2>Help make the next gate pass.</h2><p>Useful contributions include reproducible bug reports, desktop testing, accessibility checks and documentation fixes. Experimental mobile work remains tracked separately.</p></div><a href="${link('docs/RELEASE-READINESS.md')}">Detailed checklist ↗</a></div><div class="doc-links">${docsLink('CONTRIBUTING.md','How to contribute')}${docsLink('CODE_OF_CONDUCT.md','Code of conduct')}<a href="${repo}/issues">Issues ↗</a></div></main>`;
}
function updatePreview(changes) {
  if (document.querySelector('#live-demo')) {
    updateLiveSettings(changes);
    return;
  }
  restoreFocus = changes.state ? "#state-select" : `[data-theme="${changes.theme}"]`;
  const info=current();
  for (const [key,value] of Object.entries(changes)) info.params.set(key,value);
  location.hash=`components/${info.route}?${info.params}`;
}

function updateLiveSettings(changes) {
  const info = current();
  for (const [key, value] of Object.entries(changes)) {
    if (value === null) info.params.delete(key);
    else info.params.set(key, value);
  }
  history.replaceState(null, '', `#components/${info.route}?${info.params}`);
  const config = demoConfig(info);
  const width = info.params.get('width') || demoPreferences.width;
  rememberDemo(config, width);
  const frame = document.querySelector('#live-demo');
  frame.dataset.config = JSON.stringify(config);
  frame.contentWindow?.postMessage({type: 'wegpui:configure', config}, location.origin);
  document.querySelectorAll('[data-theme]').forEach(button => button.setAttribute('aria-pressed', String(button.dataset.theme === config.theme)));
  document.querySelector('#care-toggle').checked = config.care;
  document.querySelector('#brand-color').value = config.brand ? `#${config.brand}` : data.tokens[config.care ? `care_${config.theme}` : config.theme]['BRAND-100'];
  document.querySelector('.live-frame-shell').style.setProperty('--demo-width', width === '768' ? '768px' : '375px');
  document.querySelector('.live-frame-shell').classList.toggle('is-wide', width === '768');
  document.querySelector('#open-demo').href = demoURL(config);
  document.querySelector('.preview-modes').outerHTML = viewTabs(info, 'live');
}
function render(focus=false) {
  const info=current();
  const page=info.page==='components'?componentPage(info):info.page==='get-started'?gettingStarted():info.page==='native'?nativeOverview():info.page==='native-guide'?nativeGettingStarted():info.page==='themes'?themes(info):info.page==='native-themes'?nativeThemes(info):info.page==='platforms'?platforms():info.page==='roadmap'?roadmap():overview();
  app.innerHTML=page.replaceAll('↗', '<svg class="external-icon" viewBox="0 0 16 16" fill="none" aria-hidden="true" focusable="false"><path d="M4 12 12 4M4 4h8v8" stroke="currentColor" stroke-width="1.4"/></svg>');
  if (info.page === 'components' && info.params.get('view') === 'native') {
    app.querySelector('.component-header')?.insertAdjacentHTML('afterend', viewTabs(info, 'native'));
  }
  document.querySelectorAll('[data-nav]').forEach(a=>a.toggleAttribute('aria-current',a.dataset.nav===info.page || (a.dataset.nav==='native' && info.page==='native-guide')));
  if (info.page === 'native') app.querySelectorAll('a[href="#get-started"]').forEach(anchor => anchor.setAttribute('href', '#native-guide'));
  document.querySelectorAll('[aria-current=""]').forEach(a=>a.setAttribute('aria-current','page'));
  const heading=app.querySelector('h1');
  document.title=`${heading?.textContent || 'Native WeUI components'} — weGPUI`;
  if(focus) {
    const key = info.page + '/' + (info.route || '');
    if (key !== lastPage) window.scrollTo(0, 0);
    (restoreFocus ? app.querySelector(restoreFocus) : app.querySelector('main'))?.focus({preventScroll:true});
    restoreFocus = null;
  }
  lastPage = info.page + '/' + (info.route || '');
  document.querySelector('#catalog-search')?.addEventListener('input',event=>{
    catalogQuery=event.target.value;
    document.querySelector('#catalog-list').innerHTML=catalogList(catalogQuery);
    document.querySelector('.catalog').classList.add('expanded');
    document.querySelector('.mobile-catalog-toggle').setAttribute('aria-expanded','true');
  });
  document.querySelector('.mobile-catalog-toggle')?.addEventListener('click',event=>{
    const button=event.currentTarget;const expanded=button.getAttribute('aria-expanded')!=='true';
    button.setAttribute('aria-expanded',String(expanded));document.querySelector('.catalog').classList.toggle('expanded',expanded);
  });
  document.querySelector('#state-select')?.addEventListener('change',event=>updatePreview({state:event.target.value}));
  document.querySelectorAll('[data-theme]').forEach(button=>button.addEventListener('click',()=>updatePreview({theme:button.dataset.theme})));
  document.querySelector('.compare-toggle')?.addEventListener('click',()=>{compare=!compare;render();document.querySelector('.compare-toggle').focus();});
  document.querySelector('#palette-select')?.addEventListener('change',event=>{
    const palette = event.target.value;
    const frame = document.querySelector('#theme-demo');
    if (!frame) { location.hash=info.page+'?palette='+encodeURIComponent(palette); return; }
    history.replaceState(null, '', '#themes?palette='+encodeURIComponent(palette));
    const updated = document.createElement('template');
    updated.innerHTML = themes(current());
    document.querySelector('.swatches').replaceWith(updated.content.querySelector('.swatches'));
    const snippet = document.querySelector('.two-column code');
    snippet.textContent = updated.content.querySelector('.two-column code').textContent;
    const config = {route:'form_checkbox', state:'default', theme:palette.endsWith('dark')?'dark':'light', care:palette.startsWith('care_'), brand:''};
    frame.dataset.config = JSON.stringify(config);
    frame.contentWindow?.postMessage({type:'wegpui:configure', config}, location.origin);
  });
  document.querySelector('#care-toggle')?.addEventListener('change', event => updateLiveSettings({care: event.target.checked ? '1' : '0'}));
  document.querySelector('#brand-color')?.addEventListener('input', event => updateLiveSettings({brand: event.target.value.slice(1)}));
  document.querySelector('#brand-reset')?.addEventListener('click', () => updateLiveSettings({brand: ''}));
  document.querySelector('#viewport-select')?.addEventListener('change', event => updateLiveSettings({width: event.target.value}));
  document.querySelectorAll('.copy').forEach(button=>button.addEventListener('click',async()=>{
    try {await navigator.clipboard.writeText(button.parentElement.querySelector('code').textContent);button.textContent='Copied';document.querySelector('#announcement').textContent='Code copied to clipboard.';}
    catch {button.textContent='Select code';const selection=window.getSelection();const range=document.createRange();range.selectNodeContents(button.parentElement.querySelector('code'));selection.removeAllRanges();selection.addRange(range);document.querySelector('#announcement').textContent='Clipboard unavailable. Code selected; use your copy shortcut.';}
  }));
}
window.addEventListener('message', event => {
  if (event.origin !== location.origin || !event.data || typeof event.data !== 'object') return;
  const frame = [...document.querySelectorAll('[data-playground]')].find(candidate => candidate.contentWindow === event.source);
  if (!frame) return;
  const message = event.data;
  if (message.type === 'wegpui:ready') {
    frame.contentWindow.postMessage({type: 'wegpui:configure', config: JSON.parse(frame.dataset.config)}, location.origin);
    const status = document.querySelector('#demo-status');
    if (status) status.textContent = 'Ready. Try the controls, or change the theme above.';
  } else if (message.type === 'wegpui:route' && data.routes.some(route => route.id === (message.route || message.config?.route))) {
    const nextRoute = message.route || message.config.route;
    const config = {...JSON.parse(frame.dataset.config), route: nextRoute, state: 'default'};
    const params = new URLSearchParams({theme: config.theme, care: config.care ? '1' : '0'});
    if (config.brand) params.set('brand', config.brand);
    const destination = `#components/${config.route}?${params}`;
    if (current().page !== 'components' || current().route !== nextRoute) location.hash = destination;
  } else if (message.type === 'wegpui:route' && message.config?.route === 'home') {
    if (current().page !== 'overview') location.hash = '#overview';
    else render(true);
  } else if (message.type === 'wegpui:error') {
    const status = document.querySelector('#demo-status');
    if (status) status.textContent = 'This example could not be opened. Try reloading or choose another component.';
  }
});
document.querySelector('.skip').addEventListener('click',event=>{event.preventDefault();document.querySelector('main')?.focus();});
window.addEventListener('hashchange',()=>{if(location.hash==='#main'){document.querySelector('main')?.focus();return;}render(true);});
render();
