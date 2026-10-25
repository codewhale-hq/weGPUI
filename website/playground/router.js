/* Local route/theming shell. Component interactions live in bridge.js. */
(() => {
  'use strict';
  const routes = window.WEGPUI_ROUTES;
  const bridge = window.WeGPUIPlayground;
  const container = document.getElementById('container');
  const status = document.getElementById('playground-status');
  const initialBodyChildren = new Set(document.body.children);
  const palette = matchMedia('(prefers-color-scheme: dark)');
  let config, feedbackTimer, feedbackFrame;

  function notify(type, extra = {}) {
    if (parent !== window) parent.postMessage({ type: `wegpui:${type}`, config: { ...config }, ...extra }, location.origin);
  }

  function clearFeedback() {
    clearTimeout(feedbackTimer);
    cancelAnimationFrame(feedbackFrame);
    status.textContent = '';
  }

  function announce(message) {
    clearFeedback();
    feedbackFrame = requestAnimationFrame(() => {
      status.textContent = String(message);
      feedbackTimer = setTimeout(() => { status.textContent = ''; }, 4000);
    });
    notify('state', { message: String(message) });
  }

  function fromHash() {
    const [route = 'home', query = ''] = location.hash.slice(1).split('?');
    const params = new URLSearchParams(query);
    return { route: route || 'home', state: params.get('state') || 'default',
      theme: params.get('theme') || 'light', care: params.get('care') === '1', brand: params.get('brand') || '' };
  }

  function normalize(value) {
    const route = Object.hasOwn(routes, value.route) ? value.route : 'home';
    const state = routes[route].states.includes(value.state) ? value.state : 'default';
    return { route, state, theme: ['light', 'dark', 'system'].includes(value.theme) ? value.theme : 'light',
      care: value.care === true || value.care === 1 || value.care === '1',
      brand: typeof value.brand === 'string' && /^#?[\da-f]{6}$/i.test(value.brand) ? value.brand.replace('#', '').toLowerCase() : '' };
  }

  function appearance() {
    const dark = config.theme === 'dark' || (config.theme === 'system' && palette.matches);
    document.body.dataset.weuiTheme = dark ? 'dark' : 'light';
    if (config.care) document.body.dataset.weuiMode = 'care';
    else delete document.body.dataset.weuiMode;
    const names = ['BRAND', 'BRAND-100', 'BRAND-80', 'BRAND-90', 'BRAND-120', 'BRAND-170',
      'BRAND-BG-100', 'BRAND-BG-90', 'BRAND-BG-110', 'BRAND-BG-130'];
    names.forEach(name => document.body.style.removeProperty(`--weui-${name}`));
    if (!config.brand) return;
    const rgb = [0, 2, 4].map(index => parseInt(config.brand.slice(index, index + 2), 16));
    const mix = amount => '#' + rgb.map(channel => Math.round(amount >= 0 ? channel + (255 - channel) * amount : channel * (1 + amount)).toString(16).padStart(2, '0')).join('');
    const values = { BRAND: mix(0), 'BRAND-100': mix(0), 'BRAND-80': mix(dark ? .2 : -.2),
      'BRAND-90': mix(dark ? .1 : -.1), 'BRAND-120': mix(dark ? -.2 : .2), 'BRAND-170': mix(dark ? -.7 : .7),
      'BRAND-BG-100': mix(0), 'BRAND-BG-90': mix(dark ? .1 : -.1), 'BRAND-BG-110': mix(dark ? -.1 : .1), 'BRAND-BG-130': mix(dark ? -.3 : .3) };
    Object.entries(values).forEach(([name, value]) => document.body.style.setProperty(`--weui-${name}`, value));
  }

  function navigate(route, state = 'default') {
    if (!Object.hasOwn(routes, route)) return;
    configure({ ...config, route, state }, 'route');
  }

  function render() {
    document.body.dataset.ready = 'false';
    clearFeedback();
    bridge.cleanup();
    [...document.body.children].forEach(node => { if (!initialBodyChildren.has(node)) node.remove(); });
    const template = document.createElement('template');
    template.innerHTML = routes[config.route].html;
    container.replaceChildren(template.content.cloneNode(true));
    const page = container.querySelector('.page');
    if (!page) throw new Error('The route has no page');
    page.classList.add(config.route, 'js_show');
    page.dataset.route = config.route;
    page.scrollTop = 0;
    bridge.mount({ route: config.route, page, config, announce, navigate });
    bridge.applyState(config.state);
    document.body.dataset.route = config.route;
    document.body.dataset.state = config.state;
    document.body.dataset.ready = 'true';
    document.getElementById('playground-loading').hidden = true;
    document.title = `weGPUI browser gallery — ${config.route}`;
  }

  function configure(value, event = 'configured') {
    try {
      const next = normalize({ ...config, ...value });
      const remount = !config || next.route !== config.route || next.state !== config.state;
      config = next;
      window.WEGPUI_PLAYGROUND_CONFIG = Object.freeze({ ...config });
      appearance();
      const params = new URLSearchParams({ theme: config.theme, state: config.state, care: config.care ? '1' : '0' });
      if (config.brand) params.set('brand', config.brand);
      history.replaceState(null, '', `#${config.route}?${params}`);
      if (remount) render();
      notify(event, { routes: Object.keys(routes) });
    } catch (error) {
      document.body.dataset.ready = 'false';
      document.getElementById('playground-loading').hidden = true;
      const message = document.createElement('section');
      message.className = 'playground-error';
      const title = document.createElement('h1'); title.textContent = 'Example unavailable';
      const detail = document.createElement('p'); detail.textContent = 'Choose another component from the catalog.';
      message.append(title, detail); container.replaceChildren(message);
      notify('error', { message: String(error.message || error) });
      console.error('Browser gallery route failed:', config?.route, error);
    }
  }

  window.addEventListener('message', event => {
    if (event.source !== parent || event.origin !== location.origin || event.data?.type !== 'wegpui:configure') return;
    if (!event.data.config || typeof event.data.config !== 'object') return;
    configure(event.data.config);
  });
  window.addEventListener('hashchange', () => configure(fromHash(), 'route'));
  palette.addEventListener('change', () => { if (config.theme === 'system') appearance(); });
  document.addEventListener('click', event => {
    const link = event.target.closest('a');
    if (!link || event.defaultPrevented) return;
    const href = link.getAttribute('href') || '';
    if (href.startsWith('#') && Object.hasOwn(routes, href.slice(1))) {
      event.preventDefault(); navigate(href.slice(1));
    } else if (/javascript:\s*home\s*\(/i.test(link.dataset.originalHref || '')) {
      event.preventDefault(); navigate('home');
    } else if (href === '#' || /^(?:https?:)?\/\//i.test(href)) event.preventDefault();
  });
  document.addEventListener('submit', event => event.preventDefault());
  configure(fromHash(), 'ready');
})();
