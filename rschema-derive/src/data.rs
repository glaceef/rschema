use darling::{
    FromAttributes,
    FromField,
};
use syn::punctuated::Punctuated;

use crate::{
    // container_attr::ContainerAttr,
    variant_attr::{
        StructVariantAttr,
        UnitVariantAttr,
    },
};

mod field;
mod field_attr;
mod variant;

pub use field::Field;
pub use field_attr::FieldAttr;
pub use variant::Variant;

#[derive(Debug)]
pub enum Data {
    // enum
    Enum(Vec<Variant>),

    // Simple structure.
    // e.x.) struct Data { ... }
    Struct(Vec<Field>),

    // Unit structure.
    // e.x.) struct Unit;
    UnitStruct,

    // The tuple structure that has just one field.
    // e.x.) struct Tuple(Ty);
    NewTypeStruct(Field),

    // The tuple structure that has multiple fields.
    // e.x.) struct Tuple(Ty1, Ty2, ...);
    TupleStruct(Vec<Field>),
}

impl Data {
    pub fn enum_from_ast(
        variants: &Punctuated<syn::Variant, syn::Token![,]>,
    ) -> darling::Result<Self> {
        if variants.is_empty() {
            return Err(darling::Error::custom("Rschema does not support zero-variant enums"));
        }

        let variants: darling::Result<Vec<Variant>> = variants
            .iter()
            .map(|variant| {
                let attr = match variant.fields {
                    // struct variant
                    syn::Fields::Named(_) => {
                        StructVariantAttr::from_attributes(&variant.attrs)?.into()
                    },

                    // unit / newtype / tuple variant
                    _ => {
                        UnitVariantAttr::from_attributes(&variant.attrs)?.into()
                    },
                };

                Self::struct_from_ast(&variant.fields)
                    .map(|data| Variant {
                        attr,
                        ident: variant.ident.clone(),
                        data,
                    })
            })
            .collect();
        Ok(Data::Enum(variants?))
    }
    
    pub fn struct_from_ast(fields: &syn::Fields) -> darling::Result<Self> {
        Ok(match fields {
            // 通常の構造体
            syn::Fields::Named(ref fields) => {
                Data::Struct(named_fields_from_ast(&fields.named)?)
            },

            // Noneのようなユニット構造体
            syn::Fields::Unit => {
                Data::UnitStruct
            },

            // フィールド１つのタプル構造体
            // 中のデータ型として扱う。
            syn::Fields::Unnamed(ref fields) if fields.unnamed.len() == 1 => {
                // 現状は、バリアントはアトリビュートの有無にかかわらず対象とするため、
                // .pop().unwrap() は必ず成功する。
                let field = unnamed_fields_from_ast(&fields.unnamed).pop().unwrap();
                Data::NewTypeStruct(field)
            },

            // そのほかのタプル構造体
            // 順序のある複合型の配列。
            syn::Fields::Unnamed(ref fields) => {
                Data::TupleStruct(unnamed_fields_from_ast(&fields.unnamed))
            },
        })
    }
}

fn check_attr_exists<'a>(
    attrs: &'a Vec<syn::Attribute>,
) -> Option<&'a syn::Attribute> {
    // Return its path if the attribute `rschema` is used
    attrs.iter().find(|attr|{
        match attr.path.segments.last() {
            Some(path) if path.ident == "rschema" => true,
            _ => false,
        }
    })
}

fn parse_named_field(field: &syn::Field) -> Option<darling::Result<Field>> {
    let attr = check_attr_exists(&field.attrs)?;
    let field = FieldAttr::from_field(field)
        .map(|v| Field {
            attr: Some(v),
            ident: field.ident.clone(), // 参照はできるか？
            ty: field.ty.clone(),
        })
        .map_err(|err| err.with_span(&attr.path) );
    Some(field)
}

fn named_fields_from_ast(
    fields: &Punctuated<syn::Field, syn::Token![,]>,
) -> darling::Result<Vec<Field>> {
    fields
        .iter()
        .filter_map(parse_named_field)
        .collect()
}

fn parse_unnamed_field(field: &syn::Field) -> Field {
    Field {
        attr: None,
        ident: field.ident.clone(), // 参照はできるか？
        ty: field.ty.clone(),
    }
}

fn unnamed_fields_from_ast(
    fields: &Punctuated<syn::Field, syn::Token![,]>,
) -> Vec<Field> {
    fields
        .iter()
        .map(parse_unnamed_field)
        .collect()
}
