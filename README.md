# Rschema &emsp; [![Crates Badge]][crates.io] [![Docs Badge]][docs.rs] [![License: Apache]][Apache 2.0] [![License: MIT]][MIT]

[Crates Badge]: https://img.shields.io/crates/v/rschema.svg
[crates.io]: https://crates.io/crates/rschema
[Docs Badge]: https://docs.rs/rschema/badge.svg
[docs.rs]: https://docs.rs/rschema
[License: Apache]: https://img.shields.io/badge/License-Apache_2.0-blue.svg
[Apache 2.0]: https://opensource.org/licenses/Apache-2.0
[License: MIT]: https://img.shields.io/badge/License-MIT-yellow.svg
[MIT]: https://opensource.org/licenses/MIT

*Rschema* provides a macro for generating JSON schemas from Rust structures.

# Example

```rust
use rschema::{
    Schema,
    Schematic,
};

#[derive(Debug, Schematic)]
#[rschema(additional_properties)]
struct Data {
    #[rschema(
        title = "Test flag",
        description = "The flag whether for test.",
    )]
    test_flag: bool,
}

#[derive(Debug, Schematic)]
struct AppConfig {
    #[rschema(
        title = "Application name",
        required,
    )]
    name: String,

    #[rschema(
        title = "Application version",
        pattern = r"^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$",
        required,
    )]
    version: String,

    #[rschema(
        title = "Application data",
        description = "This property is optional.",
    )]
    other_data: Data,
}

fn main() -> rschema::Result<()> {
    Schema::new::<AppConfig>("Application Config")
        .write_pretty("../schemas/config.schema.json")?;

    Ok(())
}
```

This code generates the following JSON schema file.

```json
{
  "title": "Application Config",
  "type": "object",
  "properties": {
    "name": {
      "title": "Application name",
      "type": "string"
    },
    "version": {
      "title": "Application version",
      "type": "string",
      "pattern": "^(0|[1-9][0-9]*)\\.(0|[1-9][0-9]*)\\.(0|[1-9][0-9]*)$"
    },
    "other_data": {
      "title": "Application data",
      "type": "object",
      "properties": {
        "test_flag": {
          "title": "Test flag",
          "description": "The flag whether for test.",
          "type": "boolean"
        }
      },
      "additionalProperties": true
    }
  },
  "required": [
    "name",
    "version"
  ],
  "additionalProperties": false
}
```


# Combination with Serde

*Rschema* is strongly intended to be used in combination with [*Serde*](https://serde.rs/).

For example, generate a JSON schema from structs and enums you define.
Data files validated by the JSON schema are always deserializable to the original structures!

