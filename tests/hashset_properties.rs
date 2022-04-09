#![allow(dead_code)]

use rschema::{
    Schema,
    Schematic,
};

use std::collections::HashSet;

#[derive(Debug, Schematic)]
enum Enum {
    UnitVariant,

    EmptyTupleVariant(),

    TupleVariant(i32, String),

    StructVariant {
        #[rschema(title = "i32")]
        value: i32,
    }
}

#[derive(Debug, Schematic)]
struct HashSetProperties {
    #[rschema(title = "HashSet<i32>")]
    prop_hashset_simgle: HashSet<i32>,

    #[rschema(title = "HashSet<Enum>")]
    prop_hashset_complex: HashSet<Enum>,
}

#[test]
fn it_generates_hashset_schema() -> rschema::Result<()> {
    let schema_str = Schema::new::<HashSetProperties>("HashSet Properties")
        .to_string_pretty()?;
    let schema_str2 = r#"{
  "title": "HashSet Properties",
  "type": "object",
  "properties": {
    "prop_hashset_simgle": {
      "title": "HashSet<i32>",
      "type": "array",
      "items": {
        "type": "number"
      },
      "uniqueItems": true
    },
    "prop_hashset_complex": {
      "title": "HashSet<Enum>",
      "type": "array",
      "items": {
        "anyOf": [
          {
            "type": "array",
            "items": [],
            "minItems": 0,
            "maxItems": 0
          },
          {
            "type": "array",
            "items": [
              {
                "type": "number"
              },
              {
                "type": "string"
              }
            ],
            "minItems": 2,
            "maxItems": 2
          },
          {
            "type": "object",
            "properties": {
              "value": {
                "title": "i32",
                "type": "number"
              }
            },
            "additionalProperties": false
          },
          {
            "type": "string",
            "enum": [
              "UnitVariant"
            ]
          }
        ]
      },
      "uniqueItems": true
    }
  },
  "additionalProperties": false
}"#;

    assert_eq!(schema_str, schema_str2);

    Ok(())
}