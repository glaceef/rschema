#![allow(dead_code)]

use rschema::{
    Schema,
    Schematic,
};

use std::collections::{
    HashMap,
    HashSet,
};

#[derive(Debug, Schematic)]
struct OtherProperties {
    #[rschema(title = "Option<i32>")]
    prop_option: Option<i32>,

    #[rschema(title = "Box<i32>")]
    prop_box: Box<i32>,

    #[rschema(title = "HashMap<String, i32>")]
    prop_hashmap: HashMap<String, i32>,

    #[rschema(title = "HashSet<i32>")]
    prop_hashset: HashSet<i32>,
}

#[test]
fn it_tests_other_properties() -> rschema::Result<()> {
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
    },
    "prop_hashmap": {
      "title": "HashMap<String, i32>",
      "type": "object",
      "properties": {},
      "additionalProperties": {
        "type": "number"
      }
    },
    "prop_hashset": {
      "title": "HashSet<i32>",
      "type": "array",
      "items": {
        "type": "number"
      },
      "uniqueItems": true
    }
  },
  "additionalProperties": false
}"#;

    assert_eq!(schema_str, schema_str2);

    Ok(())
}