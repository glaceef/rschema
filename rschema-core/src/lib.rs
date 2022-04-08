mod draft;
mod error;
mod r#type;
mod schema;
mod schematic;

pub use draft::Draft;
pub use error::Error;
pub use r#type::{
    AdditionalProperties,
    ArrayKeys,
    EnumKeys,
    Items,
    NumericKeys,
    ObjectKeys,
    Properties,
    Property,
    Type,
    StringKeys,
    TupleKeys,
};
pub use schema::Schema;
pub use schematic::Schematic;

/// Alias for a `Result` with the error type `rschema::Error`.
/// 
pub type Result<T> = std::result::Result<T, Error>;
