use proc_macro2::TokenStream as TokenStream2;
use quote::{
    ToTokens,
    quote,
};

use crate::{
    ContainerAttribute,
    Definitions,
    Field,
};

use super::FnTypeBody;

pub struct FnDefsMapBody {
    stmt_insert_self: Option<TokenStream2>,
    stmts: Vec<TokenStream2>,
}

impl ToTokens for FnDefsMapBody {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let ref stmt_insert_self = self.stmt_insert_self;
        let ref stmts = self.stmts;

        tokens.extend(quote! {
            let mut defs_map = rschema::DefinitionsMap::new();
            #stmt_insert_self
            #(
                #stmts
            )*
            defs_map
        });
    }
}

impl FnDefsMapBody {
    pub fn new(
        attr: &impl ContainerAttribute,
        fn_type_body: &mut FnTypeBody,
    ) -> Self {
        let stmt_insert_self = match attr.definitions() {
            Definitions::Named(name) => {
                // Defined in `$defs` with the given name.
                let def_name = quote! { #name };
                quote_stmt_insert_defs(def_name, fn_type_body)
            },
            Definitions::Auto => {
                // Defined in `$defs` with the auto-generated name.
                let def_name = quote! {
                    std::any::type_name::<Self>()
                };
                quote_stmt_insert_defs(def_name, fn_type_body)
            },
            Definitions::Skip => None,
        };

        Self {
            stmt_insert_self,
            stmts: vec![],
        }
    }

    pub fn empty() -> Self {
        Self {
            stmt_insert_self: None,
            stmts: vec![],
        }
    }

    pub fn with_stmts(
        attr: &impl ContainerAttribute,
        fn_type_body: &mut FnTypeBody,
        stmts: Vec<TokenStream2>,
    ) -> Self {
        let mut body = FnDefsMapBody::new(
            attr,
            fn_type_body,
        );
        body.stmts = stmts;

        body
    }

    pub fn with_fields(
        attr: &impl ContainerAttribute,
        fn_type_body: &mut FnTypeBody,
        fields: &[Field],
    ) -> Self {
        FnDefsMapBody::with_stmts(
            attr,
            fn_type_body,
            fields
                .iter()
                .map(quote_stmt_append_defs)
                .collect(),
        )
    }

    pub fn with_defs_maps(
        attr: &impl ContainerAttribute,
        fn_type_body: &mut FnTypeBody,
        defs_maps: Vec<FnDefsMapBody>,
    ) -> Self {
        FnDefsMapBody::with_stmts(
            attr,
            fn_type_body,
            defs_maps
                .iter()
                .map(stmt_extend_defs_map)
                .collect(),
        )
    }
}

fn quote_stmt_insert_defs(
    def_name: TokenStream2,
    fn_type_body: &mut FnTypeBody,
) -> Option<TokenStream2> {
    // Make `__type()` return `Type::Ref(def_name)`.
    let new_fn_type_body = FnTypeBody::Ref(def_name.clone());
    let def = std::mem::replace(fn_type_body, new_fn_type_body);

    // Instead, the original return value of `__type()` is defined in `$defs`.
    Some(quote! {
        defs_map.insert::<Self>(
            #def_name,
            #def,
        );
    })
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
        defs_map.extend_ty::<#ty>();
    }
}

fn stmt_extend_defs_map(
    defs_map: &FnDefsMapBody,
) -> TokenStream2 {
    quote! {
        defs_map.extend({
            #defs_map
        });
    }
}
