#![allow(dead_code)]

use rschema::{
    Schema,
    Schematic,
};

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
struct ArrayProperties {
    #[rschema(title = "[i32; 3]")]
    prop_array3: [i32; 3],

    // #[rschema(title = "&[i32]")]
    // prop_ref_array: &'a [i32],

    #[rschema(title = "Vec<i32>")]
    prop_vec: Vec<i32>,

    #[rschema(title = "(i32, String, bool)")]
    prop_tuple3: (i32, String, bool),

    #[rschema(title = "Vec<Enum>")]
    prop_vec_enum: Vec<Enum>,

    #[rschema(
        title = "All keywords",
        min_items = 1,
        max_items = 5,
        unique_items,
    )]
    prop_all_keywords: Vec<i32>,
}

#[test]
fn it_generates_array_schema() -> rschema::Result<()> {
    let schema_str = Schema::new::<ArrayProperties>("Array Properties")
        .to_string_pretty()?;
    let schema_str2 = r#"{
  "title": "Array Properties",
  "type": "object",
  "properties": {
    "prop_array3": {
      "title": "[i32; 3]",
      "type": "array",
      "items": {
        "type": "number"
      },
      "minItems": 3,
      "maxItems": 3
    },
    "prop_vec": {
      "title": "Vec<i32>",
      "type": "array",
      "items": {
        "type": "number"
      }
    },
    "prop_tuple3": {
      "title": "(i32, String, bool)",
      "type": "array",
      "items": [
        {
          "type": "number"
        },
        {
          "type": "string"
        },
        {
          "type": "boolean"
        }
      ]
    },
    "prop_vec_enum": {
      "title": "Vec<Enum>",
      "type": "array",
      "items": {
        "anyOf": [
          {
            "type": "array",
            "items": []
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
            ]
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
      }
    },
    "prop_all_keywords": {
      "title": "All keywords",
      "type": "array",
      "items": {
        "type": "number"
      },
      "minItems": 1,
      "maxItems": 5,
      "uniqueItems": true
    }
  },
  "additionalProperties": false
}"#;

    assert_eq!(schema_str, schema_str2);

    Ok(())
}