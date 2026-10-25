# Interactive browser examples

This iframe runs browser controls using pinned WeUI 2.6.26 CSS and local JavaScript adapters.
It does not run Rust GPUI components in the browser. The website's separate native view retains
the recorded GPUI captures, source links and acceptance limits.

All 60 example templates come from the proper files imported by the pinned package's
`src/example/index.html`. The official index shell, company/contact/registration footer,
WeChat SDK, `example.js` and analytics code are omitted. Original templates are preserved as
plaintext `source/{route}.html.txt`; the runtime bundle removes scripts, inline handlers and
external navigation. Component interactions and accessibility adapters are in `bridge.js`.
Uploads create local object URLs and never configure a transport. The iframe CSP blocks
connections and form submissions and permits only local scripts/assets and local image data.

`vendor/PROVENANCE.json` records source/package versions, tarball digests, individual asset
hashes and per-template transformations. Both npm tarballs and every selected package file are
verified when regenerating. WeUI, WeUI.js and Zepto license notices are bundled locally.
Zepto's full MIT license is from its immutable v1.2.0 source tag; its bundled header is retained.

The normal build is `python3 website/build.py --out OUTPUT` and uses checked-in static files.
It requires no Cargo, Rust toolchain, npm installation or network access. Rust API docs are an
optional separate prebuilt input. To regenerate the vendor data, first fetch the verified npm
packages with the repository's upstream tooling, obtain the pinned Zepto license, then run:

```sh
python3 website/playground/vendor_upstream.py \
  --weui PATH/weui/package --weui-js PATH/weui.js/package \
  --zepto-license PATH/zepto-1.2.0-MIT-LICENSE
```

Use `playground/index.html#ROUTE?theme=light&state=default&care=0&brand=` at either a site root
or a project prefix such as `/weGPUI/`. Empty brand preserves the pinned palette. Care mode is
the upstream contrast palette; it does not independently enlarge text. A custom six-digit brand
color derives light/dark tint variants without replacing default colors unless selected.

The parent may send `{type:'wegpui:configure', config:{route,state,theme,care,brand}}` through
`postMessage` with the current origin. The iframe checks both origin and parent window. It
emits `wegpui:ready`, `wegpui:route`, `wegpui:state` and `wegpui:error`. Appearance-only changes
preserve the mounted DOM and input values. Route/state changes reset the example. Readiness is
available as `body[data-route][data-state][data-ready=true]` and the normalized current config
as `window.WEGPUI_PLAYGROUND_CONFIG`.

Building and local preview do not publish or deploy the site.
