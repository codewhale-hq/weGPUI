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

function overview() {
  const features = [['Native by construction','Rust components rendered by GPUI. No embedded browser is required for the desktop gallery.'],['A familiar design language','WeUI 2.6.26 layouts, tokens and assets, adapted to native application composition.'],['Built to inspect','Explore the source, reproduce a gallery state, and see exactly what has been verified.']];
  return `<main id="main" tabindex="-1" class="overview">
    <section class="hero"><div class="hero-copy">${eyebrow('WeUI × Rust × GPUI')}<h1>Familiar design.<br><em>Native feel.</em></h1><p>A WeUI component library for native Rust applications. Forms, navigation, overlays and the details in between—ready to explore in GPUI.</p><div class="hero-actions"><a class="button primary" href="#components/button_default">Explore the components <span aria-hidden="true">→</span></a><a class="button secondary" href="#get-started">Build your first screen</a></div><p class="hero-footnote">Open source · Desktop-first · Developer preview</p></div><div class="hero-visual" aria-label="Native gallery screenshot"><div class="visual-caption"><span class="status-dot"></span>Captured from the native gallery <span class="mono">01 / FORM</span></div><div class="hero-capture"><img src="captures/form_primary-light-gpui.png" alt="Native WeUI form with title, fields and a green primary action" width="750" height="1624"></div><div class="visual-tag"><span class="mini-code">&lt;/&gt;</span><span>Rust all the way down.<small>Composed with GPUI elements.</small></span></div></div></section>
    <section class="feature-row" aria-label="About the library">${features.map(([title,text])=>`<div><h2>${title}</h2><p>${text}</p></div>`).join('')}</section>
    <section class="home-catalog"><div class="section-heading"><div>${eyebrow('A complete gallery inventory')}<h2>Small pieces.<br>Whole experiences.</h2></div><p>Browse ${data.routes.length} example routes, from a single button to a complete form. Route coverage is implemented; release acceptance is still in progress.</p></div><div class="showcase-grid">${[['button_default','Actions, with intention.'],['form_select','Inputs that belong.'],['picker','Choices, made clear.']].map(([id,caption])=>{const r=data.routes.find(x=>x.id===id);const state=id==='picker'?'single':'default';return `<a class="showcase-card" href="#components/${id}${state==='default'?'':'?state='+state}"><div class="showcase-image"><img src="${r.images[state].light.gpui}" width="750" height="1624" loading="lazy" alt="${escapeHTML(r.title)} native gallery capture"></div><div><span>${caption}</span><span aria-hidden="true">↗</span></div><small>${r.familyLabel}</small></a>`;}).join('')}</div></section>
    <section class="build-banner"><div>${eyebrow('From gallery to application')}<h2>Your next screen<br>starts here.</h2><p>Run the native gallery, inspect its source, and adapt a complete example to your app.</p><a class="text-link" href="#get-started">Read the getting-started guide →</a></div>${code('git clone https://github.com/codewhale-hq/weGPUI.git\ncd weGPUI\ncargo run --example gallery')}</section>
    <section class="honest-note"><span class="outlined-icon" aria-hidden="true">◎</span><div><h2>Open source. Open about the gaps.</h2><p>We’re preparing a verified desktop release. Mobile is experimental, and no platform has completed every acceptance gate yet.</p></div><a href="#platforms">See platform status →</a></section>
  </main>`;
}
function catalogList(query) {
  const filtered = data.routes.filter(r=>`${r.title} ${r.id} ${r.category} ${r.description}`.toLowerCase().includes(query.toLowerCase()));
  const selected = current().route;
  const categories = [...new Set(filtered.map(r=>r.category))];
  return `<p class="result-count" aria-live="polite">${filtered.length} of ${data.routes.length} routes</p>${filtered.length ? categories.map(category=>`<section class="nav-group"><h3>${category}</h3>${filtered.filter(r=>r.category===category).map(r=>`<a href="#components/${r.id}" ${selected===r.id?'aria-current="page"':''}>${escapeHTML(r.title)}<span aria-hidden="true">${r.states.length>1?r.states.length:''}</span></a>`).join('')}</section>`).join('') : '<p class="empty-results">No matching components. Try “form”, “dialog” or “navigation”.</p>'}`;
}
function current() {
  const [path, query=''] = location.hash.slice(1).split('?');
  const parts=(path || 'overview').split('/');
  return {page:parts[0],route:parts[1],params:new URLSearchParams(query)};
}
function componentPage(info) {
  const r = data.routes.find(r=>r.id===info.route);
  if (!r) return `<main id="main" tabindex="-1" class="document">${titlePage('Not found','That component isn’t in this gallery.','Try the component catalog or return to the overview.')}<a class="button primary" href="#components/button_default">Browse components</a></main>`;
  const theme=info.params.get('theme')==='dark'?'dark':'light';
  const state=r.states.includes(info.params.get('state'))?info.params.get('state'):'default';
  const command=`cargo run --example gallery -- --page ${r.id}${state==='default'?'':' --state '+state} --theme ${theme}`;
  return `<div class="explorer"><aside class="catalog" aria-label="Component catalog"><div class="catalog-top"><h2>Components <span>${data.routes.length}</span></h2><label class="search-box"><span class="sr-only">Search components</span><span aria-hidden="true">⌕</span><input id="catalog-search" type="search" placeholder="Search components…" value="${escapeHTML(catalogQuery)}" autocomplete="off"></label><button class="mobile-catalog-toggle" aria-expanded="false" aria-controls="catalog-list">Browse all components <span aria-hidden="true">⌄</span></button></div><div id="catalog-list">${catalogList(catalogQuery)}</div></aside>
    <main id="main" tabindex="-1" class="component-main"><div class="component-header">${eyebrow(r.category+' / '+r.familyLabel)}<h1>${escapeHTML(r.title)}</h1><p class="page-intro">${escapeHTML(r.description)}</p></div>
    <div class="preview-toolbar"><div class="preview-label"><span class="status-dot"></span>Native capture</div><label>State <select id="state-select">${r.states.map(s=>`<option value="${s}" ${s===state?'selected':''}>${s==='default'?'Default':s}</option>`).join('')}</select></label><fieldset class="segmented"><legend class="sr-only">Capture theme</legend>${['light','dark'].map(t=>`<button type="button" data-theme="${t}" aria-pressed="${t===theme}">${t==='light'?'Light':'Dark'}</button>`).join('')}</fieldset><button type="button" class="compare-toggle" aria-pressed="${compare}">${compare?'Hide comparison':'Compare WeUI'}</button></div>
    <div class="component-workspace ${compare?'comparing':''}"><div class="preview-stage"><figure><figcaption><span>weGPUI</span><span class="mono">NATIVE / macOS</span></figcaption><div class="capture-window"><img id="native-capture" src="${r.images[state][theme].gpui}" width="750" height="1624" alt="${escapeHTML(r.title)}, ${state} state, ${theme} theme: historical native macOS capture"></div></figure>${compare?`<figure><figcaption><span>WeUI reference</span><span class="mono">BROWSER</span></figcaption><div class="capture-window"><img src="${r.images[state][theme].weui}" width="750" height="1624" alt="Upstream WeUI reference for ${escapeHTML(r.title)}, ${state}, ${theme}"></div></figure>`:''}</div>
    <aside class="component-details"><div><h2>Try it natively</h2><p>The preview is a recorded screenshot. Run this command for the interactive desktop example.</p>${code(command)}</div><div><h2>Go to the source</h2><a class="source-link" href="${link(r.source)}">Gallery implementation <span aria-hidden="true">↗</span></a><div class="module-list">${r.modules.length?r.modules.map(m=>apiLink(m)).join(''):docsLink(r.source,'Gallery-specific implementation')}</div></div><div class="provenance"><h2>About this capture</h2><p>Historical macOS capture from <code>${data.captureCommit}</code>, at a 375 × 812 pt viewport. It illustrates the component; it is not a fresh runtime or accessibility verdict.</p>${docsLink('docs/PARITY.md','Capture method and differences')}<br>${docsLink('docs/STATUS.md','Known component gaps')}</div></aside></div>
    <div class="component-bottom"><span class="mono">${escapeHTML(r.id)}</span><a href="#get-started">Set up your application →</a></div></main></div>`;
}
function gettingStarted() {
  const install=`[dependencies]\nwegpui = { git = "${repo}", rev = "${data.sourceCommit}" }\ngpui = { package = "gpui-pre", version = "=${data.gpuiVersion}" }\ngpui_platform = { package = "gpui-pre-platform", version = "=${data.gpuiVersion}", features = ["font-kit", "wayland", "x11"] }`;
  return `<main id="main" tabindex="-1" class="document">${titlePage('Getting started','From clone to native window.','Start with a complete example, then bring the components into your own GPUI application.')}<div class="doc-layout"><article><section><h2><span class="step-number">01</span>Set up your environment</h2><p>Use Rust ${data.rustVersion} or newer. macOS requires Xcode tooling; Windows needs the MSVC build tools and Windows SDK. Linux needs the system libraries below.</p><details><summary>Debian / Ubuntu system dependencies</summary>${code('sudo apt-get install libxkbcommon-dev libxkbcommon-x11-dev \\\n  libwayland-dev libvulkan-dev libfontconfig-dev libfreetype-dev \\\n  libx11-xcb-dev libxcb1-dev libssl-dev pkg-config clang')}</details></section><section><h2><span class="step-number">02</span>Run the gallery</h2>${code('git clone '+repo+'.git\ncd weGPUI\ncargo run --example gallery')}<p>Or run <code>cargo run --example minimal</code> for a small application with a dialog, state updates and focus handling.</p></section><section><h2><span class="step-number">03</span>Add it to your app</h2><p>There is no crates.io release yet. Use a Git revision with matching exact GPUI pins. The example below includes Linux windowing features.</p>${code(install)}<p>Call <code>wegpui::init(cx)</code>, attach <code>wegpui::Assets</code>, and use <code>Theme</code> and the components inside your GPUI views.</p><a class="button secondary" href="${link('examples/minimal.rs')}">Read the complete minimal application ↗</a></section><section><h2><span class="step-number">04</span>Build with the reference nearby</h2><div class="doc-links">${data.apiIncluded?'<a href="api/wegpui/index.html">API reference ↗</a>':docsLink('src/lib.rs','Library documentation')}${docsLink('CONTRIBUTING.md','Contributor guide')}${docsLink('docs/UPSTREAM-COVERAGE.md','Upstream example mapping')}${docsLink('docs/STATUS.md','Known differences')}</div></section></article><aside class="reading-note"><h2>Before you ship</h2><p>The project is a developer preview. Desktop runtime and screen-reader acceptance are not complete. Mobile remains experimental.</p><a href="#platforms">Review platform status →</a><hr><h2>Why exact GPUI pins?</h2><p>These components depend on GPUI APIs that evolve. Keep the application and library on the same GPUI snapshot.</p><hr><h2>A library, not a backend</h2><p>Your application owns business logic, data, persistence and upload transport. The gallery demonstrates component composition.</p></aside></div></main>`;
}
function themes(info) {
  const palette=Object.hasOwn(data.tokens,info.params.get('palette'))?info.params.get('palette'):'light';
  const tokens=data.tokens[palette];
  const swatches=['BRAND-100','BG-0','BG-1','BG-2','FG-0','FG-1','RED-100','BLUE-100'].filter(k=>tokens[k]);
  return `<main id="main" tabindex="-1" class="document">${titlePage('Design tokens','A shared visual language.','WeUI-derived palettes, application-wide theme control, and room for your own brand.')}<div class="theme-panel"><label for="palette-select">Palette</label><select id="palette-select">${Object.keys(data.tokens).map(k=>`<option value="${k}" ${k===palette?'selected':''}>${k}</option>`).join('')}</select><p>Values are generated from the pinned WeUI 2.6.26 CSS.</p><div class="swatches">${swatches.map(k=>`<div><div class="swatch" style="background:${escapeHTML(tokens[k])}"></div><strong>${k}</strong><code>${escapeHTML(tokens[k])}</code></div>`).join('')}</div></div><div class="two-column"><section><h2>Set the application theme</h2>${code('use wegpui::{Theme, ThemeMode};\n\nTheme::set_mode(ThemeMode::Dark, cx);\nTheme::set_care(true, cx);\nTheme::set_brand(Some(gpui::rgb(0x07c160).into()), cx);')}<p>Care mode and brand overrides are implemented; full visual and accessibility acceptance for every combination is still outstanding.</p>${docsLink('src/theme.rs','Theme API source')}</section><section><h2>Explore every token</h2><p>Colors, typography and component dimensions come from the same generated source. The raw values include CSS expressions where appropriate.</p><a class="source-link" href="${link('docs/tokens.json')}">Browse all ${Object.keys(tokens).length} tokens ↗</a>${apiLink('theme','Theme API reference')}<h3>Keep regeneration reproducible</h3><p>The extraction script preserves provenance and hashes for the upstream package.</p>${docsLink('docs/SOURCES.md','Sources and attribution')}</section></div></main>`;
}
function platforms() {
  const rows=[['macOS','Build verified','Hosted native build, tests and Clippy pass. Historical rendering and AX samples exist.','Current input/IME, resizing, VoiceOver and distributable app acceptance.','desktop'],['Windows','Qualification pending','Historical mingw cross-build exists. A native MSVC CI build is part of the desktop release plan.','Native windows, scaling, keyboard/IME, Narrator or NVDA, and packaging.','desktop'],['Linux','X11 samples verified','Builds and sampled Xvfb rendering, XTEST input and AT-SPI trees pass.','Wayland, broader input/IME/scaling, Orca and packaging.','desktop'],['iOS','Experimental · blocked','Library type-checks pass. The simulator app build fails in the published GPUI 0.3.9 pair.','Compatible upstream APIs, then full mobile runtime and accessibility qualification.','experimental'],['Android','Experimental · no backend','Library-only type-check passes. No upstream native platform backend is available.','Backend implementation and all application/runtime acceptance.','experimental']];
  return `<main id="main" tabindex="-1" class="document">${titlePage('Platform readiness','Evidence before promises.','The first release targets macOS, Windows and Linux. Mobile stays experimental. No platform has completed all release gates yet.')}<div class="status-key"><span><i class="status-dot"></i>Verified describes the named check only</span><span>Updated from committed receipts</span></div><div class="platform-table" role="region" aria-label="Platform readiness" tabindex="0"><table><thead><tr><th>Platform</th><th>What we know</th><th>Still needed for release</th></tr></thead><tbody>${rows.map(([os,status,known,missing,kind])=>`<tr><th scope="row">${os}<span class="platform-badge ${kind}">${status}</span></th><td>${known}</td><td>${missing}</td></tr>`).join('')}</tbody></table></div><div class="two-column status-notes"><section><h2>What “supported” will mean</h2><p>A versioned build, real launch and rendering, input/focus/IME checks, assistive-technology acceptance and verified distribution. Every claim points to its tested source and host.</p><a href="#roadmap">Read the release roadmap →</a></section><section><h2>Inspect the evidence</h2><div class="doc-links">${docsLink('docs/PLATFORMS.md','Full acceptance matrix')}${docsLink('docs/receipts/ci-20261024/README.md','Hosted build results')}${docsLink('docs/receipts/ios-path-20261024/README.md','Linux input and tree receipts')}${docsLink('docs/proposals/gpui-ios-039-incompatibility.md','Upstream iOS blocker')}</div></section></div></main>`;
}
function roadmap() {
  const stages=[['01','Correctness','Close the documented component defects. Exercise controls, keyboard behavior, forms and overlays, with focused regression tests.'],['02','Desktop acceptance','Verify native macOS, Windows and Linux runtimes, IME, scaling and screen readers on identified hosts.'],['03','Developer experience','Prove fresh-project setup, finalize examples and versioned API docs, and verify distributable packages.'],['04','Public release','Review the version, package name, signing, support policy and publication details. Publish the website, crate and desktop examples.']];
  return `<main id="main" tabindex="-1" class="document">${titlePage('Desktop-first release','A useful project.<br>A dependable release.','The website makes the work discoverable. Verification, clear contracts and reliable installation make it something people can depend on.')}<div class="roadmap-list">${stages.map(([n,t,d])=>`<section><span>${n}</span><div><h2>${t}</h2><p>${d}</p></div><span class="roadmap-status">In progress</span></section>`).join('')}</div><div class="honest-note"><div><h2>Help make the next gate pass.</h2><p>Useful contributions include reproducible bug reports, desktop testing, accessibility checks and documentation fixes. Experimental mobile work remains tracked separately.</p></div><a href="${link('docs/RELEASE-READINESS.md')}">Detailed checklist ↗</a></div><div class="doc-links">${docsLink('CONTRIBUTING.md','How to contribute')}${docsLink('CODE_OF_CONDUCT.md','Code of conduct')}<a href="${repo}/issues">Issues ↗</a></div></main>`;
}
function updatePreview(changes) {
  restoreFocus = changes.state ? "#state-select" : `[data-theme="${changes.theme}"]`;
  const info=current();
  for (const [key,value] of Object.entries(changes)) info.params.set(key,value);
  location.hash=`components/${info.route}?${info.params}`;
}
function render(focus=false) {
  const info=current();
  const page=info.page==='components'?componentPage(info):info.page==='get-started'?gettingStarted():info.page==='themes'?themes(info):info.page==='platforms'?platforms():info.page==='roadmap'?roadmap():overview();
  app.innerHTML=page.replaceAll('↗', '<svg class="external-icon" viewBox="0 0 16 16" fill="none" aria-hidden="true" focusable="false"><path d="M4 12 12 4M4 4h8v8" stroke="currentColor" stroke-width="1.4"/></svg>');
  document.querySelectorAll('[data-nav]').forEach(a=>a.toggleAttribute('aria-current',a.dataset.nav===info.page));
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
  document.querySelector('#palette-select')?.addEventListener('change',event=>{location.hash='themes?palette='+encodeURIComponent(event.target.value);});
  document.querySelectorAll('.copy').forEach(button=>button.addEventListener('click',async()=>{
    try {await navigator.clipboard.writeText(button.parentElement.querySelector('code').textContent);button.textContent='Copied';document.querySelector('#announcement').textContent='Code copied to clipboard.';}
    catch {button.textContent='Select code';const selection=window.getSelection();const range=document.createRange();range.selectNodeContents(button.parentElement.querySelector('code'));selection.removeAllRanges();selection.addRange(range);document.querySelector('#announcement').textContent='Clipboard unavailable. Code selected; use your copy shortcut.';}
  }));
}
document.querySelector('.skip').addEventListener('click',event=>{event.preventDefault();document.querySelector('main')?.focus();});
window.addEventListener('hashchange',()=>{if(location.hash==='#main'){document.querySelector('main')?.focus();return;}render(true);});
render();
