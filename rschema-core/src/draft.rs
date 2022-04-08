use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Draft {
    #[serde(rename = "http://json-schema.org/draft-04/schema#")]
    Draft4,

    #[serde(rename = "http://json-schema.org/draft-06/schema#")]
    Draft6,

    #[serde(rename = "http://json-schema.org/draft-07/schema#")]
    Draft7,

    #[serde(rename = "https://json-schema.org/draft/2019-09/schema")]
    Draft201909,

    #[serde(rename = "https://json-schema.org/draft/2020-12/schema")]
    Draft202012,
}
