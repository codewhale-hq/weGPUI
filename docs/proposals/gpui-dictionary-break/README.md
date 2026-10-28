# Dictionary line breaking: dependency decision

Status: owner-approved on 26 October 2026. The owner approved ICU4X dictionaries, then
explicitly authorized dependencies needed to finish parity. Implementation and native review
are in progress; this decision does not establish acceptance.
The existing [text plan](../gpui-text-i18n/README.md) originally put “A new shaper, ICU,
hyphenation” outside its approved continuation. The earlier `unicode-linebreak` approval
did not select new dictionary/data package versions; the owner's subsequent ICU approval
and dependency authorization supersede that restriction.

## Concrete change

Replace only the shared `LineWrapper::break_indices` provider with ICU4X's dictionary line
segmenter. Its byte indices already fit the four consumers repaired by the Unicode batch:
estimated fragment wrapping/truncation, monotonic shaped rows and logical RTL rows. Keep
existing object-offset mapping, shaped geometry, emergency wrapping and bidi resolution.
The replacement makes the direct `unicode-linebreak` edge redundant; its transitive use stays.

Use `icu_segmenter = "=2.4.0"`, `default-features = false`, `features = ["compiled_data"]`.
`LineSegmenter::new_dictionary(Default::default()).segment_str(text)` supplies the shared
opportunities; omit the initial zero index to preserve the current provider contract.
This includes local dictionaries for Thai, Lao, Khmer and Myanmar, with no runtime network,
LSTM feature, provider inference or service requirement.

The standalone dependency-resolution review copies the current lock graph and adds only:

| New package | Version |
| --- | --- |
| `icu_segmenter` | 2.4.0 |
| `icu_segmenter_data` | 2.4.0 |
| `icu_locale_fallback` | 2.4.0 |
| `icu_locale_fallback_data` | 2.4.0 |

No existing package version changes in that resolution; [the delta](dependency-delta.json)
records the result. Existing ICU collections/provider/core dependencies are already 2.4.0.
The segmenter declares Rust 1.88 and Unicode-3.0 licensing, compatible with the project's
Rust 1.95 declaration. Its official crate archive is SHA-256
`61b195dc605f81aa181bec9b3bd14e26cc71b903e4644f6af578ca9772c7a710` (4,474,652 bytes).
This source-archive size is not the linked binary size; measure linked growth before landing.

## Verification required after approval

Review the locked graph and licences, migrate the extracted harness build to the resolved
ICU dependency graph, and run the existing cases plus the native multilingual fixture.
Compare long dictionary-script text against Chromium at narrow/wide sizes, retain actual
break strings and captures, and repeat affected macOS/Linux and Windows build/tests gates.
Keep browser/Unicode-version tailoring differences explicit. Do not infer a real IME,
native-speaker or screen-reader pass from correct automatic line breaks.

Primary sources: [ICU4X segmenter API](https://docs.rs/icu_segmenter/2.4.0/icu_segmenter/struct.LineSegmenter.html),
[official version metadata](https://crates.io/api/v1/crates/icu_segmenter/2.4.0).
The versioned crate manifest/source and registry checksum were read for this decision.

## Chromium dictionary continuation

The preceding dependency decision describes the initial stock ICU4X implementation.
The later continuation uses a custom ICU4X data provider with Chromium's abridged
Khmer and Lao word lists and retains ICU4X's baked Thai/Myanmar dictionaries and
line rules. Source licenses, exact hashes, and regeneration instructions are in
[`vendor/gpui_pre/src/text_system/browser_dictionary`](../../../vendor/gpui_pre/src/text_system/browser_dictionary/README.md).
An isolated provider prototype matched the observed Chromium opportunities for
one Khmer corpus and left the other three sampled scripts' opportunities unchanged.
That comparison is not full browser or language acceptance; the ICU4X matching
algorithm differs from Chromium's patched ICU4C implementation.
