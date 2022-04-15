use darling::FromDeriveInput;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(rschema))]
pub struct NewTypeStructAttr {
    #[darling(default)]
    pub definitions: Option<bool>,
}
