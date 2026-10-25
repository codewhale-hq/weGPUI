# Article list constructor contracts

Based on main `e685e07`, in the existing Linux cloud verification container.
`ArticleList::new(ListStyle::Inside)` previously retained `Inside` as its marker
kind, but rendering only produces numeric prefixes for `Decimal`. The shorthand
`inside()` manually corrected that field. Both entry points now select decimal
markers through the shared constructor.

`tests.txt`: all nine library tests pass, including explicit/shorthand agreement
for all four styles and nested marker inheritance. `checks.txt`: all-target build,
Clippy with warnings denied, strict rustdoc and formatting pass.

This is source-contract and build evidence. No new rendering, real input,
accessibility-tree or screen-reader result is claimed for this slice.
