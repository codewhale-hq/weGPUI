# Proposed GPUI modal accessibility-action guard

**Unapplied; additional owner approval is required.** Existing approval covers the AccessKit
cache-signal/state compatibility fixes and GPUI window-bounds hooks already vendored. It does
not authorize this new dispatch hook or the GPUI view-cache reuse change. No production/vendor
file, Cargo dependency, package version or registry file is changed by this proposal. The patches have passed `git apply --check` and Rust 1.98.1 rustfmt
parsing/format checks on separate candidate files, not compilation or runtime tests.

**Build compatibility changes:** the first-party patch calls the new
`Window::with_a11y_action_guard` API. Unpatched registry GPUI 0.3.9 does not have that method,
so the updated library would **fail to compile** against it. Keeping the package version at
0.3.9 preserves only the version label, not compatibility with the registry API. Every
consumer would need the explicit GPUI workspace-root patch, including Git/path consumers;
dependency manifests do not propagate patch tables. There is no silent fallback or feature
switch that pretends this protection exists on unpatched GPUI.

This is a stronger requirement than the existing behavior-only compatibility fixes. Current
`cargo package` listing excludes the nested vendor packages, so publication/distribution
needs an owner-approved way to supply the patched GPUI source. No published fork, package,
registry change or release is included or authorized by this proposal. The approval request
must include this consumer/build-policy effect, not describe the change as version-only parity.

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

[compat.patch](compat.patch) modifies only three files in the existing `gpui-pre 0.3.9` copy:

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
qualifies the baseline controls, not this unapplied patch. AT-active cache performance,
activation/deactivation transitions and complete cached accessibility remain unmeasured.

## First-party integration

[library-integration.patch](library-integration.patch) is separate and also unapplied:

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

## Qualification required after approval

1. Apply the exact backend and library deltas, then format/build and run all five required
   source gates. Run the new vendor regressions explicitly; dependency builds do not run them.
2. Repeat saved custom Click and built-in Focus at 0/1/20/100/500ms, plus after full close.
   Assert no owner mutation and restored opener focus; an AT true return alone is insufficient.
3. Exercise custom multiple matching listeners where the first closes or closes/reopens the
   scope; later old-session listeners must not run. Check a known removed target and Blur.
4. Verify ordinary open controls, custom caller callbacks, synthetic text selection and native
   fallback actions still work; test two windows to prove scope isolation.
5. Drive nested/deferred Modal children, rapid close/reopen, reduced motion and passive toast
   controls, including caller-supplied passive targets while background callbacks continue.
   Rerun depth-ordered three-layer trees/keyboard; no modal-ordering regression.
6. Use the cached/live probe before and after unrelated parent redraws, direct child notify,
   full refresh, AT activation after an inactive cached frame, and active→inactive transitions.
   Missing nodes/callbacks cannot count as guard acceptance. Record performance observations.

Vendor and first-party bases are in [source-bases.json](source-bases.json) and
[library-source-bases.json](library-source-bases.json). The reviewable patches
are the approval artifact; no publication, hosted fork, registry patch or extra package is
requested. Further dependency changes remain outside this proposal.
