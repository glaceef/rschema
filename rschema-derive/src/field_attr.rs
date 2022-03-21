use darling::FromAttributes;

mod field;
use field::Field;

#[derive(Debug, FromAttributes)]
#[darling(attributes(rschema))]
// #[darling(attributes(rschema), forward_attrs(cfg))]
pub struct FieldAttr {
    pub field: Field,

    #[darling(default)]
    pub required: bool,
}