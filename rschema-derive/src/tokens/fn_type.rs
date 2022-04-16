use proc_macro2::TokenStream as TokenStream2;
use quote::{
    ToTokens,
    quote,
};

use crate::{
    Case,
    ContainerAttribute,
    EnumAttribute,
    Field,
    StructAttribute,
    TupleStructAttribute,
    Variant,
};

use super::utils::{
    self,
    quote_ty,
    rename_ident,
};

mod additional_properties;
mod items;
mod properties;
mod required;
mod unique_items;

pub use additional_properties::AdditionalProperties;
pub use items::Items;
pub use properties::Properties;
pub use required::Required;
pub use unique_items::UniqueItems;

pub enum FnTypeBody<'a> {
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
        unique_items: UniqueItems,
    },

    Enum {
        types: Vec<FnTypeBody<'a>>,
        enum_units_type: Option<TokenStream2>,
    },

    Ref(TokenStream2),
}

impl<'a> ToTokens for FnTypeBody<'a> {
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
                enum_units_type,
            } => {
                match (types.is_empty(), enum_units_type) {
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
                                    #enum_units_type // Don't put a comma at the end.
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

impl<'a> FnTypeBody<'a> {
    pub fn for_struct(
        attr: &'a (impl ContainerAttribute + StructAttribute),
        fields: &'a [Field],
    ) -> Self {
        Self::Struct {
            properties: Properties::new(attr, fields),
            required: Required::new(fields),
            additional_properties: AdditionalProperties::new(attr),
        }
    }

    pub fn for_newtype(
        field: &'a Field,
    ) -> Self {
        Self::NewTypeStruct(field)
    }

    pub fn for_tuple(
        attr: &'a (impl ContainerAttribute + TupleStructAttribute),
        fields: &'a [Field],
    ) -> Self {
        Self::TupleStruct {
            items: Items::new(fields),
            items_len: fields.len(),
            unique_items: UniqueItems::new(attr),
        }
    }

    pub fn for_enum(
        attr: &impl EnumAttribute,
        variants: &[Variant],
        types: Vec<FnTypeBody<'a>>,
    ) -> Self {
        let enum_units_type = quote_enum_units_type(attr, variants);

        Self::Enum {
            types,
            enum_units_type,
        }
    }
}

pub struct FnType<'a> {
    body: FnTypeBody<'a>,
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
    pub fn new(body: FnTypeBody<'a>) -> Self {
        Self { body }
    }
}

fn unit_ident(
    variant: &Variant,
    rename_all: Option<Case>,
) -> Option<String> {
    variant.is_unit().then(|| {
        rename_ident(
            &variant.ident,
            variant.attr.rename.as_ref(),
            rename_all,
        )
    })
}

fn quote_enum_units_type(
    attr: &impl EnumAttribute,
    variants: &[Variant],
) -> Option<TokenStream2> {
    let idents: Vec<String> = variants
        .iter()
        .filter_map(|variant| unit_ident(variant, attr.rename_all()))
        .collect();

    // ユニットバリアントが存在しない場合、トークンを埋め込まない。
    if idents.is_empty() {
        return None;
    }

    Some(quote! {
        rschema::Type::String(rschema::StringKeys {
            enm: vec![
                #(
                    #idents.into(),
                )*
            ],
            ..Default::default()
        })
    })
}
