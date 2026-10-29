# Mac desktop components

The desktop collection provides reusable Rust/GPUI components shaped around the current
Mac chat interface. It complements the mobile WeUI ports in `wegpui::components`.
Its layout follows the observed WeChat 4.2.0.9 Mac app; its control contracts are informed
by WeUiSharp. Appearance follows the same global `Theme` as the rest of weGPUI. Desktop surfaces
are sampled from the Mac reference; shared control accents and states use WeUI tokens.

```sh
cargo run --example gallery -- --page desktop-chat --theme light --size 1120x760
cargo run --example gallery -- --page desktop-settings --state shortcuts --theme dark
cargo run --example gallery -- --page desktop-history --state media
```

Use the example selector at the top of the native window to move between Chat, Controls,
Messages, Contacts, Settings, Menus & dialogs, Chat history, Chat info and Favorites.
`All components` returns to the complete native catalog. The website shows recorded native
renders and the actual Rust source; it does not execute GPUI in the browser.

## Reusable API

All types below are exported from `wegpui::desktop`. Initialize weGPUI and its assets as
described in the main README. Wrap the application root in `accessibility::ModalScope`.
The [complete gallery composition](../examples/gallery/pages/macos.rs) compiles against
these public components.

| Area | Components | State and composition |
| --- | --- | --- |
| Controls | `DesktopButton`, `DesktopSwitch`, `DesktopCheckbox`, `DesktopField`, `DesktopSelect`, `DesktopTabs`, `DesktopBadge`, `DesktopKeycap` | Controlled values and callbacks; compact, selected, disabled, loading and error presentation |
| Shell | `DesktopRail`, `DesktopSplit`, `ChatHeader`, `ConversationRow` | Caller supplies navigation, pane width, selected conversation, unread count, pin/mute state and header actions |
| Identity | `DesktopAvatar`, `ContactRow`, `ContactProfile`, `DesktopMemberGrid` | Initials, caller images, grouped avatars, contact fields and member cells |
| Messages | `MessageBubble`, `MessageQuote`, `MessageTranslation`, `MessageAttachment` | Incoming/outgoing layouts, author/avatar, arbitrary bubble children, quotes, translations and status text |
| Composer | `DesktopComposer` | Existing `Entity<TextArea>`, caller toolbar and send callback; input uses the existing GPUI editor and IME path |
| Settings | `DesktopSettings`, `DesktopSettingRow` | Caller navigation, sections, labels, descriptions and arbitrary controls |
| Layers | `DesktopMenu`, `DesktopMenuItem`, `DesktopPopover`, `DesktopDialog`, `DesktopNotice` | Existing `Presence`/`Modal` focus and dismissal authority; caller stores open state and handles actions |
| Media | `DesktopEmojiPicker`, `DesktopMediaGrid`, `DesktopMediaTile`, `DesktopEmptyState` | Caller emoji set, image assets, actions, captions, durations and empty content |
| Icons | `DesktopIcon`, `DesktopIconKind` | Original geometric SVGs, 35 named shapes, adjustable size/tint, no additional asset registration |

`DesktopButtonStyle` provides Quiet, Filled, Primary, Danger and Selection treatments.
Loading and disabled buttons expose their state and do not activate callbacks. Switches
and checkboxes expose a toggled state. Fields retain native editing, clear and focus behavior;
an error marks the existing input invalid. `DesktopSelect` is a controlled trigger: supply
the current value, expanded state and `on_open`, then compose a menu in a popover.

Keep IDs stable and distinct among repeated siblings. `MessageBubble::new(id)` requires an
ID for that reason. Applications own message transport, persistence, files, recording,
calling, playback, translation and locale-specific wording. Attachment content is visual;
compose it inside a `DesktopButton` to attach an application action.

## Colors

`DesktopPalette::get(cx)` follows the existing global `Theme`; it adds no second
appearance global. Light/dark mode, care mode and custom brand changes propagate to
both collections. Desktop surfaces use the measured Mac colors rather than mobile
surface aliases. The current reference was normalized to sRGB before sampling.

| Desktop role | Light | Dark |
| --- | --- | --- |
| Canvas | `#fafafa` | `#1e1e1f` |
| Conversation list / received bubble | `#eeeef0` | `#2f2f30` |
| Pinned conversation | `#e6e6e8` | `#3b3b3d` |
| Navigation rail | `#e3e4e6` | `#2a2c2d` |
| Panel | `#fafafa` | `#29292a` |
| Selected row | `#1cac72` | `#0da869` |
| Primary text | `#1a1a1c` | `#e0e0e4` |
| Supporting text | `#626266` | `#aaaab0` |
| Field placeholder | `#626266` | `#b7b7bd` |
| Field | `#e6e6e8` | `#454547` |
| Border | `#ededed` | `#29292a` |

Shared primary buttons, switches and selected rail icons use WeUI `brand`; hover,
destructive text, links and unread badges use `bg_color_active`, `red_80`, `link`
and `red`. A host brand override also replaces selected-row color. Sent bubbles
use the Mac light/dark green treatment, or a host brand composited over the canvas.

Text on primary buttons, avatars and badges uses a contrasting black or white foreground.
Selected conversation rows retain the explicitly approved Mac reference’s white text
treatment in ordinary mode. That reference treatment is below 4.5:1; care mode uses
black selected-row text and an opaque preview foreground. Supporting and placeholder
text are raised above the reference values for legibility.
Demo avatar colors identify synthetic people; they are content, not design tokens.

## Interaction and limits

The chat example filters conversations, changes selection, edits a multiline draft, inserts
emoji or sample attachments, and appends sent messages in memory. Enter inserts a new line;
Command+Enter sends while the composer is focused, with a marked-text guard. The send button
is unavailable for a blank draft. Sidebar dividers respond to dragging and Left/Right keys;
Home/End select their configured limits. Narrow chat windows show a single pane with a
Conversations button.

Menus support Tab, Up/Down, Enter/Space and Escape through GPUI and the existing modal
focus system. Settings switches and selections are local controlled values. History filters
and search operate on sample records; media tiles open the existing WeUI image viewer.
Calls, microphone recording, forwarding and external transport are visibly disabled where
the gallery has no application handler. Sample data resets when an example is rebuilt.

The collection is a developer preview, not a complete WeChat client. Current native captures
and bounded interaction evidence are in the [Mac component receipt](receipts/macos-components-20261029/README.md).
Complete reader, IME, platform and release acceptance remain separately tracked.
