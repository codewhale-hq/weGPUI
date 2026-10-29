# Mac desktop component collection — 2026-10-29

This slice adds an original Rust/GPUI desktop collection and nine gallery routes.
The visual structure follows the observed WeChat for macOS 4.2.0.9. Desktop surfaces use sampled Mac reference colors after sRGB normalization. Shared
control accents and states use the existing WeUI 2.6.26 theme. `DesktopPalette` follows
the existing global theme; it adds no independent appearance state.

## Source references

- [IUpdatable/WeUiSharp](https://github.com/IUpdatable/WeUiSharp/tree/974b51e63e33131b45fa6f5f47de7c7d15f495da)
  at `974b51e63e33131b45fa6f5f47de7c7d15f495da`: Button, ToggleButton, Field,
  ComboBox, ContextMenu, MessageBox and Window control/state contracts. Its README
  declares MIT. No WPF code, icons or artwork are redistributed.
- Installed WeChat for macOS, version 4.2.0.9: light and dark chat, General,
  Shortcuts, Notifications, contacts, chat info, emoji and history surfaces.
  Private screenshots remain in the local CW reference artifact and are excluded
  from this public repository. The app was observed back in Dark Mode afterward.
- Existing WeUI token extraction in `src/theme/palette.rs`, sourced from the
  pinned `weui@2.6.26` package. [Color mapping and API](../../DESKTOP.md).

The desktop icons are original geometric SVGs. Public examples use synthetic people
and messages. Media previews use the existing licensed WeUI sample images and the
platform emoji font. WeChat's proprietary stickers and personal app content are absent.

## Native captures

`captures.json` records 46 native captures: 45 on macOS and the dark context menu
in the Linux ARM64 VM. The 23 scenes cover light and dark at
1120 × 760 logical pixels / 2240 × 1520 raster pixels. It records the binary hash,
host, command arguments, image hashes and source-manifest hash. The VM menu sample
records its own platform and binary identity; its earlier Mac capture omitted the popup
and was rejected. `vm/` adds a supplemental care-mode capture from the VM. `native-source.json`
records the exact library, example, vendor and Cargo inputs. The capture runner checked
that these inputs and the built binary stayed unchanged throughout the matrix.

| Route | States |
| --- | --- |
| desktop-chat | default, empty |
| desktop-controls | default, disabled, error |
| desktop-messages | default |
| desktop-contacts | default |
| desktop-settings | default, shortcuts, notifications, appearance |
| desktop-overlays | default, menu, dialog, emoji, toast |
| desktop-history | default, files, media, links, empty |
| desktop-chat-info | default |
| desktop-favorites | default |

The website checks this matrix independently of the original mobile WeUI and older
wide gallery matrices. It identifies the Mac collection, its source and its capture host.
The website's nine Mac examples use desktop captures; the 62 existing routes retain
their separate phone/desktop evidence.

## Verification scope

The focused native build, existing library/gallery tests, Clippy, visual review,
interaction observations and website checks are recorded in `verification.json`.
Screenshots demonstrate rendering at their recorded source. They do not establish
complete WeChat parity, live messaging, VoiceOver, real IME acceptance or support on
every operating system. Calling, recording and external transport remain application
responsibilities. No hosted native CI or client release qualification is claimed. The owner authorized
a website update after this pre-publication validation; the existing manual
[Pages workflow](https://github.com/codewhale-hq/weGPUI/actions/workflows/pages.yml)
records the deployment identity and result separately.
