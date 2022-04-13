#![allow(dead_code)]

use rschema::{
    Schema,
    Schematic,
};

#[derive(Debug, Schematic)]
#[rschema(additional_properties)]
struct RequiredProperties {
    #[rschema(
        title = "i32",
        required,
    )]
    prop_required1: i32,

    #[rschema(
        title = "String",
        required,
    )]
    prop_required2: String,

    #[rschema(title = "bool")]
    prop_not_required: bool,
}

#[test]
fn it_generates_required_properties() -> rschema::Result<()> {
    let schema_str = Schema::new::<RequiredProperties>("RequiredProperties")
        .to_string_pretty()?;
    let schema_str2 = r#"{
  "title": "RequiredProperties",
  "type": "object",
  "properties": {
    "prop_required1": {
      "title": "i32",
      "type": "number"
    },
    "prop_required2": {
      "title": "String",
      "type": "string"
    },
    "prop_not_required": {
      "title": "bool",
      "type": "boolean"
    }
  },
  "required": [
    "prop_required1",
    "prop_required2"
  ],
  "additionalProperties": true
}"#;

    assert_eq!(schema_str, schema_str2);

    Ok(())
}