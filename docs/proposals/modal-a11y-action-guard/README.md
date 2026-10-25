# Proposed GPUI modal accessibility-action guard

**Owner-approved and applied on 2026-10-25.** The owner explicitly approved this dispatch
hook, GPUI view-cache reuse change and mandatory consumer build effect. The recorded patches
were applied on `3840b47` and formatted with Rust 1.98.1. Combined-source five gates and the
sampled Linux timing/caller/cache checks now pass; the full upstream GPUI Cargo test build
remains blocked by its missing test setup. Exact results and limits are in the
[applied-source receipt](../../receipts/overlay-contracts-20261025/action-guard-fixed-20261025/README.md).
No package version or dependency edge was added by the guard subtask.

**Build compatibility changes:** the first-party patch calls the new
`Window::with_a11y_action_guard` API. Unpatched registry GPUI 0.3.9 does not have that method,
so the updated library **fails to compile** against it. Keeping the package version at
0.3.9 preserves only the version label, not compatibility with the registry API. Every
consumer needs the explicit GPUI workspace-root patch, including Git/path consumers;
dependency manifests do not propagate patch tables. There is no silent fallback or feature
switch that pretends this protection exists on unpatched GPUI.

This is a stronger requirement than the existing behavior-only compatibility fixes. Current
`cargo package` listing excludes the nested vendor packages, so publication/distribution
needs an owner-approved way to supply the patched GPUI source. No published fork, package,
registry change or release was authorized. The owner approved the required workspace-root
patch/build-policy effect; distribution/publication decisions remain separate.

The current closing guard misses queued accessibility actions. Actual Linux X11 runs on
`49268de` retain a SwipedCell proxy, close its surrounding Modal by native mask click, and then
invoke the saved action. At requested 0/1ms, custom Click calls owner code with Presence false
and steals restored focus. Built-in Component.grabFocus at 0/1ms separately steals restored
focus. At 20/100/500ms the adapter rejects removed objects instead. Raw callbacks, timestamps,
focus trees, controls and failures are in
[the runtime receipt](../../receipts/overlay-contracts-20261025/custom-actions-followup/README.md).
These are actual AT-SPI requests, not screen-reader acceptance.

## Why a library-only wrapper is insufficient

- `Window::handle_a11y_action` dispatches private saved listeners directly before its built-in
  fallback. It does not dispatch a mouse/key event or consult propagation flags for those
  listeners. `Window::on_a11y_action` only appends listeners; no public filter/remove API exists.
- Built-in Focus reads the private focus-id map and calls `window.focus`; Blur unconditionally
  calls `window.blur`. Mouse capture cannot intercept either path.
- Semantics can hide the next accessibility tree. The reproduced event runs before that tree
  replaces the already-painted action/focus maps, so a hidden-state change alone is too late.
- Opt-in guards inside each library callback would protect only known handlers. They cannot
  generically protect arbitrary caller children or the built-in Focus route.

Independent source reviews reached the same conclusion. The bounded extra backend API below
makes the existing live model predicate available at the central dispatch boundary.

## Exact proposed changes

The approved [compat.patch](compat.patch) modifies only three files in the existing `gpui-pre 0.3.9` copy:

| File | Change |
| --- | --- |
| `src/window.rs` | Add `Window::with_a11y_action_guard(Rc<dyn Fn() -> bool>, f)`; retain predicate references on registered listeners; reject unknown/inactive targets before custom or built-in dispatch; recheck before every matching callback; preserve guard scopes through deferred prepaint/paint/reuse. |
| `src/window/a11y.rs` | Capture inherited live predicate chains on real nodes and synthetic leaves; require all predicates and a currently known target; clear metadata with each frame; add two focused model regressions. |
| `src/view.rs` | Bypass explicit view-cache reuse while accessibility is active in both request-layout and paint; keep ordinary caching when inactive. |

The predicates are per Window, side-effect-free and evaluated at action time. A refused action
is consumed; it never falls through to synthetic Click, Focus or Blur. Closing a scope
prevents later callbacks for its guarded targets or registered within that scope from firing.
Registered callbacks also retain their own scope, including a callback manually registered
for a target outside it.

Capturing only listener guards would leave built-in Focus/Blur open. Capturing only node
guards would leave manually registered callbacks' registration scopes unrepresented. Both
are included. `A11yNodeBuilder::push` and `push_leaf` capture target metadata centrally, so
there is no additional element-lifecycle hook and synthetic text children are covered too.

DeferredDraw stores an immutable vector snapshot of predicate references, restores it during
both phases, and copies it when replaying deferred draws. It does not retain a mutable stack
pointer or an evaluated bool. Nested chains compose with AND; changing any live ancestor
immediately makes its already-painted descendants inactive. Context restoration follows the
existing Window drawing-scope normal-return contract; this proposal introduces no panic
recovery guarantee.

## Cache bound

