use indexmap::IndexMap;

use std::{
    any::TypeId,
    collections::HashMap,
};

use crate::{
    Definitions,
    Schematic,
    Type,
};

#[derive(Debug)]
pub struct DefinitionsMap {
    pub map: IndexMap<TypeId, (String, Type)>,
}

impl DefinitionsMap {
    pub fn new() -> Self {
        Self {
            map: IndexMap::new(),
        }
    }

    // これは呼ばれないこともある
    pub fn insert<T: 'static + Schematic>(
        &mut self,
        name: String,
        def: Type,
    ) {
        // 自身の直接のプロパティについての情報を追加
        let id = TypeId::of::<T>();
        // IDが衝突した場合は上書きしないようにしているが、
        // カスタマイズされていると内容が異なるのでどちらも残さなければいけなそう。
        self.map.entry(id).or_insert((name, def));
    }

    // これはいったんすべてのプロパティについて呼び出す。
    // そのプロパティの型の definitions_map が空だったら何もしない。
    pub fn append<T: Schematic>(&mut self) {
        let definitions_map = <T as Schematic>::__defs_map();
        for (id, item) in definitions_map.map.into_iter() {
            self.map.insert(id, item);
        }
    }

    pub fn build(self) -> Definitions {
        let mut defs = Definitions::new();
        for (_id, (name, def)) in self.map.into_iter() {
            defs.insert(name, def);
        }
        defs
    }
}
