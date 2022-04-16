use proc_macro2::TokenStream as TokenStream2;
use quote::{
    ToTokens,
    quote,
};

use crate::{
    ContainerAttribute,
    Field,
};

use super::FnTypeBody2;

pub struct FnDefsMapBody {
    stmt_insert_self: Option<TokenStream2>,
    stmts: Vec<TokenStream2>,
}

impl ToTokens for FnDefsMapBody {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let ref stmt_insert_self = self.stmt_insert_self;
        let ref stmts = self.stmts;

        tokens.extend(quote! {
            let mut map = rschema::DefinitionsMap::new();
            #stmt_insert_self
            #(
                #stmts
            )*
            map
        });
    }
}

impl FnDefsMapBody {
    pub fn new(
        attr: &impl ContainerAttribute,
        fn_type_body: &mut FnTypeBody2,
    ) -> Self {
        // $defs に定義するかどうか
        let stmt_insert_self = attr.definitions().then(|| {
            // 名前を決定する記述。
            // 外部から渡すことになった場合はここを変える。
            let def_name = quote! {
                std::any::type_name::<Self>()
            };

            // __type() の返却値を Type::Ref に置き換える。
            let new_fn_type_body = FnTypeBody2::Ref(def_name.clone());
            let def = std::mem::replace(fn_type_body, new_fn_type_body);

            quote! {
                map.insert::<Self>(
                    #def_name,
                    #def,
                );
            }
        });

        Self {
            stmt_insert_self,
            stmts: vec![],
        }
    }

    pub fn with_fields(
        attr: &impl ContainerAttribute,
        fn_type_body: &mut FnTypeBody2,
        fields: &[Field],
    ) -> Self {
        let mut body = FnDefsMapBody::new(
            attr,
            fn_type_body,
        );

        body.stmts = fields
            .iter()
            .map(quote_stmt_append_defs)
            .collect();

        body
    }

    pub fn with_stmts(
        attr: &impl ContainerAttribute,
        fn_type_body: &mut FnTypeBody2,
        stmts: Vec<TokenStream2>,
    ) -> Self {
        let mut body = FnDefsMapBody::new(
            attr,
            fn_type_body,
        );

        body.stmts = stmts;

        body
    }
}

pub struct FnDefsMap {
    body: FnDefsMapBody,
}

impl ToTokens for FnDefsMap {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let ref body = self.body;

        tokens.extend(quote! {
            fn __defs_map() -> rschema::DefinitionsMap {
                #body
            }
        });
    }
}

impl FnDefsMap {
    pub fn new(body: FnDefsMapBody) -> Self {
        Self { body }
    }
}

fn quote_stmt_append_defs(Field{ ty, .. }: &Field) -> TokenStream2 {
    quote! {
        // このプロパティの型が持っている DefinitionsMap を取り込む。
        map.append::<#ty>();
    }
}
