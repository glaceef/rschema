use proc_macro2::TokenStream as TokenStream2;
use quote::{
    ToTokens,
    quote,
};

use crate::{
    ContainerAttribute,
    Field,
    StructAttribute,
    TupleStructAttribute,
};

use super::*;

mod additional_properties;
mod items;
mod properties;
mod required;

pub use additional_properties::AdditionalProperties;
pub use items::Items;
pub use properties::Properties;
pub use required::Required;

pub enum FnTypeBody2<'a> {
    Struct {
        properties: Properties<'a>,
        required: Required<'a>,
        additional_properties: AdditionalProperties,
    },

    UnitStruct,

    NewTypeStruct(&'a Field<'a>),

    TupleStruct {
        items: Items<'a>,
        items_len: usize,
        unique_items: Option<bool>,
    },

    Enum {
        types: Vec<FnTypeBody2<'a>>,
        enum_units_ty: Option<TokenStream2>,
    },

    Ref(TokenStream2),
}

impl<'a> ToTokens for FnTypeBody2<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let token_stream = match self {
            Self::Struct {
                properties,
                required,
                additional_properties,
            } => quote! {
                rschema::Type::Object(rschema::ObjectKeys {
                    properties: #properties,
                    required: #required,
                    additional_properties: #additional_properties,
                })
            },

            Self::UnitStruct => quote! {
                rschema::Type::Null
            },

            Self::NewTypeStruct(field) => quote_ty(field),

            Self::TupleStruct {
                items,
                items_len,
                unique_items,
            } => {
                let unique_items = quote_option(unique_items);
                quote! {
                    rschema::Type::Array(rschema::ArrayKeys {
                        items: #items,
                        min_items: Some(#items_len),
                        max_items: Some(#items_len),
                        unique_items: #unique_items,
                    })
                }
            },

            Self::Enum {
                types,
                enum_units_ty,
            } => {
                match (types.is_empty(), enum_units_ty) {
                    ( true, None) => {
                        // Zero-variant enums are prevented in advance.
                        // So this message is never used.
                        unreachable!("Rschema does not support zero-variant enums.");
                    },
                    ( true, Some(ty)) => {
                        // Only unit variants
                        quote! { #ty }
                    },
                    _ => {
                        quote! {
                            rschema::Type::Enum(rschema::EnumKeys {
                                any_of: vec![
                                    #(
                                        #types,
                                    )*
                                    #enum_units_ty // Don't put a comma at the end.
                                ],
                            })
                        }
                    },
                }
            }

            Self::Ref(def_name) => quote! {
                rschema::Type::Ref(#def_name)
            },
        };

        tokens.extend(token_stream);
    }
}

impl<'a> FnTypeBody2<'a> {
    pub fn for_struct(
        attr: &'a (impl ContainerAttribute + StructAttribute),
        fields: &'a [Field<'a>],
    ) -> Self {
        let properties = Properties::new(attr, fields);
        let required = Required::new(fields);
        let additional_properties = AdditionalProperties::new(attr);

        Self::Struct {
            properties,
            required,
            additional_properties,
        }
    }

    pub fn for_newtype(
        field: &'a Field<'a>,
    ) -> Self {
        Self::NewTypeStruct(field)
    }

    pub fn for_tuple(
        attr: &'a (impl ContainerAttribute + TupleStructAttribute),
        fields: &'a [Field<'a>],
    ) -> Self {
        let items = Items::new(fields);
        let items_len = fields.len();
        let unique_items = attr.unique_items();

        Self::TupleStruct {
            items,
            items_len,
            unique_items,
        }
    }

    pub fn for_enum(
        types: Vec<FnTypeBody2<'a>>,
        enum_units_ty: Option<TokenStream2>,
    ) -> Self {
        Self::Enum {
            types,
            enum_units_ty,
        }
    }
}

pub struct FnType<'a> {
    body: FnTypeBody2<'a>,
}

impl<'a> ToTokens for FnType<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let ref body = self.body;

        tokens.extend(quote! {
            fn __type(
                min_length: Option<u64>,
                max_length: Option<u64>,
                pattern: Option<String>,
                format: Option<String>,
                minimum: Option<i64>,
                maximum: Option<i64>,
                multiple_of: Option<i64>,
                exclusive_minimum: Option<i64>,
                exclusive_maximum: Option<i64>,
                min_items: Option<usize>,
                max_items: Option<usize>,
                unique_items: Option<bool>,
            ) -> rschema::Type {
                #body
            }
        });
    }
}

impl<'a> FnType<'a> {
    pub fn new(body: FnTypeBody2<'a>) -> Self {
        Self { body }
    }
}
