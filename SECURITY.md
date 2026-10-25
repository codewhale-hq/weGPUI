# Security policy

## Supported versions

weGPUI has not published a release. Only the `main` branch is maintained. Fixes land there.

## Reporting a vulnerability

Please do **not** put exploit details in a public issue, pull request or discussion.

Use GitHub’s [private vulnerability reporting form](https://github.com/codewhale-hq/weGPUI/security/advisories/new).
Private reporting was confirmed enabled through the repository API on October 24, 2026.
You can also reach it from the repository’s **Security → Report a vulnerability** button.
The report is shared privately with repository maintainers; do not file the exploit in a public issue.

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
