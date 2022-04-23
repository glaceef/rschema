#![allow(dead_code)]

use rschema::{
    Schema,
    Schematic,
};

#[derive(Debug, Schematic)]
struct StringProperties<'a> {
    #[rschema(title = "&str")]
    prop_ref_str: &'a str,

    #[rschema(title = "&'static str")]
    prop_static_ref_str: &'static str,

    #[rschema(title = "String")]
    prop_string: String,

    #[rschema(title = "char")]
    prop_char: char,

    #[rschema(
        title = "All keywords",
        min_length = 0,
        max_length = 20,
        pattern = r"^\w+$",
        format = "format",
    )]
    prop_all_keywords: String,
}

#[test]
fn it_tests_string_properties() -> rschema::Result<()> {
    let schema_str = Schema::new::<StringProperties>("String Properties")
        .to_string_pretty()?;
    let schema_str2 = r#"{
  "title": "String Properties",
  "type": "object",
  "properties": {
    "prop_ref_str": {
      "title": "&str",
      "type": "string"
    },
    "prop_static_ref_str": {
      "title": "&'static str",
      "type": "string"
    },
    "prop_string": {
      "title": "String",
      "type": "string"
    },
    "prop_char": {
      "title": "char",
      "type": "string",
      "minLength": 1,
      "maxLength": 1
    },
    "prop_all_keywords": {
      "title": "All keywords",
      "type": "string",
      "minLength": 0,
      "maxLength": 20,
      "pattern": "^\\w+$",
      "format": "format"
    }
  },
  "additionalProperties": false
}"#;

    assert_eq!(schema_str, schema_str2);

    Ok(())
}