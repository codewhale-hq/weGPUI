---
name: weGPUI
description: Native Mac desktop components and a distinct WeUI mobile collection, presented through a restrained visual catalog.
colors:
  shared-brand: "#07c160"
  shared-brand-care-light: "#018942"
  desktop-selected-light: "#1cac72"
  desktop-selected-dark: "#0da869"
  desktop-sent-light: "#95ec69"
  desktop-sent-dark: "#3eb575"
  solid-black: "#000000"
  solid-white: "#ffffff"
  desktop-canvas-light: "#fafafa"
  desktop-canvas-dark: "#1e1e1f"
  desktop-list-light: "#eeeef0"
  desktop-list-dark: "#2f2f30"
  desktop-pinned-light: "#e6e6e8"
  desktop-pinned-dark: "#3b3b3d"
  desktop-rail-light: "#e3e4e6"
  desktop-rail-dark: "#2a2c2d"
  desktop-panel-dark: "#29292a"
  desktop-text-light: "#1a1a1c"
  desktop-text-dark: "#e0e0e4"
  desktop-support-light: "#626266"
  desktop-support-dark: "#aaaab0"
  desktop-placeholder-dark: "#b7b7bd"
  desktop-field-dark: "#454547"
  desktop-border-light: "#ededed"
  shared-hover-light: "#ececec"
  shared-hover-dark: "#373737"
  shared-link-light: "#576b95"
  shared-link-dark: "#7d90a9"
  shared-danger-light: "#c84040"
  shared-danger-dark: "#fb7373"
  shared-unread: "#fa5151"
  catalog-paper: "#fff"
  catalog-wash: "#f7f7f7"
  catalog-ink: "rgba(0, 0, 0, .9)"
  catalog-body: "rgba(0, 0, 0, .7)"
  catalog-muted: "rgba(0, 0, 0, .6)"
  catalog-line: "rgba(0, 0, 0, .1)"
  catalog-focus: "#059a4c"
  catalog-selected: "#b4ecce"
  catalog-selected-ink: "#191919"
  code-background: "#1e1e1e"
  code-text: "#ddd"
typography:
  native-title:
    fontFamily: ".SystemUIFont"
    fontSize: "15px"
    fontWeight: 500
  native-body:
    fontFamily: ".SystemUIFont"
    fontSize: "14px"
    lineHeight: 1.5
  native-control:
    fontFamily: ".SystemUIFont"
    fontSize: "13px"
    lineHeight: 1.3
  native-field:
    fontFamily: ".SystemUIFont"
    fontSize: "13px"
    lineHeight: 1.2
  native-support:
    fontFamily: ".SystemUIFont"
    fontSize: "12px"
  native-caption:
    fontFamily: ".SystemUIFont"
    fontSize: "11px"
  catalog-display:
    fontFamily: '-apple-system, BlinkMacSystemFont, "Segoe UI", "Noto Sans", sans-serif'
    fontSize: "clamp(38px, 4.6vw, 62px)"
    fontWeight: 550
    lineHeight: 1.15
    letterSpacing: "-.035em"
  catalog-heading:
    fontFamily: '-apple-system, BlinkMacSystemFont, "Segoe UI", "Noto Sans", sans-serif'
    fontSize: "24px"
    fontWeight: 550
    lineHeight: 1.15
    letterSpacing: "-.025em"
  catalog-body:
    fontFamily: '-apple-system, BlinkMacSystemFont, "Segoe UI", "Noto Sans", sans-serif'
    fontSize: "13px"
    lineHeight: 1.85
  catalog-label:
    fontFamily: '-apple-system, BlinkMacSystemFont, "Segoe UI", "Noto Sans", sans-serif'
    fontSize: "12px"
  code:
    fontFamily: '"SFMono-Regular", Consolas, "Liberation Mono", monospace'
    fontSize: "11px"
    lineHeight: 1.85
rounded:
  native-small: "4px"
  native-control: "5px"
  native-tabs: "6px"
  native-menu: "7px"
  native-composer: "8px"
  native-dialog: "10px"
  catalog-preview: "12px"
