use darling::FromDeriveInput;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(rschema))]
pub struct TupleStructAttr {
    #[darling(default)]
    pub unique_items: Option<bool>,

    #[darling(default)]
    pub definitions: Option<bool>,
}
