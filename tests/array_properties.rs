#![allow(dead_code)]

use rschema::{
    Schema,
    Schematic,
};

#[derive(Debug, Schematic)]
#[rschema(unique_items)]
struct TupleStruct(u32, u32);

#[derive(Debug, Schematic)]
enum Enum {
    UnitVariant,

    EmptyTupleVariant(),

    NewTypeVariant(i32),

    TupleVariant(String, bool),

    StructVariant {
        #[rschema(title = "i32")]
        value: i32,
    }
}

#[derive(Debug, Schematic)]
struct ArrayProperties<'a> {
    #[rschema(title = "[i32; 3]")]
    prop_sized_array: [i32; 3],

    #[rschema(title = "&[i32]")]
    prop_array: &'a [i32],

    #[rschema(title = "Vec<i32>")]
    prop_vec: Vec<i32>,

    #[rschema(title = "(i32, String, bool)")]
    prop_tuple: (i32, String, bool),

    #[rschema(
        title = "TupleStruct",
        unique_items,
    )]
    prop_unique_tuple_struct: TupleStruct,

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
fn it_tests_array_properties() -> rschema::Result<()> {
    let schema_str = Schema::new::<ArrayProperties>("Array Properties")
        .to_string_pretty()?;
    let schema_str2 = r#"{
  "title": "Array Properties",
  "type": "object",
  "properties": {
    "prop_sized_array": {
      "title": "[i32; 3]",
      "type": "array",
      "items": {
        "type": "number"
      },
      "minItems": 3,
      "maxItems": 3
    },
    "prop_array": {
      "title": "&[i32]",
      "type": "array",
      "items": {
        "type": "number"
      }
    },
    "prop_vec": {
      "title": "Vec<i32>",
      "type": "array",
      "items": {
        "type": "number"
      }
    },
    "prop_tuple": {
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
      ],
      "minItems": 3,
      "maxItems": 3
    },
    "prop_unique_tuple_struct": {
      "title": "TupleStruct",
      "type": "array",
      "items": [
        {
          "type": "number"
        },
        {
          "type": "number"
        }
      ],
      "minItems": 2,
      "maxItems": 2,
      "uniqueItems": true
    },
    "prop_vec_enum": {
      "title": "Vec<Enum>",
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
            "type": "number"
          },
          {
            "type": "array",
            "items": [
              {
                "type": "string"
              },
              {
                "type": "boolean"
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