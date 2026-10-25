# Cached accessibility replay diagnostic

This is an independent diagnostic for the pinned vendored GPUI source, before any
proposed action-guard patch. It does not contain a Modal, Presence, or guard. A
missing callback in this fixture therefore cannot demonstrate successful guarding.

## Source observations

- `src/view.rs`, `Entity<T>::cached` documents that an unnotified entity reuses its
  subtree when bounds and text style do not change. The fixture supplies definite
  240×180 cached bounds and keeps its parent labels at fixed heights.
- `src/view.rs`, `prepaint_view` selects `reuse_prepaint` for a matching cache key,
  unnotified child, and non-refreshing window. `paint_view` subsequently selects
  `reuse_paint` when no fresh element was retained.
- `src/window/a11y.rs`, `A11y::begin_frame` clears `focus_ids`, `node_bounds`,
  `action_listeners`, and the node builder each active accessibility frame.
- `src/window.rs`, `reuse_prepaint` replays hitboxes, tooltip requests, accessed
  element states, line layouts, dispatch subtrees and deferred draws. It does not
  replay accessibility nodes, focus IDs, bounds or action listeners.
- `src/window.rs`, `reuse_paint` replays cursor styles, input handlers, mouse
  listeners, accessed element states, tab stops, text layouts and scene items. It
  does not replay accessibility listeners.
- `src/elements/div.rs`, custom accessibility listeners are registered during
  actual `Interactivity::paint`, not by the replay paths.

These source observations were subsequently checked by actual runtime below.
The exact compiled source, platform module, Cargo files and hashes are retained
alongside each run.

## Fixture and collection sequence

The root compiled `cached-probe.rs.txt` in the existing external consumer, reusing
its approved Cargo files and existing `src/support/platform.rs`. The initial
callback-type compile failure and successful corrected build logs are retained.
This subtask did not run Cargo or apply the proposed vendor patch.

The left child uses `Entity<Child>::cached(StyleRefinement)` and the right child
uses an ordinary entity child. Both expose a stable named button, tracked native
focus and a custom AT Click handler. Custom clicks increment only that child's
counter and notify the child. `CHILD_RENDER`, `CUSTOM_AT_CLICK`, `PARENT_RENDER` and
`SNAPSHOT` log records distinguish callbacks from rendering.

1. Start Xvfb plus openbox, private AT-SPI-enabled D-Bus, and scale 1. Activate
   accessibility before recording the baseline. Save the tree, both node proxies,
   names/roles/paths and initial native `F3` snapshot.
2. Invoke the cached child's custom AT Click and built-in Focus; then focus the
   live child. Save callback counts, focused node and native `F3` snapshot. These
   are positive controls on the same artifact/session.
3. Press `F1` once. This increments only the parent counter and calls the parent's
   `cx.notify()`. It deliberately does not change child state, call `window.refresh`,
   or focus/blur anything. Capture the new tree and logs before any action. The
   cached child's render count should stay fixed if cache reuse actually occurred;
   the live child should render.
4. Invoke the saved cached AT Click, record method result/error and callback
   count, then invoke saved cached Focus. Save the fresh tree and `F3` native
   focus/counter snapshot. Also try the live child's action as a control. A D-Bus
   exception, disappearing target, no custom callback, and successful live callback
   are different observations; preserve each.
5. Press `F2` to notify the cached entity directly. Capture its new render and
   tree, then repeat custom Click/Focus. This is a child-notification rescue control.
   `F4` explicitly calls `window.refresh()` as a separate full-refresh rescue.
6. Repeat a parent-only redraw to establish repeatability. Preserve all failures
   and record exact action timing and binary/source hashes.

Do not focus another node or call window.refresh between the parent-only redraw
and the first missing-target/action observation: focus changes request a full
window refresh and can conceal cache reuse. Do not infer successful guarding from
the absence of callbacks. Saved Focus results remain independent of custom Click
results. The fixture does not qualify Orca speech or cross-platform behavior.

## Actual baseline collected

See `docs/receipts/overlay-contracts-20261025/custom-actions-followup/cached-baseline/`
for two actual X11 runs on unchanged approved GPUI. The immutable binary SHA-256
is `26eda739b3a8444f33c688a9a76c466a4ef37817fe067fbb4642ef44d57d53fb`.
The successful fixture source SHA-256 is
`688be4bf6989b0ea91e9b26fe1071ea9a6c99a6be839518ca6e63c5c69ec61a8`.

Both parent-only redraws in `run-2` preserve the cached render count (7→7 and 9→9)
while live renders advance (7→8 and 11→12). Fresh trees omit the cached target
while cached pixels remain visible. Saved Click/Focus calls error and do not
mutate/focus the cached child; the live control stays operable. Direct child notify
and a full window refresh each restore the target. Fresh independent clients then
successfully perform custom Click (native count 1→2) and Focus (native cached focus
true), respectively. Original saved proxies remain defunct after reappearance;
their separate errors are preserved rather than treated as failed application
rescue or successful guarding.

This confirms the preexisting cache replay gap and supports including the paired
AT-active view-cache bypass in the concrete proposal. It does not qualify the
unapplied fix, general cached rendering performance, screen-reader speech or other
hosts. No guard exists in the baseline fixture.
