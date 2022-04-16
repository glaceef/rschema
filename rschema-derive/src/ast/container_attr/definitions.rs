use darling::FromMeta;

// Define in `$defs`. If not given this, always written inline.
// 
// Allowed format are either
//  - `#[rschema(defs)]`
//  - `#[rschema(defs = "name")]`
#[derive(Debug, FromMeta)]
pub enum Definitions {
    Named(String),
    Auto,
    Skip,
}

impl Default for Definitions {
    fn default() -> Self {
        Definitions::Skip
    }
}

// True if the attribute is like `#[rschema(defs)]`.
pub fn is_word_format_error(err: &darling::Error) -> bool {
    // This is not good, but this is the only way I know of to check.
    err.to_string() == "Unexpected meta-item format `word`"
}

pub fn and_then(result: darling::Result<String>) -> darling::Result<Definitions> {
    match result {
        Ok(name) => Ok(Definitions::Named(name)),
        Err(ref e) if is_word_format_error(e) => {
            // The format `#[rschema(defs)]` is treated as `Define::Auto`,
            // not as a parse error.
            Ok(Definitions::Auto)
        },
        Err(e) => Err(e),
    }
}
