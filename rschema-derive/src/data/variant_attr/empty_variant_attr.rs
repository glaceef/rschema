use darling::FromAttributes;

#[derive(Debug, FromAttributes)]
#[darling(attributes(rschema))]
pub struct EmptyVariantAttr {
}
