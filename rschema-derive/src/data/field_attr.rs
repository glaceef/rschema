use darling::FromField;

#[derive(Debug, FromField, PartialEq)]
#[darling(attributes(rschema))]
pub struct FieldAttr {
    /* common */
    #[darling(default)]
    pub title: Option<String>,
    #[darling(default)]
    pub description: Option<String>,
    #[darling(default)]
    pub comment: Option<String>,
    #[darling(default)]
    pub deprecated: Option<bool>,
    #[darling(default)]
    pub required: Option<bool>,

    /* type: string */
    #[darling(default)]
    pub min_length: Option<u64>,
    #[darling(default)]
    pub max_length: Option<u64>,
    #[darling(default)]
    pub pattern: Option<String>,
    #[darling(default)]
    pub format: Option<String>,

    /* type: number */
    #[darling(default)]
    pub minimum: Option<i64>,
    #[darling(default)]
    pub maximum: Option<i64>,
    #[darling(default)]
    pub multiple_of: Option<i64>,
    #[darling(default)]
    pub exclusive_minimum: Option<i64>,
    #[darling(default)]
    pub exclusive_maximum: Option<i64>,

    /* type: array */
    #[darling(default)]
    pub min_items: Option<usize>,
    #[darling(default)]
    pub max_items: Option<usize>,
    #[darling(default)]
    pub unique_items: Option<bool>,

    /* control */
    #[darling(default)]
    pub rename: Option<String>,
    #[darling(default)]
    pub alt: Option<syn::TypePath>,
    #[darling(default)]
    pub skip: Option<bool>,
}