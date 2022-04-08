#![allow(dead_code)]

use rschema::{
    Schema,
    Schematic,
};

#[derive(Debug, Schematic)]
struct OtherProperties {
    #[rschema(title = "Option<i32>")]
    prop_option: Option<i32>,

    #[rschema(title = "Box<i32>")]
    prop_box: Box<i32>,
}

#[test]
fn it_generates_other_schema() -> rschema::Result<()> {
    let schema_str = Schema::new::<OtherProperties>("Other Properties")
        .to_string_pretty()?;
    let schema_str2 = r#"{
  "title": "Other Properties",
  "type": "object",
  "properties": {
    "prop_option": {
      "title": "Option<i32>",
      "anyOf": [
        {
          "type": "number"
        },
        {
          "type": "null"
        }
      ]
    },
    "prop_box": {
      "title": "Box<i32>",
      "type": "number"
    }
  },
  "additionalProperties": false
}"#;

    assert_eq!(schema_str, schema_str2);

    Ok(())
}