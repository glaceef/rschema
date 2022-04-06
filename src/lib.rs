//! # Rschema
//! 
//! *Rschema* provides a macro for generating JSON schemas from Rust structures.
//! 
//! # Example
//! 
//! ```rust
//! use rschema::{
//!     Schema,
//!     Schematic,
//! };
//! 
//! #[derive(Debug, Schematic)]
//! #[rschema(additional_properties)]
//! struct Data {
//!     #[rschema(field(
//!         title = "Test flag",
//!         description = "The flag whether for test.",
//!     ))]
//!     test_flag: bool,
//! }
//! 
//! #[derive(Debug, Schematic)]
//! struct AppConfig {
//!     #[rschema(
//!         field(
//!             title = "Application name",
//!         ),
//!         required,
//!     )]
//!     name: String,
//! 
//!     #[rschema(
//!         field(
//!             title = "Application version",
//!             pattern = r"^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$",
//!         ),
//!         required,
//!     )]
//!     version: String,
//! 
//!     #[rschema(field(
//!         title = "Application data",
//!         description = "This property is optional.",
//!     ))]
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
//! # Attributes
//! 
//! ## Common attributes
//! 
//! All types have the following attributes.
//! 
//! | Top-level attribute | Sub attribute | Type | Meaning |
//! | --- | --- | --- | --- |
//! | `field` | `title` | `String` | **Required**. The short description about the field. |
//! | | `description` | `String` | The more lengthy description about the field. |
//! | `required` | | `bool` | Whether the property is required or not. |
//! 
//! ## Dedicated attributes
//! 
//! Each types have dedicated attributes.
//! 
//! Using another type's attributes does not raise errors, but no meaning.
//! 
//! #### `String`
//! 
//! | Attribute | Type | Meaning |
//! | --- | --- | --- |
//! | `min_length` | `u64` | The minimum length. |
//! | `max_length` | `u64` | The maximum length. |
//! | `pattern` | `String` | The regular expression to restrict value. |
//! | `format` | `String` | The basic semantic identification of certain kinds of string values that are commonly used. |
//! 
//! #### `Number`
//! 
//! | Attribute | Type | Meaning |
//! | --- | --- | --- |
//! | `minimum` | `u64` | Specify the minimum. |
//! | `maximum` | `u64` | Specify the maximum. |
//! | `multiple_of` | `u64` | Numbers can be restricted to a multiple of a given number. |
//! | `exclusive_minimum` | `bool` | Specify the minimum of ranges. |
//! | `exclusive_maximum` | `bool` | Specify the maximum of ranges. |
//! 
//! #### `Array`
//! 
//! | Attribute | Type | Meaning |
//! | --- | --- | --- |
//! | `min_items` | `u64` | Specify the minimum length of the array. |
//! | `max_items` | `u64` | Specify the maximum length of the array. |

pub use rschema_core::{
    ArrayProp,
    EnumProp,
    Error,
    Items,
    ObjectProp,
    Properties,
    Property,
    PropType,
    Result,
    Schema,
    Schematic,
    StringProp,
    TupleProp,
};

#[allow(unused_imports)]
#[macro_use]
extern crate rschema_derive;
#[doc(hidden)]
pub use rschema_derive::*;
