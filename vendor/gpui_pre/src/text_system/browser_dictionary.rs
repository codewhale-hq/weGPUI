use icu_collections::char16trie::Char16Trie;
use icu_provider::{DataError, DataPayload, DataProvider, DataRequest, DataResponse};
use icu_segmenter::{
    LineSegmenter,
    provider::{Baked, DictionaryBreakData, SegmenterDictionaryExtendedV2},
};
use zerovec::ZeroVec;

struct BrowserDictionaryProvider;

impl DataProvider<SegmenterDictionaryExtendedV2> for BrowserDictionaryProvider {
    fn load(
        &self,
        request: DataRequest,
    ) -> Result<DataResponse<SegmenterDictionaryExtendedV2>, DataError> {
        let bytes: &'static [u8] = match request.id.marker_attributes.as_str() {
            "khmerdict" => include_bytes!("browser_dictionary/khmerdict.u16le"),
            "laodict" => include_bytes!("browser_dictionary/laodict.u16le"),
            _ => return Baked.load(request),
        };
        let code_units = ZeroVec::<u16>::parse_bytes(bytes).expect("generated u16 trie bytes");
        let trie = Char16Trie::new(code_units);
        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(DictionaryBreakData::Char16Trie(trie)),
        })
    }
}

thread_local! {
    static SEGMENTER: LineSegmenter = {
        let mut segmenter =
            LineSegmenter::new_for_non_complex_scripts(Default::default()).static_to_owned();
        segmenter
            .load_dictionary_unstable(&BrowserDictionaryProvider)
            .expect("load embedded and baked line dictionaries");
        segmenter
    };
}

pub(crate) fn break_indices(text: &str) -> Vec<usize> {
    SEGMENTER.with(|segmenter| segmenter.as_borrowed().segment_str(text).collect())
}