The current code clears a11y nodes/listeners/focus maps in `A11y::begin_frame`, while cached
prepaint/paint replay does not reconstruct those maps. Simply adding a guard stack would not
make a cached descendant's disappearing callback a successful guard test. The bounded patch
therefore disables that reuse only while a11y is active, using the same two existing cache
bypass sites as the inspector. Changing only request-layout could leave paint trying to
unwrap missing cache state; both sites must change together.

[cache-source-evidence.md](cache-source-evidence.md) records exact source paths and control
flow. The root built [cached-probe.rs.txt](cached-probe.rs.txt) in a separate existing-API
consumer, immutable executable SHA-256
`26eda739b3a8444f33c688a9a76c466a4ef37817fe067fbb4642ef44d57d53fb`.
The [actual cached baseline](../../receipts/overlay-contracts-20261025/custom-actions-followup/cached-baseline/README.md)
reproduces accessible-node/action loss while cached pixels stay visible. Two parent-only
redraws leave cached render counts at 7 and 9 while live counts advance 7→8 and 11→12.
Fresh trees are captured before actions/focus. Direct child notify restores a target that a fresh client
can Click (native count 1→2); full refresh restores a target that a fresh client can Focus
(native cached focus=true). Old proxies remain defunct and their errors are preserved.

This confirms the existing replay omission and justifies the paired AT-active bypass. It
qualifies the baseline controls, not the newly applied patch. AT-active cache performance,
activation/deactivation transitions and complete cached accessibility remain unmeasured.

## First-party integration

The approved [library-integration.patch](library-integration.patch) is recorded separately:

- A role/id-less AnyElement wrapper places the child's **own root node**, prepaint, synthetic
  children and paint inside the backend scope. Forwarding the child's role/id from Semantics
  would create its own node before the scope starts, leaving root Focus unguarded.
- Every Modal wraps only its own deferred layer with a predicate reading the existing shared
  Presence input flag plus a new shared opening epoch. Presence remains Send+Sync and its
  existing public generation counter/API is preserved.
- Closing flips the existing flag immediately. Reopening updates the epoch, so listeners from
  the previous painted session stay rejected until the new session's frame registers them.
  This does not attach a generation to incoming adapter requests: after a stable NodeId is
  reused by a newly painted session, requests address that current node.
- Passive overlays guard their own closed/reopened targets too. Background page targets stay
  outside that scope, so page actions continue; the wrapper adds no focus, modal or hidden state.
  Any enclosing modal lifecycle guards still apply. Nested open modals inherit open ancestor
  lifecycle guards; closing an ancestor rejects all its descendants.

This fixes **closed/reopened-session dispatch**, not general topmost-modal authorization.
An outer modal stays logically open beneath an inner one. Naively ANDing every ancestor's
hidden/topmost flag would also reject the active inner modal. Pre-frame inner-show ordering
and arbitrary external deferred priorities remain separate contracts; neither is claimed fixed.

## Applied-source qualification

| Check | Result |
| --- | --- |
| Combined root gates | PASS: formatting, all targets, 47 root unit tests/one ignored doctest, strict Clippy and rustdoc, Rust 1.98.1 locked/offline. |
| Full vendored Cargo unit build | BLOCKED: published GPUI test setup lacks font fixtures and optional test-support dependencies; no extra packages/features/lock changes added. |
| Two new metadata regressions | PASS in a separate source-extraction harness using existing rlibs; exact NodeBuilder/test text with documented std map/String aliases and debug metadata disabled. This does not replace the full vendor suite. |
| Saved custom Click | PASS at 0/1/20/100/500ms: no closed callback, opener restored, normal/reopen controls work. |
| Saved built-in Focus | PASS at 0/1ms, plus after full close; no restored-focus theft. |
| Caller callbacks / epoch | PASS: raw caller handler, multiple matching listeners, same-request close and close→reopen suppress old-session callbacks; new session works. |
| Passive / nested / windows | PASS: passive own targets rejected while page callbacks continue; closed ancestor/live-child inputs rejected; independent window callbacks continue. |
| AT-active cache | PASS: parent-only redraw keeps both nodes, retained/fresh Click/Focus work; old baseline failure and the fixed harness's initial WM-startup failure retained. |

These results are source, interface and native state observations. Full screen-reader/human
listening, cross-host qualification, arbitrary palettes/themes/states and hardware remain
separate. The input guard does not establish topmost-modal authorization or automatic child
lifecycle/announcement behavior. AT activation/deactivation cache transitions and performance
remain unmeasured. Blur uses the same source preflight but has no dedicated Linux AT request
fixture in this run; no full accessible-action matrix is claimed.

Vendor and first-party bases are in [source-bases.json](source-bases.json) and
[library-source-bases.json](library-source-bases.json). The recorded patches are
the approved change artifacts. No publication, hosted fork, registry patch or extra package
was authorized. Further dependency changes remain outside this approval.
