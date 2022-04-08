use serde::Serialize;

/// Meta-schema versions. [Read more](https://json-schema.org/understanding-json-schema/reference/schema.html#schema)
/// 
#[derive(Debug, Serialize)]
pub enum Draft {
    /// Add `"$schema": "http://json-schema.org/draft-04/schema#"`.
    /// 
    #[serde(rename = "http://json-schema.org/draft-04/schema#")]
    Draft4,

    /// Add `"$schema": "http://json-schema.org/draft-06/schema#"`.
    /// 
    #[serde(rename = "http://json-schema.org/draft-06/schema#")]
    Draft6,

    /// Add `"$schema": "http://json-schema.org/draft-07/schema#"`.
    /// 
    #[serde(rename = "http://json-schema.org/draft-07/schema#")]
    Draft7,

    /// Add `"$schema": "https://json-schema.org/draft/2019-09/schema"`.
    /// 
    #[serde(rename = "https://json-schema.org/draft/2019-09/schema")]
    Draft201909,

    /// Add `"$schema": "https://json-schema.org/draft/2020-12/schema"`.
    /// 
    #[serde(rename = "https://json-schema.org/draft/2020-12/schema")]
    Draft202012,
}
