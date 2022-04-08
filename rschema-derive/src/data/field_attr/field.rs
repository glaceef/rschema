use darling::FromMeta;

#[derive(Debug, FromMeta)]
pub struct Field {
    /* common */
    pub title: String,
    #[darling(default)]
    pub description: Option<String>,
    #[darling(default)]
    pub comment: Option<String>,
    #[darling(default)]
    pub deprecated: Option<bool>,

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
}
