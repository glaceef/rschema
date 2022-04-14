#![allow(dead_code)]

use rschema::{
    Schema,
    Schematic,
};

// A structure in an extern crate.
#[derive(Debug)]
struct ExternalStruct {
    data: i32,
}

// Provide schema information instead of ExternalStruct
#[derive(Debug, Schematic)]
struct AlternativeStruct {
    #[rschema(minimum = 10)]
    data: i32,
}

mod extern_crate {
    use super::Schematic;

    // Provide schema information instead of ExternalStruct
    #[derive(Debug, Schematic)]
    pub struct AlternativeStruct {
        #[rschema(minimum = 10)]
        data: i32,
    }
}

#[derive(Debug, Schematic)]
struct External {
    #[rschema(rename = "prop___i32")]
    prop_i32: i32,

    #[rschema(
        alt = "AlternativeStruct",
        title = "AlternativeStruct",
    )]
    prop_external: ExternalStruct,

    #[rschema(
        alt = "extern_crate::AlternativeStruct",
        title = "AlternativeStruct",
    )]
    prop_external2: ExternalStruct,
}

#[test]
fn it_tests_external() -> rschema::Result<()> {
    let schema_str = Schema::new::<External>("External")
        .to_string_pretty()?;
    let schema_str2 = r#"{
  "title": "External",
  "type": "object",
  "properties": {
    "prop___i32": {
      "type": "number"
    },
    "prop_external": {
      "title": "AlternativeStruct",
      "type": "object",
      "properties": {
        "data": {
          "type": "number",
          "minimum": 10
        }
      },
      "additionalProperties": false
    },
    "prop_external2": {
      "title": "AlternativeStruct",
      "type": "object",
      "properties": {
        "data": {
          "type": "number",
          "minimum": 10
        }
      },
      "additionalProperties": false
    }
  },
  "additionalProperties": false
}"#;

    assert_eq!(schema_str, schema_str2);

    Ok(())
}