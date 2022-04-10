use darling::FromDeriveInput;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(rschema))]
pub struct EmptyStructAttr {
}
