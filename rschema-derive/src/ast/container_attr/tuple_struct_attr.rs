use darling::FromDeriveInput;

use super::definitions::{
    Definitions,
    and_then,
};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(rschema))]
pub struct TupleStructAttr {
    #[darling(default)]
    pub unique_items: Option<bool>,

    #[darling(default)]
    #[darling(and_then = "and_then")]
    pub defs: Definitions,
}