spacing:
  tight: "4px"
  small: "5px"
  control-gap: "7px"
  inset: "8px"
  row-gap: "10px"
  message-gap: "12px"
  section: "24px"
  catalog-inset: "32px"
  catalog-mobile-inset: "18px"
components:
  desktop-button-primary:
    backgroundColor: "{colors.shared-brand}"
    textColor: "{colors.solid-black}"
    typography: "{typography.native-control}"
    rounded: "{rounded.native-control}"
    padding: "0 12px"
    height: "34px"
  desktop-button-filled:
    backgroundColor: "{colors.desktop-pinned-light}"
    textColor: "{colors.desktop-text-light}"
    typography: "{typography.native-control}"
    rounded: "{rounded.native-control}"
    padding: "0 12px"
    height: "34px"
  desktop-button-quiet:
    textColor: "{colors.desktop-text-light}"
    typography: "{typography.native-control}"
    rounded: "{rounded.native-control}"
    padding: "0 12px"
    height: "34px"
  desktop-button-danger:
    backgroundColor: "{colors.desktop-pinned-light}"
    textColor: "{colors.shared-danger-light}"
    typography: "{typography.native-control}"
    rounded: "{rounded.native-control}"
    padding: "0 12px"
    height: "34px"
  desktop-selection-reference:
    backgroundColor: "{colors.desktop-selected-light}"
    textColor: "{colors.solid-white}"
    typography: "{typography.native-control}"
    rounded: "{rounded.native-control}"
    padding: "0 12px"
  desktop-field:
    backgroundColor: "{colors.desktop-pinned-light}"
    textColor: "{colors.desktop-text-light}"
    typography: "{typography.native-field}"
    rounded: "{rounded.native-control}"
    padding: "0 9px"
    height: "30px"
  desktop-switch:
    backgroundColor: "{colors.shared-brand}"
    width: "36px"
    height: "21px"
  desktop-message-received:
    backgroundColor: "{colors.desktop-list-light}"
    textColor: "{colors.desktop-text-light}"
    typography: "{typography.native-body}"
    rounded: "{rounded.native-control}"
    padding: "8px 12px"
  desktop-message-sent:
    backgroundColor: "{colors.desktop-sent-light}"
    textColor: "{colors.solid-black}"
    typography: "{typography.native-body}"
    rounded: "{rounded.native-control}"
    padding: "8px 12px"
  desktop-attachment:
    typography: "{typography.native-control}"
    width: "236px"
  catalog-category-selected:
    backgroundColor: "{colors.catalog-selected}"
    textColor: "{colors.catalog-selected-ink}"
    typography: "{typography.catalog-label}"
    rounded: "{rounded.native-control}"
    padding: "6px 11px"
---

# Design System: weGPUI

## Overview

**Creative North Star: "The Native Mac Component Catalog"**

The native desktop collection uses the compact geometry, pane relationships and system typography of a modern Mac chat interface. Quiet neutral surfaces carry conversations, controls and content; green identifies a small set of meaningful states. The mobile collection retains its WeUI identity. Both collections follow the existing global `Theme`, while desktop surfaces keep their own sampled Mac roles.

The website is a restrained visual catalog. Real native renders, readable labels and source material carry its character. Its white canvas and modest controls give the component examples room to be inspected. The original desktop collection and the WeUI mobile ports remain identifiable within the same catalog.

This records the implemented system in [DesktopPalette and controls](src/desktop/controls.rs), [messages](src/desktop/messaging.rs), [panels](src/desktop/panels.rs), [icons](src/desktop/icons.rs), [native composition](examples/gallery/pages/macos.rs) and [catalog styles](website/style.css). The sampled surface map and compatibility limits live in [DESKTOP.md](docs/DESKTOP.md); the full mobile palette remains in [tokens.json](docs/tokens.json). This is a visual reference, not a statement of complete platform or accessibility acceptance.

**Key Characteristics:**

- Compact Mac controls within clearly differentiated panes.
- Sampled desktop neutrals with shared WeUI control accents.
- Native system type, small radii and original geometric icons.
- A quiet catalog that presents actual native component captures.

## Colors

The palette pairs finely separated Mac greys with purposeful green states; the catalog uses white, a light wash and subdued black text.

### Primary

