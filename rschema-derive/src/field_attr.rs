use darling::FromAttributes;

mod field;
use field::Field;

#[derive(Debug, FromAttributes)]
#[darling(attributes(schema))]
// #[darling(attributes(schema), forward_attrs(cfg))]
pub struct FieldAttr {
    pub field: Field,

    #[darling(default)]
    pub required: bool,
}