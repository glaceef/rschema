use indexmap::IndexMap;

use std::{
    any::TypeId,
    ops::{
        Deref,
        DerefMut,
    },
};

use crate::{
    Definitions,
    Schematic,
    Type,
};

type DefsMapItem = (&'static str, Type);
type InnerMap = IndexMap<TypeId, DefsMapItem>;

#[derive(Debug)]
pub struct DefinitionsMap {
    // 現状、名前を指定できるようになった場合に衝突を検知できない。
    pub map: InnerMap,
}

impl Deref for DefinitionsMap {
    type Target = InnerMap;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for DefinitionsMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

impl IntoIterator for DefinitionsMap {
    type Item = (TypeId, DefsMapItem);
    type IntoIter = indexmap::map::IntoIter<TypeId, DefsMapItem>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}

impl DefinitionsMap {
    pub fn new() -> Self {
        Self {
            map: IndexMap::new(),
        }
    }

    pub fn insert<T: 'static + Schematic>(
        &mut self,
        name: &'static str,
        def: Type,
    ) {
        let id = TypeId::of::<T>();
        self.entry(id).or_insert((name, def));
    }

    pub fn extend_ty<T: Schematic>(&mut self) {
        let definitions_map = <T as Schematic>::__defs_map();
        self.extend(definitions_map);
    }

    pub fn build(self) -> Definitions {
        Definitions::from_iter(self.map.into_values())
    }
}
