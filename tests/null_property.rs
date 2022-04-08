#![allow(dead_code)]

use rschema::{
    Schema,
    Schematic,
};

#[derive(Debug, Schematic)]
struct NullProperty {
    #[rschema(title = "null")]
    prop_null: (),
}

#[test]
fn it_generates_null_schema() -> rschema::Result<()> {
    let schema_str = Schema::new::<NullProperty>("Null Property")
        .to_string_pretty()?;
    let schema_str2 = r#"{
  "title": "Null Property",
  "type": "object",
  "properties": {
    "prop_null": {
      "title": "null",
      "type": "null"
    }
  },
  "additionalProperties": false
}"#;

    assert_eq!(schema_str, schema_str2);

    Ok(())
}