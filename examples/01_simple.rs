#![allow(dead_code)]

use rschema::{
    Schema,
    Schematic,
};

#[derive(Debug, Schematic)]
struct SimpleData {
    #[rschema(title = "Data Name")]
    name: String,

    #[rschema(
        minimum = 0,
        exclusive_maximum = 256,
    )]
    value: i32,

    #[rschema(skip)]
    private_value: i32,
}

fn main() -> rschema::Result<()> {
    let schema_str = Schema::new::<SimpleData>("01_Simple")
        .to_string_pretty()?;
    let expected = r#"{
  "title": "01_Simple",
  "type": "object",
  "properties": {
    "name": {
      "title": "Data Name",
      "type": "string"
    },
    "value": {
      "type": "number",
      "minimum": 0,
      "exclusiveMaximum": 256
    }
  },
  "additionalProperties": false
}"#;

    assert_eq!(schema_str, expected);

    Ok(())
}