- **Shared WeUI Green** (`shared-brand`): primary buttons, enabled switches and selected rail icons. The light care palette uses `shared-brand-care-light`.
- **Mac Selection Green** (`desktop-selected-light`, `desktop-selected-dark`): the selected conversation or selection-style control. Its hue is distinct from the shared primary-control green.
- **Mac Sent Green** (`desktop-sent-light`, `desktop-sent-dark`): outgoing message bubbles. Received bubbles use the list surface.
- **Catalog Selection Wash** (`catalog-selected`): selected categories and catalog navigation, paired with `catalog-selected-ink`.

### Secondary

- **Slate Link** (`shared-link-light`, `shared-link-dark`): links and attachment icon accents.
- **Destructive Red** (`shared-danger-light`, `shared-danger-dark`): destructive text and invalid field presentation.
- **Unread Red** (`shared-unread`): compact unread-count badges; the badge foreground is chosen for contrast.

### Neutral

| Desktop surface role | Light token | Dark token |
| --- | --- | --- |
| Canvas | `desktop-canvas-light` | `desktop-canvas-dark` |
| Conversation list and received bubble | `desktop-list-light` | `desktop-list-dark` |
| Pinned conversation | `desktop-pinned-light` | `desktop-pinned-dark` |
| Navigation rail | `desktop-rail-light` | `desktop-rail-dark` |
| Panel | `desktop-canvas-light` | `desktop-panel-dark` |
| Primary text | `desktop-text-light` | `desktop-text-dark` |
| Supporting text | `desktop-support-light` | `desktop-support-dark` |
| Field placeholder | `desktop-support-light` | `desktop-placeholder-dark` |
| Field | `desktop-pinned-light` | `desktop-field-dark` |
| Border | `desktop-border-light` | `desktop-panel-dark` |

The catalog uses `catalog-paper` for the page, `catalog-wash` for preview wells, `catalog-ink` for headings, `catalog-body` for prose and `catalog-muted` for supporting labels. `catalog-line` supplies quiet dividers. Code panes use `code-background` and `code-text`. These website roles do not replace the native desktop surface map.

Primary-button and outgoing-bubble foregrounds are chosen as black or white by the native contrast calculation. Ordinary selected conversation rows preserve the explicitly user-approved Mac reference treatment: white titles with partially transparent white previews and timestamps. That treatment is below normal-text AA (4.5:1). Care mode uses black selected-row text and opaque preview text. The `desktop-selection-reference` entry records this specific compatibility treatment; it is not a foreground rule for new surfaces. Supporting and placeholder colors have been raised above the sampled reference values for legibility. No full AA claim is made, including for arbitrary host brand overrides.

Custom branding follows `Theme::set_brand`: the shared accent and selected-row color change, and the sent-bubble color is composited through the existing theme surface logic. Do not replace the sampled neutral roles during that operation. Demo avatar colors identify synthetic people and are not palette primitives. Sidecar tonal strips are generated inspection aids, not additional runtime colors.

**The Sampled Surface Rule.** Keep canvas, list, rail, pinned, panel, field and bubble roles distinct using the current `DesktopPalette`; mobile surface aliases do not replace the sampled Mac map.

**The One Theme Rule.** Light, dark, care and brand changes resolve through the existing global `Theme`; component collections do not create independent appearance state.

## Typography

**Native Font:** the platform system UI family (`.SystemUIFont` in the native gallery shell).

**Catalog Font:** the system sans-serif stack recorded in the frontmatter.

**Code Font:** the SF Mono, Consolas and Liberation Mono stack recorded in the frontmatter.

**Character:** Native type is compact and functional, with hierarchy carried mainly by size and placement. The website keeps the same familiar system character, adds larger catalog headings and reserves monospace for source, commands and API identifiers.

### Hierarchy

- **Native title:** medium text for the chat header. Settings section headings use a larger native step (18px), and dialog titles use 17px.
- **Native body:** message content and the base desktop composition. Message text uses the recorded 1.5 line height; conversation names use the same body size.
- **Native control and field:** compact button, menu and field text, with the separate line heights recorded above.
- **Native support and caption:** previews, descriptions, authors and status text. Small timestamps and the composer keyboard hint use 10px; that is a metadata treatment, not the default reading size.
- **Catalog display:** the overview title; the narrowest stylesheet breakpoint sets it to 42px. Component-page titles use 34px, becoming 30px on narrow screens.
- **Catalog heading, body and label:** short headings, restrained prose and tool controls. Reading prose is bounded to approximately 70–72 characters; the caption allows 75 characters.
- **Code:** source panes use the recorded compact monospace role and remain scrollable.

