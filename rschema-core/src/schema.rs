use serde::{
    Serialize,
    Deserialize,
};

use crate::{
    PropType,
    Result,
    Schematic,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Schema {
    pub title: String,
    pub description: Option<String>,

    #[serde(flatten)]
    pub ty: PropType,
}

impl Schema {
    pub fn new<T, S>(title: S) -> Self
    where
        T: Schematic,
        S: Into<String>,
    {
        Schema {
            title: title.into(),
            description: None,
            ty: T::__type_no_attr(), // もしかしたらContainer Attributesで指定するかも
        }
    }

    pub fn description(
        &mut self,
        description: impl Into<String>
    ) -> &mut Self {
        self.description = Some(description.into());
        self
    }

    pub fn to_string(&self) -> Result<String> {
        let schema_str = serde_json::to_string(self)?;
        Ok(schema_str)
    }

    pub fn to_string_pretty(&self) -> Result<String> {
        let schema_str = serde_json::to_string_pretty(self)?;
        Ok(schema_str)
    }

    pub fn save(
        &self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<()> {
        let schema_str = self.to_string_pretty()?;
        std::fs::write(path, schema_str)?;
        Ok(())
    }
}
