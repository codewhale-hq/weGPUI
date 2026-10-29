'use strict';
const data = window.WEGPUI;
const repo = 'https://github.com/codewhale-hq/weGPUI';
const app = document.querySelector('#app');
const escapeHTML = value => String(value).replace(/[&<>"']/g, c => ({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]));
const link = path => `${repo}/blob/${data.sourceCommit}/${path}`;
const code = text => `<div class="code-block"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>${escapeHTML(text)}</code></pre></div>`;
const docsLink = (path, label) => `<a href="${path.startsWith('reports/') ? path : link(path)}">${label} <span aria-hidden="true">↗</span></a>`;
const apiLink = (module, label) => `<a class="module-link" href="${data.apiIncluded ? `api/wegpui/${module}/index.html` : 'source/api/' + module + '.rs.txt'}">${escapeHTML(label || module.replaceAll('/', '::'))} <span aria-hidden="true">↗</span></a>`;
const nativeLayout = (info = current()) => info.params.get('layout') === 'phone' ? 'phone' : 'desktop';
const nativeImages = (route, layout) => layout === 'desktop' ? route.desktopImages : route.images;
const nativeEvidence = (route, layout) => layout === 'desktop' ? route.desktopCaptureEvidence : route.captureEvidence;
const layoutControls = layout => `<fieldset class="segmented layout-control"><legend class="sr-only">Native layout</legend>${['desktop','phone'].map(value => `<button type="button" data-native-layout="${value}" aria-pressed="${layout === value}">${value === 'desktop' ? 'Desktop' : 'Phone'}</button>`).join('')}</fieldset>`;
const titlePage = (title, intro) => `<h1>${title}</h1><p class="page-intro">${intro}</p>`;
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
  params.set('theme', selected === 'live' ? demoConfig(info).theme : info.params.get('theme') === 'dark' ? 'dark' : 'light');
  const url = view => { params.set('view', view); return `#components/${info.route}?${params}`; };
  const route = data.routes.find(route => route.id === info.route);
  return `<nav class="preview-modes" aria-label="Component preview"><a href="${escapeHTML(url('native'))}" ${selected === 'native' ? 'aria-current="page"' : ''}>Native GPUI</a>${route?.playground ? `<a href="${escapeHTML(url('live'))}" ${selected === 'live' ? 'aria-current="page"' : ''}>WeUI browser reference</a>` : ''}</nav>`;
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
  if (info.params.get('view') !== 'live') return nativeComponentPage(info);
  const r = data.routes.find(route => route.id === info.route);
  if (!r?.playground) return nativeComponentPage(info);
  const config = demoConfig(info);
  const states = data.playground?.states?.[r.id] || r.states;
  if (!states.includes(config.state)) config.state = 'default';
  const width = info.params.get('width') || demoPreferences.width;
  const wide = width === '768';
  rememberDemo(config, width);
  const color = config.brand ? `#${config.brand}` : data.tokens[config.care ? `care_${config.theme}` : config.theme]['BRAND-100'];
  return `<div class="explorer">${catalogSidebar()}<main id="main" tabindex="-1" class="component-main live-component">
    <div class="component-header"><h1>${escapeHTML(r.title)}</h1><p class="page-intro">${escapeHTML(r.description.replace(/^Native /, ''))}</p></div>
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

const searchIcon = '<svg viewBox="0 0 20 20" fill="none" aria-hidden="true"><circle cx="8.5" cy="8.5" r="5.75" stroke="currentColor" stroke-width="1.5"/><path d="m13 13 4 4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>';
const galleryOrder = ['desktop-form', 'button_default', 'form_primary', 'dialog', 'picker', 'form_switch', 'navbar', 'actionsheet', 'form_checkbox', 'tabbar', 'list', 'slider', 'toast', 'form_select_primary', 'half-screen-dialog', 'panel', 'uploader', 'steps_vertical', 'preview', 'form_radio', 'searchbar_filled', 'badge', 'icons', 'languages', 'progress', 'grid'];
const galleryStates = {dialog:'ios1', picker:'single', actionsheet:'ios', 'half-screen-dialog':'1', toast:'success', navbar:'tab2', tabbar:'tab2', form_checkbox:'checked', form_radio:'checked', slider:'value80', progress:'uploading'};
let galleryQuery = '';
let galleryCategory = 'All';
let galleryTheme = 'light';
let galleryLayout = 'desktop';

function galleryItems() {
  const routes = [...data.routes].sort((a, b) => {
    const rank = route => galleryOrder.includes(route.id) ? galleryOrder.indexOf(route.id) : galleryOrder.length;
    return rank(a) - rank(b) || a.title.localeCompare(b.title);
  });
  return routes.filter(r => (galleryCategory === 'All' || r.category === galleryCategory) &&
    `${r.title} ${r.id} ${r.category} ${r.description}`.toLowerCase().includes(galleryQuery.toLowerCase()));
}

function galleryCards() {
  const routes = galleryItems();
  if (!routes.length) return '<div class="gallery-empty"><h2>No matching components</h2><p>Try another name or choose All.</p><button type="button" class="button secondary" id="clear-gallery">Clear filters</button></div>';
  return routes.map((r, index) => {
    const preferred = galleryStates[r.id] || 'default';
    const images = nativeImages(r, galleryLayout);
    const state = images[preferred]?.[galleryTheme] ? preferred : r.states.find(state => images[state]?.[galleryTheme]) || preferred;
    const captured = nativeEvidence(r, galleryLayout)?.[state]?.[galleryTheme];
    return `<a class="gallery-card" href="#components/${r.id}?state=${state}&theme=${galleryTheme}&layout=${galleryLayout}">
      <div class="gallery-image ${['dialog', 'toast', 'actionsheet', 'picker', 'half-screen-dialog'].includes(r.id) ? 'overlay-crop' : ''} ${['dialog', 'toast'].includes(r.id) ? 'center-crop' : ''}"><img src="${images[state][galleryTheme].gpui}" alt="${escapeHTML(r.title)}, ${state} state, rendered in native GPUI" width="${(captured?.viewport?.[0] || 375) * 2}" height="${(captured?.viewport?.[1] || 812) * 2}" loading="${index < 4 ? 'eager' : 'lazy'}" decoding="async"><span class="image-link" aria-hidden="true">↗</span></div>
      <div class="gallery-card-label"><h2>${escapeHTML(r.title)}</h2><span>${r.states.length > 1 ? `${r.states.length} states` : r.category}</span></div>
      <span class="sr-only">${captured ? escapeHTML(captured.platform) : 'macOS'} native capture. View Rust source and all states.</span>
    </a>`;
  }).join('');
}

function overview() {
  const params = current().params;
  galleryQuery = params.get('q') || '';
  galleryCategory = [...new Set(data.routes.map(r => r.category))].includes(params.get('category')) ? params.get('category') : 'All';
  galleryTheme = params.get('theme') === 'dark' ? 'dark' : 'light';
  galleryLayout = nativeLayout();
  return `<main id="main" tabindex="-1" class="gallery-page">
    <section class="gallery-intro"><div><h1>WeUI, <span>in Rust.</span></h1><p>WeUI components, rendered natively with GPUI.</p></div><a class="button primary" href="#get-started">Run the Rust gallery <span aria-hidden="true">→</span></a></section>
    <div class="gallery-toolbar"><label class="gallery-search">${searchIcon}<span class="sr-only">Search gallery</span><input id="gallery-search" type="search" placeholder="Find a component…" value="${escapeHTML(galleryQuery)}" autocomplete="off"><kbd aria-hidden="true">/</kbd></label><div class="gallery-tools">${layoutControls(galleryLayout)}<span class="gallery-total" role="status">${galleryItems().length} components</span><fieldset class="segmented"><legend class="sr-only">Gallery theme</legend>${['light','dark'].map(theme => `<button type="button" data-gallery-theme="${theme}" aria-pressed="${theme === galleryTheme}">${theme === 'light' ? 'Light' : 'Dark'}</button>`).join('')}</fieldset></div></div>
    <div class="gallery-categories" role="group" aria-label="Component category">${['All', 'Actions', 'Inputs', 'Navigation', 'Overlays', 'Feedback', 'Layout', 'Media', 'Content', 'Gallery'].map(category => `<button type="button" data-category="${category}" aria-pressed="${category === galleryCategory}">${category === 'Inputs' ? 'Forms & inputs' : category === 'Gallery' ? 'Composition' : category}</button>`).join('')}</div>
    <div class="gallery-grid" id="gallery-grid" data-layout="${galleryLayout}">${galleryCards()}</div>
    <p class="gallery-caption">Desktop and phone layouts, captured from the Rust gallery. Open a component for its states, source and run command. Mobile backends remain experimental.</p>
  </main>`;
}

function updateGallery() {
  const params = new URLSearchParams();
  if (galleryQuery) params.set('q', galleryQuery);
  if (galleryCategory !== 'All') params.set('category', galleryCategory);
  if (galleryTheme !== 'light') params.set('theme', galleryTheme);
  if (galleryLayout === 'phone') params.set('layout', 'phone');
  history.replaceState(null, '', '#overview' + (params.size ? '?' + params : ''));
  document.querySelector('#gallery-grid').innerHTML = galleryCards();
  document.querySelector('#gallery-grid').dataset.layout = galleryLayout;
  document.querySelectorAll('[data-native-layout]').forEach(button => button.setAttribute('aria-pressed', String(button.dataset.nativeLayout === galleryLayout)));
  document.querySelector('.gallery-total').textContent = `${galleryItems().length} components`;
  document.querySelectorAll('[data-category]').forEach(button => button.setAttribute('aria-pressed', String(button.dataset.category === galleryCategory)));
  document.querySelectorAll('[data-gallery-theme]').forEach(button => button.setAttribute('aria-pressed', String(button.dataset.galleryTheme === galleryTheme)));

}

function catalogList(query) {
  const filtered = data.routes.filter(r=>`${r.title} ${r.id} ${r.category} ${r.description}`.toLowerCase().includes(query.toLowerCase()));
  const selected = current().route;
  const mode = current().params.get('view') === 'live' ? '?view=live' : '?layout=' + nativeLayout();
  const categories = [...new Set(filtered.map(r=>r.category))];
  return `<p class="result-count" aria-live="polite">${filtered.length} of ${data.routes.length} routes</p>${filtered.length ? categories.map(category=>`<section class="nav-group"><h3>${category}</h3>${filtered.filter(r=>r.category===category).map(r=>`<a href="#components/${r.id}${mode}" ${selected===r.id?'aria-current="page"':''}>${escapeHTML(r.title)}<span aria-hidden="true">${r.states.length>1?r.states.length:''}</span></a>`).join('')}</section>`).join('') : '<p class="empty-results">No matching components. Try “form”, “dialog” or “navigation”.</p>'}`;
}
function current() {
  const [path, query=''] = location.hash.slice(1).split('?');
  const parts=(path || 'overview').split('/');
  return {page:parts[0],route:parts[1],params:new URLSearchParams(query)};
}
function nativeComponentPage(info) {
  const r = data.routes.find(route => route.id === info.route);
  if (!r) return `<main id="main" tabindex="-1" class="document"><h1>Component not found.</h1><a class="button primary" href="#overview">Back to the gallery</a></main>`;
  const layout = nativeLayout(info);
  const images = nativeImages(r, layout);
  const theme = info.params.get('theme') === 'dark' ? 'dark' : 'light';
  const requestedState = r.states.includes(info.params.get('state')) ? info.params.get('state') : 'default';
  const state = images[requestedState]?.[theme] ? requestedState : r.states.find(state => images[state]?.[theme]) || requestedState;
  const capture = nativeEvidence(r, layout)?.[state]?.[theme] || {commit:data.captureCommit, platform:'macOS', viewport:[375,812], receipt:'docs/PARITY.md'};
  const command = capture.args ? 'cargo run --example gallery -- ' + capture.args.join(' ') : `cargo run --example gallery -- --page ${r.id}${state === 'default' || r.nativeArgs ? '' : ' --state ' + state} --theme ${theme} --size ${capture.viewport.join('x')}${r.nativeArgs?.[state] ? ' ' + r.nativeArgs[state] : ''}`;
  return `<div class="explorer">${catalogSidebar()}<main id="main" tabindex="-1" class="component-main native-component">
    <div class="component-header"><a class="back-link" href="#overview?layout=${layout}&theme=${theme}">← All components</a><h1>${escapeHTML(r.title)}</h1><p class="page-intro">${escapeHTML(r.description)}</p></div>
    ${viewTabs(info, 'native')}
    <div class="preview-toolbar"><span class="preview-label">Native GPUI</span>${layoutControls(layout)}<label>State <select id="state-select">${r.states.map(value => `<option value="${value}" ${value === state ? 'selected' : ''}>${value === 'default' ? (r.id === 'languages' && layout === 'phone' ? 'English paragraphs' : 'Default') : escapeHTML(value.replaceAll('-', ' '))}</option>`).join('')}</select></label><fieldset class="segmented"><legend class="sr-only">Capture theme</legend>${['light','dark'].map(value => `<button type="button" data-theme="${value}" aria-pressed="${value === theme}">${value === 'light' ? 'Light' : 'Dark'}</button>`).join('')}</fieldset>${r.playground && layout === 'phone' ? `<button type="button" class="compare-toggle" aria-pressed="${compare}">${compare ? 'Hide comparison' : 'Compare WeUI'}</button>` : ''}</div>
    <div class="component-workspace ${layout === 'desktop' ? 'desktop-workspace' : ''} ${compare && r.playground && layout === 'phone' ? 'comparing' : ''}"><div class="preview-stage"><figure><figcaption><span>weGPUI</span><span>${escapeHTML(capture.platform)}</span></figcaption><div class="capture-window ${r.id === 'languages' && state === 'default' && layout === 'phone' ? 'paragraph-capture' : ''}"><img id="native-capture" src="${images[state][theme].gpui}" width="${capture.viewport[0] * 2}" height="${capture.viewport[1] * 2}" alt="${escapeHTML(r.title)}, ${escapeHTML(state)} state, ${theme} theme: ${escapeHTML(capture.platform)} native capture"></div></figure>${compare && r.playground && layout === 'phone' ? `<figure><figcaption><span>WeUI reference</span><span>Browser</span></figcaption><div class="capture-window"><img src="${r.images[state][theme].weui}" width="750" height="1624" alt="WeUI browser reference for ${escapeHTML(r.title)}"></div></figure>` : ''}</div>
    <aside class="source-pane"><div class="source-heading"><h2>Rust source</h2><a href="${r.rustSource}">${escapeHTML(r.source.split('/').at(-1))} ↗</a></div><div id="rust-source" class="code-block"><button class="copy" type="button" aria-label="Copy Rust source" disabled>Copy</button><pre tabindex="0" aria-label="Rust gallery source"><code>Loading Rust source…</code></pre></div><p id="source-status" class="source-status" role="status"></p><div class="module-list">${r.modules.map(module => apiLink(module)).join('')}</div></aside></div>
    <details class="run-example"><summary>Run this example</summary>${code(command)}<p>The recorded ${layout} preview uses a ${capture.viewport.join(' × ')} layout.</p></details>
    <p class="capture-note">Recorded on ${escapeHTML(capture.platform)} at <code>${capture.commit.slice(0,7)}</code>${capture.modified ? ' with the desktop gallery changes' : ''}. ${docsLink(capture.receipt, 'Capture details')} · <a href="#get-started">Set up weGPUI</a></p>
  </main></div>`;
}

function nativeGettingStarted() {
  const install=`[dependencies]\nwegpui = { git = "${repo}", rev = "${data.sourceCommit}" }\ngpui = { package = "gpui-pre", version = "=${data.gpuiVersion}" }\ngpui_platform = { package = "gpui-pre-platform", version = "=${data.gpuiVersion}", features = ["font-kit", "wayland", "x11"] }`;
  const patches=Object.entries(data.compatibilityPatches || {}).map(([name,path])=>`${name} = { path = "../weGPUI/${path}" }`).join('\n');
  const patchGuide=patches?`<h3>Match the approved compatibility sources</h3><p>The current native library requires the patched GPUI API to compile. Cargo patches in a dependency are not inherited. Keep a checkout at the same revision alongside your application and add these entries to the application's workspace-root Cargo.toml. Adjust the paths to your checkout.</p>${code('[patch.crates-io]\n'+patches)}<p>The package versions stay unchanged. These approved local copies carry the sampled Linux accessibility fixes.</p>${docsLink('vendor/README.md','Compatibility changes and limits')}`:'';
  const previewNote=data.sourceDirty?'<p class="provenance">This local preview includes uncommitted changes. Source links and the Git dependency revision identify the last commit; rebuild from committed source before sharing this setup.</p>':'';
  return `<main id="main" tabindex="-1" class="document">${titlePage('From clone to native window.','Start with a complete example, then bring the components into your own GPUI application.')}<div class="doc-layout"><article><section><h2><span class="step-number">01</span>Set up your environment</h2><p>The manifest declares Rust ${data.rustVersion} or newer; Linux stable and the declared minimum version pass the recorded build checks. macOS requires Xcode tooling; Windows needs the MSVC build tools and Windows SDK. Linux needs the system libraries below.</p><details><summary>Debian / Ubuntu system dependencies</summary>${code('sudo apt-get install libxkbcommon-dev libxkbcommon-x11-dev \\\n  libwayland-dev libvulkan-dev libfontconfig-dev libfreetype-dev \\\n  libx11-xcb-dev libxcb1-dev libssl-dev pkg-config clang')}</details></section><section><h2><span class="step-number">02</span>Run the gallery</h2>${code('git clone '+repo+'.git\ncd weGPUI\ncargo run --example gallery')}<p>Use <code>-- --size 1100x760</code> for a desktop-sized window, or <code>-- --size 375x812</code> for the phone layout.</p><p>Or run <code>cargo run --example minimal</code> for a small application with a dialog, state updates and focus handling.</p></section><section><h2><span class="step-number">03</span>Add it to your app</h2><p>There is no crates.io release yet. Use a Git revision with matching exact GPUI pins. The example below includes Linux windowing features.</p>${code(install)}${previewNote}${patchGuide}<p>Call <code>wegpui::init(cx)</code>, attach <code>wegpui::Assets</code>, and use <code>Theme</code> and the components inside your GPUI views.</p><a class="button secondary" href="${link('examples/minimal.rs')}">Read the complete minimal application ↗</a></section><section><h2><span class="step-number">04</span>Build with the reference nearby</h2><div class="doc-links">${data.apiIncluded?'<a href="api/wegpui/index.html">API reference ↗</a>':docsLink('src/lib.rs','Library documentation')}${docsLink('CONTRIBUTING.md','Contributor guide')}${docsLink('docs/UPSTREAM-COVERAGE.md','Upstream example mapping')}${docsLink('docs/STATUS.md','Known differences')}</div></section></article><aside class="reading-note"><h2>Before you ship</h2><p>The project is a developer preview. Desktop runtime and screen-reader acceptance are not complete. Mobile remains experimental.</p>${docsLink('docs/PLATFORMS.md', 'Review platform status')}<hr><h2>Why exact GPUI pins?</h2><p>These components depend on GPUI APIs that evolve. Keep the application and library on the same GPUI snapshot.</p><hr><h2>A library, not a backend</h2><p>Your application owns business logic, data, persistence and upload transport. The gallery demonstrates component composition.</p></aside></div></main>`;
}
function themes(info) {
  const palette = Object.hasOwn(data.tokens, info.params.get('palette')) ? info.params.get('palette') : 'light';
  const tokens = data.tokens[palette];
  const config = {route: 'form_checkbox', state: 'default', theme: palette.endsWith('dark') ? 'dark' : 'light', care: palette.startsWith('care_'), brand: ''};
  rememberDemo(config, demoPreferences.width);
  const swatches = ['BRAND-100', 'BG-0', 'BG-1', 'BG-2', 'FG-0', 'FG-1', 'RED-100', 'BLUE-100'].filter(key => tokens[key]);
  return `<main id="main" tabindex="-1" class="document">${titlePage('Make it feel like yours.', 'Explore the pinned WeUI palettes, then try light, dark, care contrast and your own brand in any component.')}<div class="theme-panel"><label for="palette-select">Palette</label><select id="palette-select">${Object.keys(data.tokens).map(key => `<option value="${key}" ${key === palette ? 'selected' : ''}>${key.replaceAll('_', ' ')}</option>`).join('')}</select><p>Original colour values from WeUI 2.6.26. Care mode increases contrast.</p><div class="swatches">${swatches.map(key => `<div><div class="swatch" style="background:${escapeHTML(tokens[key])}"></div><strong>${key}</strong><code>${escapeHTML(tokens[key])}</code></div>`).join('')}</div></div><div class="two-column"><section><h2>Use the palette</h2>${code(`document.body.setAttribute('data-weui-theme', '${config.theme}');\ndocument.body.setAttribute('data-weui-mode', '${config.care ? 'care' : 'normal'}');`)}<p>Use the controls above any live example to change its brand colour. Reset returns to the selected palette.</p><a class="text-link" href="#components/button_default">Try a themed component →</a><h3>Keep the source nearby</h3><p>All ${Object.keys(tokens).length} generated tokens and their original CSS are available to inspect.</p>${docsLink('docs/tokens.json', 'Browse the token source')}<p><a href="playground/vendor/weui.css">View the pinned stylesheet ↗</a></p></section><section class="theme-live-preview"><h2>Live palette preview</h2><div class="live-frame-shell">${liveFrame(config, 'theme-demo')}</div></section></div></main>`;
}

function nativeThemes(info) {
  const palette=Object.hasOwn(data.tokens,info.params.get('palette'))?info.params.get('palette'):'light';
  const tokens=data.tokens[palette];
  const swatches=['BRAND-100','BG-0','BG-1','BG-2','FG-0','FG-1','RED-100','BLUE-100'].filter(k=>tokens[k]);
  return `<main id="main" tabindex="-1" class="document">${titlePage('A shared visual language.','WeUI-derived palettes, application-wide theme control, and room for your own brand.')}<div class="theme-panel"><label for="palette-select">Palette</label><select id="palette-select">${Object.keys(data.tokens).map(k=>`<option value="${k}" ${k===palette?'selected':''}>${k}</option>`).join('')}</select><p>Values are generated from the pinned WeUI 2.6.26 CSS.</p><div class="swatches">${swatches.map(k=>`<div><div class="swatch" style="background:${escapeHTML(tokens[k])}"></div><strong>${k}</strong><code>${escapeHTML(tokens[k])}</code></div>`).join('')}</div></div><div class="two-column"><section><h2>Set the application theme</h2>${code('use wegpui::{Theme, ThemeMode};\n\nTheme::set_mode(ThemeMode::Dark, cx);\nTheme::set_care(true, cx);\nTheme::set_brand(Some(gpui::rgb(0x07c160).into()), cx);')}<p>Care mode and brand overrides are implemented; full visual and accessibility acceptance for every combination is still outstanding.</p>${docsLink('src/theme.rs','Theme API source')}</section><section><h2>Explore every token</h2><p>Colors, typography and component dimensions come from the same generated source. The raw values include CSS expressions where appropriate.</p><a class="source-link" href="${link('docs/tokens.json')}">Browse all ${Object.keys(tokens).length} tokens ↗</a>${apiLink('theme','Theme API reference')}<h3>Keep regeneration reproducible</h3><p>The extraction script preserves provenance and hashes for the upstream package.</p>${docsLink('docs/SOURCES.md','Sources and attribution')}</section></div></main>`;
}
function updatePreview(changes) {
  if (document.querySelector('#live-demo')) {
    updateLiveSettings(changes);
    return;
  }
  restoreFocus = changes.layout ? `[data-native-layout="${changes.layout}"]` : changes.state ? "#state-select" : `[data-theme="${changes.theme}"]`;
  const info=current();
  const route = data.routes.find(route => route.id === info.route);
  const images = route ? nativeImages(route, changes.layout || nativeLayout(info)) : null;
  if (changes.state && route) {
    const theme = info.params.get('theme') === 'dark' ? 'dark' : 'light';
    if (!images[changes.state]?.[theme]) changes.theme = Object.keys(images[changes.state])[0];
  }
  if ((changes.theme || changes.layout) && !changes.state && route) {
    changes.theme = changes.theme || (info.params.get('theme') === 'dark' ? 'dark' : 'light');
    const state = info.params.get('state') || 'default';
    if (!images[state]?.[changes.theme]) changes.state = route.states.find(state => images[state]?.[changes.theme]) || state;
  }
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
  const page = info.page === 'components' ? componentPage(info) : ['get-started', 'native-guide'].includes(info.page) ? nativeGettingStarted() : ['themes', 'native-themes'].includes(info.page) ? nativeThemes(info) : info.page === 'reference-themes' ? themes(info) : overview();
  app.innerHTML=page.replaceAll('↗', '<svg class="external-icon" viewBox="0 0 16 16" fill="none" aria-hidden="true" focusable="false"><path d="M4 12 12 4M4 4h8v8" stroke="currentColor" stroke-width="1.4"/></svg>');

  document.querySelectorAll('[data-nav]').forEach(a=>a.toggleAttribute('aria-current',a.dataset.nav===info.page || (a.dataset.nav==='overview' && ['components','native'].includes(info.page)) || (a.dataset.nav==='get-started' && info.page==='native-guide') || (a.dataset.nav==='themes' && info.page==='native-themes')));
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
  document.querySelector('#gallery-search')?.addEventListener('input', event => { galleryQuery = event.target.value; updateGallery(); });
  document.querySelector('#gallery-grid')?.addEventListener('click', event => {
    if (!event.target.closest('#clear-gallery')) return;
    galleryQuery = '';
    galleryCategory = 'All';
    document.querySelector('#gallery-search').value = '';
    updateGallery();
    document.querySelector('#gallery-search').focus();
  });
  document.querySelectorAll('[data-category]').forEach(button => button.addEventListener('click', () => { galleryCategory = button.dataset.category; updateGallery(); }));
  document.querySelectorAll('[data-gallery-theme]').forEach(button => button.addEventListener('click', () => { galleryTheme = button.dataset.galleryTheme; updateGallery(); }));
  document.querySelectorAll('[data-native-layout]').forEach(button => button.addEventListener('click', () => {
    if (document.querySelector('#gallery-grid')) { galleryLayout = button.dataset.nativeLayout; updateGallery(); }
    else updatePreview({layout:button.dataset.nativeLayout});
  }));
  const sourceNode = document.querySelector('#rust-source code');
  if (sourceNode) {
    const route = data.routes.find(route => route.id === info.route);
    fetch(route.rustSource).then(response => { if (!response.ok) throw new Error('Source unavailable'); return response.text(); }).then(source => {
      if (!sourceNode.isConnected) return;
      sourceNode.textContent = source;
      sourceNode.closest('.code-block').querySelector('.copy').disabled = false;
      document.querySelector('#source-status').textContent = `${source.split('\n').length} lines · ${route.source}`;
    }).catch(() => {
      if (!sourceNode.isConnected) return;
      sourceNode.textContent = 'Could not load the Rust source. Open the source link above to read it on GitHub.';
      document.querySelector('#source-status').textContent = 'Source preview unavailable.';
    });
  }
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
    history.replaceState(null, '', '#reference-themes?palette='+encodeURIComponent(palette));
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
    params.set('view', 'live');
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
document.addEventListener('keydown', event => {
  if (event.key === '/' && !event.ctrlKey && !event.metaKey && !event.altKey && !['INPUT', 'TEXTAREA', 'SELECT'].includes(event.target.tagName) && !event.target.isContentEditable) {
    const search = document.querySelector('#gallery-search, #catalog-search');
    if (search) { event.preventDefault(); search.focus(); }
  }
});
window.addEventListener('hashchange',()=>{if(location.hash==='#main'){document.querySelector('main')?.focus();return;}render(true);});
render();
