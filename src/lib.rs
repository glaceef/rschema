//! # Rschema
//! 
//! *Rschema* provides a macro for generating JSON schemas from Rust structures.
//! 
//! # Example
//! 
//! ```no_run
//! use rschema::{
//!     Schema,
//!     Schematic,
//! };
//! 
//! #[derive(Debug, Schematic)]
//! #[rschema(additional_properties)]
//! struct Data {
//!     #[rschema(
//!         title = "Test flag",
//!         description = "The flag whether for test.",
//!     )]
//!     test_flag: bool,
//! }
//! 
//! #[derive(Debug, Schematic)]
//! struct AppConfig {
//!     #[rschema(
//!         title = "Application name",
//!         required,
//!     )]
//!     name: String,
//! 
//!     #[rschema(
//!         title = "Application version",
//!         pattern = r"^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$",
//!         required,
//!     )]
//!     version: String,
//! 
//!     #[rschema(
//!         title = "Application data",
//!         description = "This property is optional.",
//!     )]
//!     other_data: Data,
//! }
//! 
//! fn main() -> rschema::Result<()> {
//!     Schema::new::<AppConfig>("Application Config")
//!         .write_pretty("../schemas/config.schema.json")?;
//! 
//!     Ok(())
//! }
//! ```
//! 
//! This code generates the following JSON schema file.
//! 
//! ```json
//! {
//!   "title": "Application Config",
//!   "type": "object",
//!   "properties": {
//!     "name": {
//!       "title": "Application name",
//!       "type": "string"
//!     },
//!     "version": {
//!       "title": "Application version",
//!       "type": "string",
//!       "pattern": "^(0|[1-9][0-9]*)\\.(0|[1-9][0-9]*)\\.(0|[1-9][0-9]*)$"
//!     },
//!     "other_data": {
//!       "title": "Application data",
//!       "type": "object",
//!       "properties": {
//!         "test_flag": {
//!           "title": "Test flag",
//!           "description": "The flag whether for test.",
//!           "type": "boolean"
//!         }
//!       },
//!       "additionalProperties": true
//!     }
//!   },
//!   "required": [
//!     "name",
//!     "version"
//!   ],
//!   "additionalProperties": false
//! }
//! ```
//! 
//! # Attributes provided
//! 
//! - [**Container attributes**](#container-attributes) — apply to a struct or enum declaration.
//! - [**Variant attributes**](#variant-attributes) — apply to a variant of an enum.
//! - [**Field attributes**](#field-attributes) — apply to one field in a struct or in an enum variant.
//! 
//! See [Understanding JSON Schema](https://json-schema.org/understanding-json-schema/) for more information on each keywords.
//! 
//! 
//! ## Container attributes
//! 
//! - `#[rschema(additional_properties)]`
//! 
//!   Whether to allow properties not included in `properties`.
//! 
//! - `#[rschema(rename_all = "...")]`
//! 
//!   Rename all the fields of structs or **unit**-variants of enums according to the given case convention.
//! 
//!   The possible values:
//! 
//!   - `"lowercase"`
//!   - `"UPPERCASE"`
//!   - `"camelCase"`
//!   - `"PascalCase"`
//!   - `"kebab-case"`
//!   - `"Train-Case"`
//!   - `"COBOL-CASE"`
//!   - `"snake_case"`
//!   - `"UPPER_SNAKE_CASE"`
//!   - `"flatcase"`
//!   - `"UPPERFLATCASE"`
//! 
//!   **Note**: For enums, the `rename_all` attribute is only effective for unit variants. Because the other variants always behave as if the `flatten` attribute of *serde* is applied.
//! 
//! 
//! ## Variant attributes
//! 
//! Only for structural variants, you can apply [container attributes](#container-attributes) just like a normal structs.
//! 
//! 
//! ## Field attributes
//! 
//! Only the `title` keyword is required, the others are optional.
//! 
//! For keywords other than in [`Common`](#common), while it raises no errors to use attributes of another types, it doesn’t really make sense to do so.
//! 
//! If you want to skip, do not use attributes.
//! 
//! #### Common
//! 
//! - `#[rschema(title = "title")]`
//! 
//!   **Required**. The short description for the field.
//! 
//! - `#[rschema(description = "description")]`
//! 
//!   The more lengthy description for the field.
//! 
//! - `#[rschema(comment)]`
//! 
//!   The comment for this schema.
//! 
//! - `#[rschema(deprecated)]`
//! 
//!   Indicate that the property this keyword applies to should not be used and may be removed in the future.
//! 
//! - `#[rschema(required)]`
//! 
//!   Indicate that the property this keyword applies to is required.
//! 
//! 
//! #### `string`
//! 
//! - `#[rschema(min_length = 1)]`
//! 
//!   Specify the minimum length. Give an integer greater than or equal to 0.
//! 
//! - `#[rschema(max_length = 1)]`
//! 
//!   Specify the maximum length. Give an integer greater than or equal to 0.
//! 
//! - `#[rschema(pattern = "regular expressions")]`
//! 
//!   The regular expression to restrict a string. You should use a raw strings if necessary to avoid unnecessary escaping.
//! 
//! - `#[rschema(format = "format")]`
//! 
//!   The basic semantic identification of certain kinds of string values that are commonly used.
//! 
//! 
//! #### `number`
//! 
//! - `#[rschema(minimum = 1)]`
//! 
//!   Specify the minimum of the range.
//! 
//! - `#[rschema(maximum = 1)]`
//! 
//!   Specify the maximum of the range.
//! 
//! - `#[rschema(multiple_of = 1)]`
//! 
//!   Restrict the number to a multiple of a given number.
//! 
//! - `#[rschema(exclusive_minimum = 1)]`
//! 
//!   Specify the **exclusive** minimum of the range.
//! 
//! - `#[rschema(exclusive_maximum = 1)]`
//! 
//!   Specify the **exclusive** maximum of the range.
//! 
//! 
//! #### `array`
//! 
//! - `#[rschema(min_items = 1)]`
//! 
//!   Specify the minimum length of the array. Give an integer greater than or equal to 0.
//! 
//! - `#[rschema(max_items = 1)]`
//! 
//!   Specify the maximum length of the array. Give an integer greater than or equal to 0.
//! 
//! - `#[rschema(unique_items)]`
//! 
//!   Indicates that the array has unique values.
//! 
//! 
//! # Combination with Serde
//! 
//! *Rschema* is strongly intended to be used in combination with [*Serde*](https://serde.rs/).
//! 
//! For example, generate a JSON schema from structs and enums you define.
//! Data files validated by the JSON schema are always deserializable to the original structures!
//! 
//! 

pub use rschema_core::{
    AdditionalProperties,
    ArrayKeys,
    EnumKeys,
    Error,
    Items,
    ObjectKeys,
    Properties,
    Property,
    Type,
    Result,
    Schema,
    Schematic,
    StringKeys,
    TupleKeys,
};

#[allow(unused_imports)]
#[macro_use]
extern crate rschema_derive;
#[doc(hidden)]
pub use rschema_derive::*;
