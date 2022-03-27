use darling::FromField;

mod field;
use field::Field;

#[derive(Debug, FromField)]
#[darling(attributes(rschema))]
// #[darling(attributes(rschema), forward_attrs(cfg))]
pub struct FieldAttr {
    pub field: Field,

    #[darling(default)]
    pub required: bool,
}