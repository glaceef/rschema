#![allow(dead_code)]

use rschema::{
    Schema,
    Schematic,
};

#[derive(Debug, Schematic)]
#[rschema(additional_properties)]
struct StructAttributes {
    #[rschema(required)]
    prop_required1: i32,

    #[rschema(required)]
    prop_required2: String,

    prop_not_required: bool,
}

#[test]
fn it_tests_struct_attributes() -> rschema::Result<()> {
    let schema_str = Schema::new::<StructAttributes>("StructAttributes")
        .to_string_pretty()?;
    let schema_str2 = r#"{
  "title": "StructAttributes",
  "type": "object",
  "properties": {
    "prop_required1": {
      "type": "number"
    },
    "prop_required2": {
      "type": "string"
    },
    "prop_not_required": {
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