#![allow(dead_code)]

use rschema::{
    Schema,
    Schematic,
};

#[derive(Debug, Schematic)]
struct NumericProperties {
    #[rschema(title = "i8")]
    prop_i8: i8,

    #[rschema(title = "i16")]
    prop_i16: i16,

    #[rschema(title = "i32")]
    prop_i32: i32,

    #[rschema(title = "i64")]
    prop_i64: i64,

    #[rschema(title = "isize")]
    prop_isize: isize,

    #[rschema(title = "u8")]
    prop_u8: u8,

    #[rschema(title = "u16")]
    prop_u16: u16,

    #[rschema(title = "u32")]
    prop_u32: u32,

    #[rschema(title = "u64")]
    prop_u64: u64,

    #[rschema(title = "usize")]
    prop_usize: usize,

    #[rschema(title = "f32")]
    prop_f32: f32,

    #[rschema(title = "f64")]
    prop_f64: f64,

    #[rschema(
        title = "All keywords",
        minimum = 0,
        maximum = 100,
        multiple_of = 10,
        exclusive_minimum = 0,
        exclusive_maximum = 100,
    )]
    prop_all_keywords: i32,
}

#[test]
fn it_generates_numeric_schema() -> rschema::Result<()> {
    let schema_str = Schema::new::<NumericProperties>("Numeric Properties")
        .to_string_pretty()?;
    let schema_str2 = r#"{
  "title": "Numeric Properties",
  "type": "object",
  "properties": {
    "prop_i8": {
      "title": "i8",
      "type": "number"
    },
    "prop_i16": {
      "title": "i16",
      "type": "number"
    },
    "prop_i32": {
      "title": "i32",
      "type": "number"
    },
    "prop_i64": {
      "title": "i64",
      "type": "number"
    },
    "prop_isize": {
      "title": "isize",
      "type": "number"
    },
    "prop_u8": {
      "title": "u8",
      "type": "number"
    },
    "prop_u16": {
      "title": "u16",
      "type": "number"
    },
    "prop_u32": {
      "title": "u32",
      "type": "number"
    },
    "prop_u64": {
      "title": "u64",
      "type": "number"
    },
    "prop_usize": {
      "title": "usize",
      "type": "number"
    },
    "prop_f32": {
      "title": "f32",
      "type": "number"
    },
    "prop_f64": {
      "title": "f64",
      "type": "number"
    },
    "prop_all_keywords": {
      "title": "All keywords",
      "type": "number",
      "minimum": 0,
      "maximum": 100,
      "multipleOf": 10,
      "exclusiveMinimum": 0,
      "exclusiveMaximum": 100
    }
  },
  "additionalProperties": false
}"#;

    assert_eq!(schema_str, schema_str2);

    Ok(())
}