# P1 accessibility fixes — Linux X11 + AT-SPI, 2026-10-24

Debian 13, rustc 1.98.1, Xvfb :98, `scripts/linux/session.sh` (at-spi2-core, python3-pyatspi).

## Set positions were off by one (fixed)

AccessKit 0.24 `position_in_set` is zero-based: `accesskit_atspi_common 0.19.1` and
`accesskit_windows 0.34.0` both add 1 when exporting. weGPUI passed `i + 1` in radio, select
menu options, article lists, form-preview items and list tips, so the first radio was announced
"2 of …". Before (`ec40ab8` baseline build and `c744f1d`): `radio button 'Email' … posinset:2`.
After: `posinset:1` / `posinset:2` (`component-states-atspi.txt`).

## Disabled buttons on Linux (upstream limitation, not fixed)

`button 'Unavailable'` (a disabled cell button) is exported without DISABLED. The node does
carry AccessKit `disabled`, but `accesskit_atspi_common 0.19.1` maps disabled to "not
sensitive" only for roles that support read-only; buttons always get Enabled|Sensitive.
Disabled switches are reported correctly (`form_switch`: `toggle button '标题文字' DISABLED`,
identical on the baseline build). The button stays unfocusable and carries the "不可用"
description, which Orca reads. macOS AX reports it disabled (review-20261024). Fixing this
needs an upstream AccessKit change; no dependency patch was made.
