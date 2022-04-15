use indexmap::IndexMap;

use std::any::TypeId;

use crate::{
    Definitions,
    Schematic,
    Type,
};

#[derive(Debug)]
pub struct DefinitionsMap {
    // 現状、名前を指定できるようになった場合に衝突を検知できない。
    pub map: IndexMap<TypeId, (&'static str, Type)>,
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
        self.map
            .entry(id)
            .or_insert((name, def));
    }

    pub fn append<T: Schematic>(&mut self) {
        let definitions_map = <T as Schematic>::__defs_map();
        self.map.extend(definitions_map.map);
    }

    pub fn append2(&mut self, definitions_map: DefinitionsMap) {
        self.map.extend(definitions_map.map);
    }

    pub fn build(self) -> Definitions {
        Definitions::from_iter(self.map.into_values())
    }
}
