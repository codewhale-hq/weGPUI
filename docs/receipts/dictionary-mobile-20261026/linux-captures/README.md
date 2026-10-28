# Linux dictionary and paired captures

These observations use Linux ARM64, X11 and software rendering. The capture
manifest records the dirty source as parent `901f770`; each recorded source hash
was independently checked against committed source `c343621`, and each frozen
gallery hash matched its manifest. The reference is WeUI 2.6.26 at the revision
in each manifest.

Both 320x812 and 375x812 matrices captured 56 of 56 pairs. All 224 native and
reference PNG files matched their recorded SHA-256 hashes. Only representative
light form images are copied here; the full captures remain in the local artifact
directory. These are unreviewed captures, not visual parity verdicts.

The dictionary fixture compares four languages at four widths. Fourteen of 16
normalized row sequences match; Khmer at 225 pixels and Myanmar at 160 differ.
The full comparison data, probe and driver are retained here. The fixture uses
a controlled Noto Sans Lao browser font to align native and browser advances;
the host's default browser font differs. See the parent receipt for details.
