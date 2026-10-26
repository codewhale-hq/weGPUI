# Current-source Layers Orca sample

Orca 48.1, AT-SPI2 2.56.2, Speech Dispatcher 0.12.0 and eSpeak NG 1.52.0 run against the
final gallery binary identified in `verification.json`. English, light appearance, X11 scale 1
and reduced motion: twelve native navigation/selection/return steps and eight time-bounded
speech windows pass. The selected control retains focus, inactive layer controls disappear,
and Orca's current flat-review object command emits the expected name plus button role.

`verification.json` correlates native timestamps, actual Orca SPEECH OUTPUT records, focus and
exposed controls. `speech.wav` wraps real ALSA file PCM from the synthesizer: mono S16_LE at
22050 Hz, 38.6264 seconds, nonzero samples. The Speech Dispatcher log confirms that sample
format, rate, channel count and eSpeak initialization. The null slave does not preserve
wall-clock pauses, so this is not an audio-timing test. No phrases or prerecorded sound were
injected. Nobody has accepted the audio by listening; this bounded sample does not complete
reader, appearance, language, timing or other-host acceptance.

The host initially lacked reader tools. Official Debian packages were extracted into an
isolated tool directory; package archive names and SHA-256 identities are retained in
`tooling/package-archives.json`. Python, typelib, shared-library, synthesizer-data and audio-plugin
paths point there. Only the generated Orca asset/table prefix configuration was relocated;
its patch is retained. No application/vendor source or Rust dependency changed for the reader run.
`session.sh` and `tooling/environment.sh` use evidence/tool-directory placeholders for reproduction.

The initial setup lacked an exported tool-root variable. A separate version-only attempt
lacked a schema compiler and stalled waiting for its accessibility bus; its exact container
was stopped. The actual reader session starts the bus before Orca and uses existing system
schemas. These setup attempts are not passes. The first complete collection used Insert+KP_Enter,
which is Orca's title command; its log/result are retained under `attempt-title-command/`.
The final driver uses held Insert+KP_Begin, and the debug log confirms the actual current
flat-review object handler. The final session and verifier exit zero.
