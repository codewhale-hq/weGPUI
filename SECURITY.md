# Security policy

## Supported versions

weGPUI has not published a release. Only the `main` branch is maintained. Fixes land there.

## Reporting a vulnerability

Please do **not** put exploit details in a public issue, pull request or discussion.

A private security contact has not been designated yet, and private vulnerability reporting has
not been confirmed for this repository. This is an open maintainer decision recorded in
`docs/STATUS.md`. Until it is resolved:

1. If GitHub shows a **Report a vulnerability** button on the repository's *Security* tab, use it.
2. Otherwise, open a public issue that says only that you have a security report and need a
   private channel. Include no technical details. A maintainer will arrange a private channel.

Include the affected commit, platform, a minimal reproduction and the impact you observed.

## Scope

weGPUI is a UI component library. Areas where a defect could have security impact include:

- text input: clipboard handling, IME text, password fields (which must not copy or cut);
- image sources: `ImageSource` paths and URIs passed to `UploaderFile`/`Gallery`, which GPUI
  loads on the application's behalf;
- the gallery's file chooser flow (`prompt_for_paths`);
- development scripts that download upstream resources (`scripts/weui_upstream.py` verifies
  SHA-256 and refuses unsafe archive members).

Vulnerabilities in GPUI or other dependencies should be reported to those projects. We are glad
to help coordinate when weGPUI is affected.
