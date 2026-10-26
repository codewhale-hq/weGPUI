# Independent desktop completion work — 2026-10-25

This continuation fixes concrete source/distribution gaps while preserving the owner's hold
on the logical-wrap prototype. **The project is not fully accepted or release-ready.** Native
macOS/Windows, hardware, complete reader/language matrices and publication decisions remain
in [the completion checklist](../../COMPLETION.md). No commit, push, deployment or publication
was made in this continuation.

## Source and isolation

Base: canonical GitHub `main` at `b1ddcae2b37b1901c17c78c6f06d9f4224ce03c2`, re-fetched without
movement. The independent native snapshot is that base plus only these changes:

- `src/components/footer.rs`: at least 44×44 link targets, wrapping link rows, host-supplied
  bottom safe-area padding, and an opaque page surface for fixed footers so covered content
  does not show through the clickable area.
- `src/components/swiped.rs`: neutral actions explicitly use the theme foreground instead of
  inheriting white text from their parent.
- `src/components/agree.rs`: agreement text can shrink/wrap and the checkbox does not shrink.
- `src/components/switch.rs`: the existing Rust 1.95 duration assertion allows a one-nanosecond
  rounding difference; production animation is unchanged. The original failure is retained
  in the [logical-wrap receipt](../gpui-logical-wrap-20261025/README.md).
- `examples/application.rs`: an application-owned draft with name validation, bounded notes,
  controlled consent/updates, theme switching, local image choosing/preview/deletion, modal
  confirmation, reset and focus-aware scrolling.
- `scripts/package_preview.py` and six artifact-contract tests: a reproducible native source
  bundle with a separate consumer, all four root Cargo patches, integrity/source manifests,
  and the project/vendor licences and provenance.

The preview artifact excludes the held step 2 vendor and row-consumer changes. Its exact selected
source is in [preview-source.json](preview-source.json); selected code/tooling hashes are in
[source-checksums.txt](source-checksums.txt). No dependency versions or requirements changed.
The generated consumer adds only its own package entry to the supplied lockfile. All five held
vendor files still match the supplied prototype byte-for-byte. Step 3 was not started.

Separate website changes fix the existing build/checker failure caused by treating the
native-only `languages` page as an upstream browser route. `website/inventory.py` is shared
by the builder and interactive checker. Only that documented exception is excluded; unknown
routes still cause inventory validation to fail. Five regression tests include a complete
real-site build under isolated Python. The Linux CI job now runs them before building the site.
The initial expanded check failed with `AssertionError: Coverage differs from the Rust gallery`
([original output](working-tree-gates/website-build.log)). The first helper import also failed
under isolated Python; loading the helper by its explicit path fixes that invocation, and a
real `python -I` build is now a regression test. These website changes are outside the native
preview artifact.

## Build and test results

Host: Debian 13 x86_64 containers, Rust/Cargo 1.95.0. Native checks use Xvfb, Openbox, Mesa
software Vulkan, Noto/DejaVu fonts, AT-SPI and a GTK desktop portal. Browser checks use
Playwright 1.64.1 and Chromium 156.0.8078.47. These are Linux results.

| Check | Result |
| --- | --- |
| `cargo fmt --all -- --check` | pass |
| `cargo build --all-targets --locked --offline` | pass |
| `cargo test --locked --offline` | 59 library unit tests pass; 3 doc examples ignored |
| `cargo clippy --all-targets --locked --offline -- -D warnings` | pass |
| `RUSTDOCFLAGS='-D warnings -W missing-docs' cargo doc --no-deps --locked --offline` | pass |
| `python3 -W error -m unittest discover -s scripts/tests -v` | 16 pass, including 6 new preview-bundle tests |
| `python3 -W error -m unittest discover -s website/tests -v` | 5 pass, including an isolated-Python build with all 528 historical capture assets |
| Working-tree logical-wrap / word-break harnesses | 16 + 4 + 7 pass; held source unchanged |
| `node --check website/app.js` | pass |
| Website builds without and with API documentation | 60 browser routes and 132 recorded states; pass |

The independent extracted workspace and separate consumer pass the five source gates
([gates](gates/summary.txt)). The combined working tree, including the held candidate, also
passes all five gates and existing harnesses ([working-tree gates](working-tree-gates/summary.txt)).
Website build/unit logs are in [website-checks](website-checks/inventory-tests.log).