**The Native Type Rule.** Use the platform UI family for native controls and content; use monospace for code and identifiers.

## Layout

Native chat is a pane composition: a fixed navigation rail (60px), a resizable conversation list and a flexible transcript. The gallery starts its list at 270px and constrains it to 230–360px; the reusable splitter has configurable limits. The chat header is 52px high. The optional information panel is 268px wide. Below a native viewport width of 760px, chat shows one pane and supplies a Conversations action. Settings switch to their compact sidebar and inset treatment below 600px.

Dense controls use small recurring gaps and insets from the spacing scale. Standard desktop buttons have a minimum height of 34px; compact buttons use 28px. Fields are 30px high. Conversation rows have 10px vertical content padding, with the avatar separated from text by 10px. The transcript uses 24px padding and 22px between message groups. Bubble content is bounded to 78% of its message row. Keep these density choices scoped to the native desktop collection; the mobile WeUI collection retains its own dimensions.

The website gallery has a centered maximum width of 1392px with 32px horizontal padding. Its desktop layout uses three columns, two at the 799px breakpoint and one at 559px. Desktop Mac capture cards preserve the complete native composition with a contained image. Phone layout uses its separate capture crop and grid rules. Narrow page padding steps through 24px to 18px.

The current overview opens with the Mac collection, then exposes the WeUI collection through the same search and category controls. Component pages connect a native capture to its actual Rust source and run command. The website labels captured native output separately from browser-rendered interactive WeUI references. The component explorer uses a 225px catalog rail, reduces it to 200px at 1199px and replaces it with a collapsible catalog below 799px. Desktop capture and source stack vertically; phone capture/source and comparisons adapt at the existing breakpoints.

## Elevation & Depth

Native panes and messages are flat. Depth comes from the measured surface differences, separators and restrained borders. Floating menus receive a soft ambient shadow; dialogs use the existing modal layer. Keyboard focus uses two-color bands on controls whose background is variable. The catalog relies on white and wash surfaces, with a small lift on the selected segmented control.

### Shadow Vocabulary

- **Native menu:** `0 6px 24px rgba(0, 0, 0, 0.2)` separates a floating menu from its parent surface.
- **Native inset focus:** black and white inset bands with spreads of 4px and 2px, respectively.
- **Native outer focus:** the same two bands outside small switches.
- **Catalog selected segment:** `0 1px 3px #00000012` distinguishes the selected segment within its wash track.

Catalog thumbnail motion uses a short transform transition (240ms) and link-reveal opacity (150ms). The desktop thumbnail layout suppresses zoom. Reduced-motion preference removes website transitions. Native desktop popovers explicitly use the existing motion-free presentation.

**The Surface Before Shadow Rule.** Separate persistent native panes with their surface roles and dividers; reserve ambient shadow for floating layers.

## Shapes

Native controls, avatars and message bodies share a small rounded rectangle (5px). Checkboxes use smaller corners; keycaps and image thumbnails use the small shape. Tabs, menus, the composer and dialogs step outward through the recorded radius scale. Switches have a rounded track (12px radius) and a circular thumb. Unread badges are pill-shaped.

Message bubbles have a small outward SVG tail colored from the same surface as the bubble. Incoming and outgoing tails mirror one another; they are part of the message silhouette. Quotes use an inset rule, and translations use an internal top divider. Original desktop icons use a common 24-unit SVG view box, rounded strokes and joins, and a stroke width of 1.65. Their normal size is 22px; controls select smaller sizes where needed.

The website uses softly rounded capture wells from the catalog preview radius. Content labels sit beneath previews instead of becoming badges inside the images. Borders remain fine and quiet.

## Components

### Buttons

Compact rectangles with a clear hierarchy and native input states.

