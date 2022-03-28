# Rschema &emsp; [![Crates Badge]][crates.io] [![Docs Badge]][docs.rs] [![License: Apache]][Apache 2.0] [![License: MIT]][MIT]

[Crates Badge]: https://img.shields.io/crates/v/rschema.svg
[crates.io]: https://crates.io/crates/rschema
[Docs Badge]: https://docs.rs/rschema/badge.svg
[docs.rs]: https://docs.rs/rschema
[License: Apache]: https://img.shields.io/badge/License-Apache_2.0-blue.svg
[Apache 2.0]: https://opensource.org/licenses/Apache-2.0
[License: MIT]: https://img.shields.io/badge/License-MIT-yellow.svg
[MIT]: https://opensource.org/licenses/MIT

***Rschema* provides a macro for generating JSON schemas from Rust structures.**

---

## Description

Often, you will use a framework such as **serde** to deserialize the contents of a configuration file into a Rust structure.

This tool generates schema information for the target file from the structure to be deserialized.

This supports pre-validation of the configuration file.

## Example code

```rust
use rschema::{
    Schema,
    Schematic,
};

#[derive(Debug, Schematic)]
#[rschema(additional_properties)]
struct Custom {
    #[rschema(field(
        title = "A flag for debugging",
        description = "Set `true` to display debug logs.",
    ))]
    debug_flag: bool,
}

#[derive(Debug, Schematic)]
struct AppConfig {
    #[rschema(
        field(
            title = "Application Config",
            description = "Application configuration file.",
        ),
        required,
    )]
    app_name: String,

    #[rschema(
        field(
            title = "Version",
            description = "Application version.",
            pattern = r"^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$",
        ),
        required,
    )]
    version: String,

    #[rschema(field(
        title = "Custom Configuration",
        description = "Additional custom configuration.",
    ))]
    custom: Custom,
}

fn main() -> rschema::Result<()> {
    Schema::new::<Config>("Application Config")
        // Add description if needed.
        .description("Application configuration file.")
        // Save as file.
        .write_pretty("config.schema.json")?;
    
    Ok(())
}
```

The above code generates the following file:

```json
{
  "title": "Application Config",
  "description": "Application configuration file.",
  "type": "object",
  "properties": {
    "app_name": {
      "title": "Application Config",
      "description": "Application configuration file.",
      "type": "string"
    },
    "version": {
      "title": "Version",
      "description": "Application version.",
      "type": "string",
      "pattern": "^(0|[1-9][0-9]*)\\.(0|[1-9][0-9]*)\\.(0|[1-9][0-9]*)$"
    },
    "custom": {
      "title": "Custom Configuration",
      "description": "Additional custom configuration.",
      "type": "object",
      "properties": {
        "debug_flag": {
          "title": "A flag for debugging",
          "description": "Set `true` to display debug logs.",
          "type": "boolean"
        }
      },
      "additionalProperties": true
    }
  },
  "required": [
    "app_name",
    "version"
  ],
  "additionalProperties": false
}
```

It can be used with support tools such as VSCode.
