use indexmap::IndexMap;

use super::Type;

pub type Definitions = IndexMap<&'static str, Type>;