- **Quiet:** transparent at rest with primary text; selected quiet controls use the accent foreground.
- **Filled:** field surface and primary text.
- **Primary:** shared brand fill with a calculated black or white foreground.
- **Danger:** field surface with destructive text.
- **Selection:** selected-row surface and the scoped reference/care foreground behavior described under Colors.
- **States:** selected and primary buttons reduce opacity on hover; other buttons use the shared hover surface. Active opacity is 0.65. Disabled or loading opacity is 0.4 and activation is unavailable. Focus-visible uses the existing two-color inset indicator.
- **Geometry:** the standard button has 12px horizontal padding and a 7px internal gap. Icon-only buttons are square at their standard or compact height.

### Inputs / Fields

Dense native fields with real editing behavior.

`DesktopField` wraps the existing text editor in the field surface, small corners and a fine frame. Focus changes the frame to the accent; invalid state uses destructive color and a supporting error label. Search adds an original search icon. Clear appears only when the field has editable content. The composer uses the existing multiline editor inside a larger rounded frame with a toolbar, keyboard hint and send control. Its blank-draft state disables Send. These visual wrappers retain the native editor and IME path.

### Choices and Tabs

Small controlled components, with selection shown through state rather than extra decoration.

The desktop switch uses a compact track and a white circular thumb; unchecked fill uses supporting text color, checked fill uses the accent. The checkbox pairs a small square indicator with a label and the same accent. Both expose toggled and disabled state. Desktop tabs place selection-style buttons in a softly rounded panel track. Website categories use a light green wash with dark selected text; the website's segmented controls use a white selected segment within a wash track.

### Navigation

A quiet rail and scannable conversation rows.

The native rail uses original icons with the active item tinted by the shared accent. Conversation rows combine an avatar, one-line title, truncated preview, timestamp and optional unread, pinned and mute states. Pinned and selected rows have separate surface roles. The narrow chat composition supplies a navigation action instead of squeezing every pane. Catalog navigation uses compact text rows and a light selection wash, with a collapsible list on small screens.

### Cards / Containers

Content sits in its parent surface without a generic card treatment.

Attachments pair a thumbnail or original file-type icon with a title and supporting detail, bounded to the component's width. They can be composed inside an application action. Settings use divided rows with labels, descriptions and caller-supplied controls. Media grids and contact/member cells reuse avatars, image thumbnails and the existing viewer. Website capture wells frame recorded renders; their labels and state counts remain outside the image.

### Messages

Readable message groups with direction carried by alignment, color and silhouette.

`MessageBubble` reverses the row for outgoing content and aligns incoming content with its avatar. Optional author and status lines sit outside the bubble. Quotes, translations and attachments compose inside the same message body. The tail follows direction and matches the bubble fill. Sent-message foreground follows the existing contrast calculation.

### Menus and Dialogs

Floating layers with the existing focus and dismissal authority.

Menus use compact icon, label and shortcut rows with a soft ambient shadow. Destructive and disabled actions use their existing states. Dialogs use the canvas surface, larger corners and right-aligned action groups. Popovers and dialogs use the existing `Presence` and `Modal` mechanisms; applications supply open state and actions. Preserve keyboard focus, Escape dismissal and background isolation when composing them.

## Do's and Don'ts

### Do:

- **Do** use the sampled Mac surface roles for the desktop collection and the existing WeUI palette for the mobile collection.
- **Do** keep both collections connected to the existing global Theme, including light, dark, care and brand state.
- **Do** retain native system typography, compact component geometry and the original SVG icon vocabulary.
- **Do** preserve focus, disabled, selected, error and loading presentation when composing reusable controls.
- **Do** keep incoming and outgoing bubble tails aligned with their direction and colored from the bubble surface.
- **Do** describe the ordinary selected-row white text as the approved reference compatibility treatment and expose the care-mode alternative.

### Don't:

- **Don't** replace sampled desktop surfaces with generic green-tinted or mobile surface mappings.
- **Don't** apply the selected-row contrast exception as a general legibility rule or claim full AA acceptance.
- **Don't** turn synthetic avatar colors into new theme tokens.
- **Don't** replace the original geometric SVG icons with text glyph approximations.
- **Don't** add persistent pane shadows where the existing surface and divider hierarchy provides separation.