The first independent rustdoc attempt failed because the new binary crate lacked a crate-level
description ([failure](initial-gates/doc.log)); its module documentation was added, and the
final full run passes. A shared Cargo target can retain artifacts across extracted snapshots
whose reproducible timestamps are identical: changed workspace packages were cleaned before
the final build to force recompilation. The packaged library's non-root patch/profile warnings
are expected; the generated workspace applies the four patches at its own root.

## Native application run

[The final run](application/result.log) drives real X11 input, keyboard focus/reveal and AT-SPI
activation with resulting state assertions. To repeat, copy the driver, launcher and image fixture
to a new empty output directory, set `CARGO_TARGET_DIR` to the rebuilt preview workspace target,
and run the launcher with X11/AT-SPI/GTK portal tools installed. It verifies:

- Required-name validation and InvalidEntry state, then actual typed text.
- Terms activation does not toggle consent; consent changes independently.
- Review hides background nodes, Cancel dismisses, reopening/Confirm increments the count once.
- Light/dark switching; reset from the edge of the footer target clears the draft without
  changing the confirmed count.
- Real GTK portal image selection, image preview, Escape dismissal, and deletion back to zero.

The captured executable's hashes before and after the session match each other and the final
gate executable. Images include [confirmed](application/confirmed.png),
[dark](application/dark.png), [selected image](application/image-selected.png) and
[preview](application/image-preview.png). The example keeps drafts in memory and performs no
upload transport.

Setup failures are retained in [failed-attempts](failed-attempts/offscreen-action-failure.log):
input sent before a real native click, a capture launched before its rebuilt binary existed,
and a GTK chooser that had not received focus. A Tab-only focus assertion for Upload after
reset also failed in an early driver run; the final chooser path uses real pointer input.
This is not a claim of complete keyboard, OS IME or screen-reader acceptance. The final recipe
activates the native window first and gives the GTK dialog focus before entering the file path.

## Eight-mode component checks

A separate small [consumer probe](components/contracts.rs) uses the same hashed library source.
It deliberately gives the neutral action a white-text parent to exercise the inheritance bug.
Light/dark × normal/care × default/custom brand all pass
([results](components-final/results.json), [run](components-final/result.log)):

- The short footer link's actual accessibility/pointer bounds are 44×44; a click near its edge
  invokes the callback once.
- Bottom padding measures 40px: the normal fixed-footer 16px plus the supplied 24px inset.
- The long agreement fits in 343px at a 375px viewport, wraps to 80px high, and contains its
  separate terms link.
- Neutral text is visible despite its white parent. The pixel probe finds opaque-glyph contrast
  from 12.12:1 to 17.94:1 against the sampled background. It measures peak glyph contrast,
  not the minimum across antialiased edges or arbitrary palettes.

Representative captures: [light](components-final/light.png),
[dark care](components-final/dark-care.png). All eight machine checks do not establish full
component, contrast, touch, reader, hardware or other-host acceptance.

## Browser verification

The complete existing Chromium interaction suite passes **21 checks at each mount**, at `/`
and `/weGPUI/`, including 360 route/viewport observations across the two mounts. It covers
route loads, overlays, controls, local media, themes, draft preservation, shell integration
and responsive theme pages. Full results and immutable built-file hashes are in
[browser](browser/verification/root/browser-checks.json). This checks the separate HTML/JS
implementation, not native GPUI. Deployed-origin access, other engines and actual readers
remain open. No site publication was dispatched.

## Preview artifact and remaining limits

Artifact: `wegpui-preview-83739a2a1608.tar.gz`, SHA-256
`1770950a88a747b152ef422f73b1119ae610a20d8c3614db15aff928cc2a275a`
([checksum](preview-sha256.txt)). The source and manifests are described in
[PREVIEW-DISTRIBUTION](../../PREVIEW-DISTRIBUTION.md). Artifact tests verify reproducibility
across checkout locations/timestamps, inclusion of all four patches and licences, rejection
of missing vendors/symlinks/unsafe paths, and detection of changed archive contents.

The source bundle is a local developer preview, not a signed installer or a publication of the
patched dependency graph. The owner hold, wrapped RTL caret/selection, date/localisation policy,
native-speaker review, full component/reader/platform matrices, native signing/installers,
live-host verification and release-policy decisions remain in [COMPLETION](../../COMPLETION.md).
No component row is promoted to accepted. Absolute host paths in retained logs are replaced
with placeholders; screenshots and source hashes retain their original meaning. Browser traces
remain local execution artifacts rather than committed receipt data.
