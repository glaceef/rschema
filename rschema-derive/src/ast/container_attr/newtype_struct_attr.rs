use darling::FromDeriveInput;

use super::definitions::{
    Definitions,
    and_then,
};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(rschema))]
pub struct NewTypeStructAttr {
    #[darling(default)]
    #[darling(and_then = "and_then")]
    pub defs: Definitions,
}
