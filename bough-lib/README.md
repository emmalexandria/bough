# `bough-lib`

`bough-lib` is the Rust crate which powers both `boughd` and Bough. It handles the output formats, 
icons, directory reading, theming, etc. 

Please note that most of the features present in the Bough CLI are feature gated in `bough-lib`. This is done to minimise dependencies, ensuring dependant applications only pay for what they use. With default features, `bough-lib` will only provide ASCII and Markdown output.

## Crate Features
|**Feature**|**Purpose**|**Additional dependencies**|
|-----------|-----------|---------------------------|
| `html` | Enables HTML output | `tl` |
| `ansi` | Enables output with ANSI codes | `owo-colors` |
| `theme` | Enables theme file support | `serde` | 




