mod definitions_map;
mod draft;
mod error;
mod schema;
mod schematic;

pub use definitions_map::DefinitionsMap;
pub use draft::Draft;
pub use error::Error;
pub use schema::{
    r#type::*,
    Definitions,
    Schema,
    Type,
};
pub use schematic::Schematic;

/// Alias for a `Result` with the error type `rschema::Error`.
/// 
pub type Result<T> = std::result::Result<T, Error>;

pub(crate) fn is_falsy(b: &Option<bool>) -> bool {
    *b != Some(true)
}
