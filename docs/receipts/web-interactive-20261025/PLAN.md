# Interactive browser gallery verification

This is a browser WeUI playground. It does not run the Rust GPUI controls in the browser and
does not replace the retained native captures, runtime matrices or desktop acceptance gaps.
No deployment or hosted workflow is authorized by this verification task.

The validator will use the existing Playwright Python environment and installed Chromium;
the observed browser is Chromium 156.0.8078.47 on Debian 13. It will retain the actual built-site
manifest, validator source, source status/hash, viewport/configuration, console/network errors,
action outcomes, screenshots and trace. Failed attempts remain in their original directories.

## Route, hosting and layout contracts

- Load every one of the 60 route IDs independently, with the local template actually mounted.
  Report load failures, JavaScript/console errors, failed HTTP responses and outside-network
  requests. All requests must stay local apart from in-memory data/blob preview URLs.
- Serve the real build at `/` and as the only site under `/weGPUI/`, so absolute asset paths
  cannot accidentally pass a project-prefix test against a fallback root site.
- Check responsive overflow at 375, 768 and 1280px. Retain representative small/wide captures
  for inputs, navigation, content, overlays and media; a route count is not visual acceptance.
- Exercise shell-to-iframe selection/configuration and deep links, dark/care/brand changes,
  reload and browser history. Verify the actual frame receives the selected route/configuration.

## User-flow checks

| Family | User outcome |
| --- | --- |
| Buttons | Pointer and keyboard activation produce feedback; disabled/loading controls cannot activate. |
| Dialog / ActionSheet / Half-screen dialog | Open with an actual trigger; Tab/Shift+Tab stay in the active dialog; Escape/Cancel close it and restore that trigger. |
| Picker | Change a real selection, cancel without applying, reopen and confirm; disabled options cannot become the selected result. |
| Form / Input | Type and edit real HTML input/textarea values; exercise validation, clear/refocus, native readonly/disabled guards and UTF-16 maxlength. Browser text insertion/composition events are labeled separately from a real OS IME. |
| Choices / Agree / Vcode | Pointer and keyboard change the intended checkbox/radio/switch; independent groups retain state. Terms links do not toggle consent. Countdown activation is guarded and its state changes are visible. |
| Navbar / Tabbar | Pointer/arrow/Home/End selection updates both selected tab and visible panel, with focus retained on the tab. |
| Search | Activate, type, submit, clear and cancel; feedback and focus match each action. |
| Slider / Progress | Keys and pointer change the adjustable slider; unavailable slider stays unchanged. Progress starts, advances and cancels through the visible controls. |
| Uploader / Gallery | Select an in-memory local PNG, open its real preview, dismiss, then delete. No external upload/HTTP post is allowed. |
| List / Panel / Grid / Footer | Actual available links/actions navigate locally or produce their documented feedback; swipe actions reveal/close through keyboard and pointer. |
| Toast / Information bar | Feedback becomes visible, announces through the status region and dismisses without stealing input focus. |
| Article / Flex / Badge / Icons / Loading / Loadmore / Steps / Preview / Message / Layers | Inspect real content, decoded assets and layout at narrow/wide widths; scroll long content and exercise any supplied links. Static examples are explicitly recorded as presentational, without inventing callback acceptance. |

Assertions will observe values, selected panels, focus, dialogs, previews and user-visible
feedback rather than merely checking that the implementation created a named element. The
complete native, screen-reader, cross-platform and all-state parity requirements remain open.
