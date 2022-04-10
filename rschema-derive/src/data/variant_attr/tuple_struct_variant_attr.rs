use darling::FromAttributes;

#[derive(Debug, FromAttributes)]
#[darling(attributes(rschema))]
pub struct TupleStructVariantAttr {
    #[darling(default)]
    pub skip: Option<bool>,

    #[darling(default)]
    pub unique_items: Option<bool>,
}
