# Read-only proposal review

The runtime collector and two independent source reviewers examined the existing public
APIs. No generic library-only interception path was found for direct custom listeners or
built-in Focus/Blur. Registering another listener only appends it; propagation flags are not
consulted by the dispatcher. Hiding the next tree cannot block the reproduced pre-update calls.

Review of the unapplied deltas found and corrected a missing test-local Rc import. The final
formatted patch captures real/synthetic target guards and registration guards, consumes
rejected actions, rechecks before each matching callback, snapshots both deferred phases and
the reuse clone, and aligns both cache-bypass conditions. The separate id/role-less wrapper
includes its child's own root and preserves public Presence's Send/Sync traits with shared
atomic state. Passive own targets are guarded while page targets remain outside the scope.

No further blocker was found for the documented normal closed/reopened-session mechanics.
This is source review and formatting/apply-check evidence, not a successful compile or runtime
qualification. Scope restoration follows existing normal-return drawing contracts; no panic
recovery is claimed. General topmost-modal authorization is explicitly outside this change:
naively ANDing an outer hidden flag would also reject its active inner modal.

The cached/live baseline was separately reviewed against native render/callback counters,
fresh trees and independent-client rescue controls. The new cache-bypass implementation has
not run; baseline target disappearance is a defect, not successful guarding.
