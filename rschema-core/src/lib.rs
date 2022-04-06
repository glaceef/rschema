mod error;
mod prop_type;
mod schema;
mod schematic;

pub use error::Error;
pub use prop_type::{
    ArrayProp,
    EnumProp,
    Items,
    NumericProp,
    ObjectProp,
    Properties,
    Property,
    PropType,
    StringProp,
    TupleProp,
};
pub use schema::Schema;
pub use schematic::Schematic;

/// Alias for a Result with the error type `rschema::Error`.
/// 
pub type Result<T> = std::result::Result<T, Error>;
