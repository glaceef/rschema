mod error;
mod prop_type;
mod properties;
mod property;
mod schema;
mod schematic;

pub use error::RschemaError;
pub use prop_type::{
    ArrayProp,
    EnumProp,
    Items,
    NumericProp,
    ObjectProp,
    PropType,
    StringProp,
    TupleProp,
};
pub use properties::Properties;
pub use property::Property;
pub use schema::Schema;
pub use schematic::Schematic;

pub type Result<T> = std::result::Result<T, RschemaError>;
