# AT-SPI option-set probe

This isolated Linux/X11 fixture exercises the live AccessKit-to-AT-SPI path for select and picker lists. It checks each option's one-based `posinset`, the ancestor-derived `setsize`, and whether the disabled item is sensitive. The date picker check uses a 201-year set with only a visible window of rows and expects year 2000 to report position 101 of 201. It saves node snapshots, screenshots and source/binary hashes. It is not a screen-reader or Windows runtime test.

Run only after coordinating a native build slot:

```sh
python3 /Volumes/VIXinSSD/CW/codewhale/scripts/build-lock.py /Volumes/VIXinSSD/cw-build-cache/build.lock -- docker exec wegpui-rtl-qualification env CARGO_TARGET_DIR=/target cargo build --manifest-path /work/full-parity/set-size-probe/Cargo.toml --locked --offline
docker exec wegpui-rtl-qualification bash /work/full-parity/set-size-probe/run.sh
```

## Radio group follow-up

`RadioGroup::rows` returns loose elements and leaves their containing group to the caller. Current `FormGroup` and `Cells` examples place an "Add more" action alongside the options under the same `RadioGroup` role. AccessKit's `size_of_set_from_container` propagates an ancestor's count to *any* descendant, not only options. Setting a count on those existing outer groups can consequently give a non-option action a misleading set size; simply counting the container's rows also includes that action. Existing third-party callers can have arbitrary decoration and cannot safely be inferred.

A complete design needs an explicit option-only accessible container (or a grouped radio-row builder) whose count comes from the actual returned options, including disabled ones. Preserve the existing loose-row API for compatibility; make decorative actions accessible siblings outside that container, and preserve row spacing, separators, focus order and click handling. A shared `FormGroup`/`Cells` arrangement must correctly handle mixed rows, multiple groups and RTL without reinterpreting arbitrary children. Verify actual AT-SPI group ancestry, positions and sizes with disabled options and a live decorative action, visual parity, and Windows build/tests only. This is not implemented by the two-listbox patch.
