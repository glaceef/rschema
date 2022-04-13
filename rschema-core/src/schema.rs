use serde::Serialize;

use std::fs;

use crate::{
    Draft,
    Result,
    Schematic,
};

mod defs;
pub mod r#type;

pub use defs::Definitions;
pub use r#type::Type;

/// This is a structure representing the JSON schema itself.
/// 
/// ## Create Schema
/// 
/// See [the documentation top](https://docs.rs/rschema/latest/rschema/) for usage.
/// 
/// ## To JSON schema string
/// 
/// ```
/// #[derive(Debug, Schematic)]
/// struct Example {
///     #[rschema(field(
///         title = "Dummy",
///         description = "Dummy field",
///     ))]
///     dummy: String,
/// }
/// 
/// fn main() -> rschema::Result<()> {
///     let schema_str = Schema::new::<Example>("Example")
///         .to_string()?;
/// 
///     assert_eq!(
///         schema_str,
///         r#"{"title":"Example","type":"object","properties":{"dummy":{"title":"Dummy","description":"Dummy field","type":"string"}},"additionalProperties":false}"#
///     );
/// 
///     Ok(())
/// }
/// ``` 
/// 
/// Use [`to_string_pretty`](fn@Schema::to_string_pretty) to generate as a pretty-prited string.
/// 
/// 
#[derive(Debug, Serialize)]
pub struct Schema {
    #[serde(rename = "$schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Draft>,

    #[serde(rename = "$id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(flatten)]
    ty: Type,

    #[serde(rename = "$defs")]
    #[serde(skip_serializing_if = "Definitions::is_empty")]
    defs: Definitions,
}

impl Schema {
    /// Create a schema object from the given type `T`.
    /// 
    pub fn new<T: Schematic>(title: &str) -> Self {
        Schema {
            schema: None,
            id: None,
            title: title.into(),
            description: None,
            ty: T::__type_no_attr(),
            defs: T::__defs(),
        }
    }

    /// Add a description about this schema.
    /// 
    pub fn description(
        &mut self,
        description: impl Into<String>,
    ) -> &mut Self {
        self.description = Some(description.into());
        self
    }

    /// Specify `$schema`.
    /// 
    pub fn schema(
        &mut self,
        schema: Draft,
    ) -> &mut Self {
        self.schema = Some(schema);
        self
    }

    /// Specify `$id`.
    /// 
    pub fn id(
        &mut self,
        id: impl Into<String>,
    ) -> &mut Self {
        self.id = Some(id.into());
        self
    }

    /// Generate a JSON schema string.
    /// 
    /// # Errors
    /// 
    /// Internally calls `serde_json::to_string`, so this can fail if it fails. [Read more](https://docs.rs/serde_json/latest/serde_json/fn.to_string.html)
    /// 
    pub fn to_string(&self) -> Result<String> {
        let schema_str = serde_json::to_string(self)?;
        Ok(schema_str)
    }

    /// Generate a pretty-printed JSON schema string.
    /// 
    /// # Errors
    /// 
    /// Internally calls `serde_json::to_string_pretty`, so this can fail if it fails. [Read more](https://docs.rs/serde_json/latest/serde_json/fn.to_string_pretty.html)
    /// 
    pub fn to_string_pretty(&self) -> Result<String> {
        let schema_str = serde_json::to_string_pretty(self)?;
        Ok(schema_str)
    }

    /// Write a JSON schema string to a file.
    /// 
    /// # Errors
    /// 
    /// This call can fail if its own `to_string` call or writing to a file fails.
    /// 
    pub fn write(
        &self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<()> {
        let schema_str = self.to_string()?;
        fs::write(path, schema_str)?;

        Ok(())
    }

    /// Write a pretty-printed JSON schema string to a file.
    /// 
    /// # Errors
    /// 
    /// This call can fail if its own `to_string_pretty` call or writing to a file fails.
    /// 
    pub fn write_pretty(
        &self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<()> {
        let schema_str = self.to_string_pretty()?;
        fs::write(path, schema_str)?;

        Ok(())
    }
}
