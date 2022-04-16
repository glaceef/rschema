use darling::FromDeriveInput;

use super::{
    Case,
    definitions::{
        Definitions,
        and_then,
    },
};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(rschema))]
pub struct EnumAttr {
    #[darling(default)]
    pub rename_all: Option<Case>,

    #[darling(default)]
    #[darling(and_then = "and_then")]
    pub defs: Definitions,
}
