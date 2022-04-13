#![allow(dead_code)]

use rschema::{
    Schema,
    Schematic,
};

mod external_crate {
    use super::*;

    #[derive(Debug, Schematic)]
    #[rschema(additional_properties)]
    #[rschema(definition)]
    pub struct Sample {
        prop_string: String,
    }
}

#[derive(Debug, Schematic)]
struct NoDefSample {
    prop_string: String,

    prop_sample: Sample,
}

#[derive(Debug, Schematic)]
#[rschema(definition)]
struct Sample {
    prop_string: String,
}

#[derive(Debug, Schematic)]
#[rschema(definition)]
struct NestedSample {
    prop_sample: Sample,
}

#[derive(Debug, Schematic)]
struct Definitions {
    prop_no_def_sample: NoDefSample,

    prop_sample: Sample,

    prop_sample_external: external_crate::Sample,

    prop_nested_sample: NestedSample,
}

#[test]
fn it_tests_definitions() -> rschema::Result<()> {
    let schema_str = Schema::new::<Definitions>("Definitions")
        .to_string_pretty()?;
    let schema_str2 = r##"{
  "title": "Definitions",
  "type": "object",
  "properties": {
    "prop_no_def_sample": {
      "type": "object",
      "properties": {
        "prop_string": {
          "type": "string"
        },
        "prop_sample": {
          "$ref": "#/$defs/definitions::Sample"
        }
      },
      "additionalProperties": false
    },
    "prop_sample": {
      "$ref": "#/$defs/definitions::Sample"
    },
    "prop_sample_external": {
      "$ref": "#/$defs/definitions::external_crate::Sample"
    },
    "prop_nested_sample": {
      "$ref": "#/$defs/definitions::NestedSample"
    }
  },
  "additionalProperties": false,
  "$defs": {
    "definitions::Sample": {
      "type": "object",
      "properties": {
        "prop_string": {
          "type": "string"
        }
      },
      "additionalProperties": false
    },
    "definitions::external_crate::Sample": {
      "type": "object",
      "properties": {
        "prop_string": {
          "type": "string"
        }
      },
      "additionalProperties": true
    },
    "definitions::NestedSample": {
      "type": "object",
      "properties": {
        "prop_sample": {
          "$ref": "#/$defs/definitions::Sample"
        }
      },
      "additionalProperties": false
    }
  }
}"##;

    assert_eq!(schema_str, schema_str2);

    Ok(())
}