#![allow(dead_code)]

use rschema::{
    Schema,
    Schematic,
};

#[derive(Debug, Schematic)]
struct UnitStruct;

#[derive(Debug, Schematic)]
struct NullProperties {
    #[rschema(title = "null")]
    prop_null: (),

    #[rschema(title = "null2")]
    prop_null2: UnitStruct,
}

#[test]
fn it_tests_null_properties() -> rschema::Result<()> {
    let schema_str = Schema::new::<NullProperties>("Null Properties")
        .to_string_pretty()?;
    let schema_str2 = r#"{
  "title": "Null Properties",
  "type": "object",
  "properties": {
    "prop_null": {
      "title": "null",
      "type": "null"
    },
    "prop_null2": {
      "title": "null2",
      "type": "null"
    }
  },
  "additionalProperties": false
}"#;

    assert_eq!(schema_str, schema_str2);

    Ok(())
}