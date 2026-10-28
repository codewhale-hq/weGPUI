# Chromium Khmer and Lao dictionary data

These word lists are from Chromium's ICU source, revision
`19dfb44a62ae60ce6b0060de8c0cef5d5c527177`, selected by Chromium
156.0.8078.47. Chromium describes this as ICU4C 78.2 with an abridged Khmer
word list and an older abridged Lao word list. The files retain their original
copyright and contributor notices, and `LICENSE-ICU` is copied from that exact
source revision. The full Unicode license is also retained at the repository root.

| File | SHA-256 |
| --- | --- |
| `khmerdict.txt` | `d06150c5e27b1852f89198de7e84f5b71fb9250f0735acd6d0526f343226bed5` |
| `laodict.txt` | `d2c4f10b0e80abaa59ef7dcdac0d1adedb785e2f4f40f4f7ff79f87e36bb55f2` |
| `khmerdict.u16le` | `7f56e4358372b19fe7b4aa15bee71e7f06e3d48f407c2c5973e5122043e3a214` |
| `laodict.u16le` | `4274ef25980a4d89dc6de38bade1ee3f67239e489178baf35d4bff971badc3a4` |
| `LICENSE-ICU` | `e55522d81edc687a341a4411e0776e54ca654e90147f354a90458aaced4116af` |

`python3 generate.py --gendict /path/to/gendict` verifies the source and the
checked-in arrays. `--write` regenerates them after checking the expected output
hash. The recorded generation used ICU4C 78.3 `gendict --uchars --toml` and
Python 3.12: the TOML `trie_data` integers are serialized as little-endian
16-bit code units. ICU4X's Char16Trie reads this ICU4C UCharsTrie encoding.
Regeneration is a development operation; no C or C++ library, generator, or
network access is needed at runtime.

The provider uses these two tries only for Khmer and Lao. Thai and Myanmar
remain the ICU4X 2.4.0 compiled data, as do all line and grapheme rules.
A standalone macOS ICU4X prototype reproduced the Chromium reference's Khmer
break opportunities for the recorded sample; the three other sampled scripts
were unchanged. Chromium's own ICU4C includes a Khmer engine patch and ICU4X
has a different matcher. Broader correctness, actual rendered rows, device and
platform qualification remain separate and cannot be inferred from that sample.

Sources: [Chromium dependency pin](https://raw.githubusercontent.com/chromium/chromium/156.0.8078.47/DEPS),
[Chromium ICU provenance](https://chromium.googlesource.com/chromium/deps/icu/+/19dfb44a62ae60ce6b0060de8c0cef5d5c527177/README.chromium),
[Khmer source](https://chromium.googlesource.com/chromium/deps/icu/+/19dfb44a62ae60ce6b0060de8c0cef5d5c527177/source/data/brkitr/dictionaries/khmerdict.txt),
[Lao source](https://chromium.googlesource.com/chromium/deps/icu/+/19dfb44a62ae60ce6b0060de8c0cef5d5c527177/source/data/brkitr/dictionaries/laodict.txt).
