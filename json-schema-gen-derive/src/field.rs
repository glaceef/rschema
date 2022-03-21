use darling::FromMeta;
use serde::Serialize;

mod type_regex;
use type_regex::{
    ARRAY_TYPE_REGEX,
    NUMBER_TYPE_REGEX,
    STRING_TYPE_REGEX,
    BOOLEAN_TYPE_REGEX,
};

/* TODO: 削除
fieldアトリビュートをまずこの構造体にパースする。
JSON文字列にシリアライズして、Field構造体にデシリアライズする。
これはdarlingがserdeほど柔軟にデシリアライズできないため。
*/

#[derive(Debug, FromMeta, Serialize)]
pub struct Field {
    /* common */
    title: String,
    description: String,

    #[darling(default)]
    #[darling(rename = "type")]
    #[serde(rename = "type")]
    ty: Option<String>,

    /* type: string */
    #[darling(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    min_length: Option<u64>,
    #[darling(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    max_length: Option<u64>,
    #[darling(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pattern: Option<String>,

    /* type: number */
    #[darling(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum: Option<i64>,
    #[darling(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum: Option<i64>,

    /* type: array */
    #[darling(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    min_items: Option<u64>,
    #[darling(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    max_items: Option<u64>,
    // #[darling(default)]
    // items: Property,

    /* type: object */
    // #[darling(default)]
    // properties: Properties,
}

impl Field {
    // ex) string, number, array, ...(lowercase)
    // OtherPropsのヴァリアントに対応している。
    pub fn set_type(&mut self, type_str: &str) {
        // アトリビュートで指定されていたらそれを優先する。
        if self.ty.is_some() {
            return;
        }

        let type_str = match type_str {
            t if  STRING_TYPE_REGEX.is_match(t) => "string",
            t if  NUMBER_TYPE_REGEX.is_match(t) => "number", // 明示的に integer を指定しない場合、数値型はすべて number となる。
            t if   ARRAY_TYPE_REGEX.is_match(t) => "array",
            t if BOOLEAN_TYPE_REGEX.is_match(t) => "boolean",
            _ => "object",
        };
        self.ty = Some(type_str.into());
    }

    pub fn is_object(&self) -> bool {
        match self.ty {
            Some(ref t) if t == "object" => true,
            _ => false,
        }
    }
}