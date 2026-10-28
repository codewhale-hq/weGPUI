# Picker and select accessibility set sizes — 2026-10-28

The picker and select listbox containers now expose their complete option count to
AccessKit. This lets platform adapters derive each option's set size, including
disabled choices and options outside a virtualized date picker's visible window.
The tested product source is `7880c37062188643540efef515d0a3a13fce670c` plus the
two `aria_size_of_set` additions in this commit; `set-size-source.json` records
the tested file hashes.

On Linux X11, the synthetic [fixture](probe-README.md) exercised the live
AccessKit-to-AT-SPI tree: all three select options and picker options reported
positions 1–3 and size 3, including the disabled middle choice; year 2000 in
a virtualized 1900–2100 picker reported 101 of 201. The JSON snapshots and
probe source are included. The macOS library suite passed 54 tests. These
are sampled automated checks, not a screen-reader, Windows UI, or full
accessibility acceptance. The existing radio rows lack an option-only
container; assigning the surrounding mixed container a count would also
mislabel decorative actions, so radio set sizes remain open.
