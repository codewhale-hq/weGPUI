# Gallery hosting

The deployable directory is `website/dist/`, including its `.nojekyll` file.
Build it with Python 3.11 or newer; the publication workflow does not invoke Rust,
Cargo, npm or a remote asset service:

```sh
python3 website/build.py
python3 -m http.server 4173 --bind 127.0.0.1 --directory website/dist
```

The default build links APIs to the exact checkout's Rust source. Generating and
including rustdoc with `--include-api` is optional for a separate local build.
The browser demonstrations and recorded comparisons do not establish native GPUI
component, screen-reader or cross-platform acceptance. Keep the developer-preview
status visible until the desktop release gates are complete.

## GitHub Pages

The published gallery is <https://codewhale-hq.github.io/weGPUI/>. The owner saved
**GitHub Actions** as the Pages source, then dispatched
[run 40211664701](https://github.com/codewhale-hq/weGPUI/actions/runs/40211664701)
from `main` at `e5b6996ca29bba628386293220fe604c6e6acf7a`. Both build and deploy
jobs succeeded. The deploy log returned that URL as its evaluated environment URL.
See [the publication receipt](../docs/receipts/pages-publication-20261025/README.md).
That receipt's cloud proxy blocked direct access to the public origin. The current local host
reaches it with HTTP 200; later deployed-origin checks retain their own publication identity.

`.github/workflows/pages.yml` is prepared for **manual publication from `main`**.
It has only a `workflow_dispatch` trigger. A push, pull request or ordinary CI run
does not trigger it. It uses the artifact deployment API and does not need a
`gh-pages` branch. Its configuration step has `enablement: 'false'`, so the workflow
cannot create a Pages site automatically.

Source pushes do not publish updates. The owner requested website updates during the current
parity continuation, authorizing this reviewed update. Other workflow dispatches require
applicable owner authorization. The existing Pages
source does not need to be configured again for each publication.

For an authorized update:

1. In [repository Pages settings](https://github.com/codewhale-hq/weGPUI/settings/pages),
   confirm **GitHub Actions** remains the build and deployment source. Leave the custom
   domain unset unless the owner has chosen one.
2. Review the `github-pages` environment under repository Settings → Environments.
   Restrict deployment branches to `main`; configure an owner reviewer if desired.
   Repository or organization Actions policies must allow the four pinned actions
   listed below and the deployment job's `pages: write` / `id-token: write` grants.
3. Open Actions → **Publish gallery to GitHub Pages** → **Run workflow**, select
   `main`, and dispatch only after confirming it still points to the approved
   commit. The build job skips other branches. This action rebuilds and publishes
   the site; it is not a preview-only run.
4. Retain the workflow run URL, checked-out commit SHA, deployment URL and browser
   verification receipt. Open a component deep link directly, reload it, and
   verify theme switching and browser interactions on the published origin.

The deployment step's returned URL is authoritative for each successful run.
Existing organization or custom-domain settings may change it. The original publication's
configuration and workflow run were performed by the owner. Subsequent authorized runs
retain separate receipts; no DNS, domain or permission change is needed for a content update.

The `build` job has repository-content read and Pages read permissions. Only the
`deploy` job receives Pages write and OIDC token permissions. Checkout does not
persist credentials. No additional repository secrets are required for standard
GitHub Pages deployment. If Pages is unavailable or disabled, configuration fails
and publication does not proceed.

### Verified action pins

The official public repositories' Git tag refs were read on 2026-10-25. The
workflow pins these exact commits rather than moving tags:

| Action | Version | Commit |
| --- | --- | --- |
| [actions/checkout](https://github.com/actions/checkout/tree/11bd71901bbe5b1630ceea73d27597364c9af683) | v4.2.2 | `11bd71901bbe5b1630ceea73d27597364c9af683` |
| [actions/configure-pages](https://github.com/actions/configure-pages/tree/983d7736d9b0ae728b81ab479565c72886d7745b) | v5.0.0 | `983d7736d9b0ae728b81ab479565c72886d7745b` |
| [actions/upload-pages-artifact](https://github.com/actions/upload-pages-artifact/tree/56afc609e74202658d3ffba0e8f6dda462b719fa) | v3.0.1 | `56afc609e74202658d3ffba0e8f6dda462b719fa` |
| [actions/deploy-pages](https://github.com/actions/deploy-pages/tree/d6db90164ac5ed86f2b6aed7e0febac5b3c0c03e) | v4.0.5 | `d6db90164ac5ed86f2b6aed7e0febac5b3c0c03e` |

### Observed access

The connected GitHub tool reported this public repository's default branch as
`main`, `has_pages: false`, an unset repository homepage and repository permissions
including `admin: true` and `push: true` on 2026-10-25. Pages was therefore not
enabled at that earlier observation. The owner's subsequent configuration and
successful run supersede that historical state. The available
connector tools expose repository reads and source operations, but no Pages
settings or workflow-dispatch operation. The cloud runtime reports no configured
outbound identity or secret binding. Existing shell credential failures do not
justify changing authentication or creating an account. Subsequent publication
still needs the owner's authenticated GitHub session or an authorized connection
with workflow-dispatch capability. The current local host has an authenticated GitHub CLI
with workflow scope; it can perform an owner-authorized dispatch without changing settings.

## Base paths and other static hosts

The same output supports a domain root (`/`) or a project prefix (`/weGPUI/`).
Assets and local links use relative paths, and navigation uses URL fragments,
such as `/weGPUI/#components/button_default`. The fragment stays in the browser,
so reloading a component needs only the site's `index.html`; no SPA rewrite,
absolute base URL or history fallback is required. Retain the trailing slash on
the site base URL and all files under `website/dist/`.

For Cloudflare Pages, the corresponding build command is
`python3 website/build.py`, the repository root is the build root, and the output
directory is `website/dist`. Select a Python 3.11+ build image. Cloudflare normally
serves at the project origin's root; no base-path edit is needed. Git integration
may enable automatic deployments, so review that setting with the owner before
connecting a repository. No Cloudflare account, project, token or deployment is
configured by these files, and no Cloudflare tool is available in this session.
Choose one approved host before publishing.
