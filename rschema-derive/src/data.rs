use darling::{
    FromAttributes,
    FromField,
};
use syn::punctuated::Punctuated;

use crate::is_falsy;

mod field;
mod field_attr;
mod variant;
mod variant_attr;

pub use field::Field;
pub use field_attr::FieldAttr;
pub use variant::Variant;
pub use variant_attr::{
    OtherVariantAttr,
    StructVariantAttr,
    TupleStructVariantAttr,
    UnitVariantAttr,
    VariantAttr,
};

#[derive(Debug, PartialEq)]
pub enum Data<'a> {
    // Simple structure.
    // e.x.) struct Data { ... }
    Struct(Vec<Field<'a>>),
    
    // Unit structure.
    // e.x.) struct Unit;
    UnitStruct,
    
    // The tuple structure that has just one field.
    // e.x.) struct Tuple(Ty);
    NewTypeStruct(Field<'a>),
    
    // The tuple structure that has multiple fields.
    // e.x.) struct Tuple(Ty1, Ty2, ...);
    TupleStruct(Vec<Field<'a>>),

    // enum
    Enum(Vec<Variant<'a>>),
}    

impl<'a> Data<'a> {
    pub fn struct_from_ast(
        fields: &'a syn::Fields,
    ) -> darling::Result<Self> {
        Ok(match fields {
            // 通常の構造体
            syn::Fields::Named(ref fields) => {
                Data::Struct(fields_from_ast(&fields.named)?)
            },

            // Noneのようなユニット構造体
            syn::Fields::Unit => {
                Data::UnitStruct
            },

            // フィールド１つのタプル構造体
            // 中のデータ型として扱う。
            syn::Fields::Unnamed(ref fields) if fields.unnamed.len() == 1 => {
                match fields_from_ast(&fields.unnamed)?.pop() {
                    Some(field) => Data::NewTypeStruct(field),
                    None => {
                        // This is no longer a NewTypeStruct, but an empty TupleStruct.
                        Data::TupleStruct(vec![])
                    },
                }
            },

            // そのほかのタプル構造体
            // 順序のある複合型の配列。
            syn::Fields::Unnamed(ref fields) => {
                Data::TupleStruct(fields_from_ast(&fields.unnamed)?)
            },
        })
    }

    pub fn enum_from_ast(
        variants: &'a Punctuated<syn::Variant, syn::Token![,]>,
    ) -> darling::Result<Self> {
        if variants.is_empty() {
            return Err(darling::Error::custom("Rschema does not support zero-variant enums"));
        }

        let variants = variants_from_ast(variants)?;
        if variants.is_empty() {
            return Err(darling::Error::custom("Don't skip all variants."));
        }

        Ok(Data::Enum(variants))
    }
}

fn parse_field(
    field: &syn::Field,
) -> darling::Result<Option<Field>> {
    let attr = FieldAttr::from_field(field)?;

    // In most cases, It is not recommended to skip unnamed fields.
    // However, Rschema does not check it. Because there might be a reason.
    let field = is_falsy(&attr.skip).then(|| {
        // Treat this field as the given type.
        let ty = match attr.alt {
            Some(ref alt) => alt.clone().into(),
            None => field.ty.clone(),
        };

        Field {
            attr,
            ident: field.ident.as_ref(),
            ty,
        }
    });
    Ok(field)
}

fn fields_from_ast(
    fields: &Punctuated<syn::Field, syn::Token![,]>,
) -> darling::Result<Vec<Field>> {
    fields
        .iter()
        .filter_map(|field| parse_field(field).transpose())
        .collect()
}

fn parse_variant<'a>(
    variant: &'a syn::Variant,
) -> darling::Result<Option<Variant>> {
    let attr: VariantAttr = match variant.fields {
        // struct variant
        syn::Fields::Named(_) => {
            StructVariantAttr::from_attributes(&variant.attrs)?.into()
        },

        // unit variant
        syn::Fields::Unit => {
            UnitVariantAttr::from_attributes(&variant.attrs)?.into()
        }

        // tuple struct variant
        syn::Fields::Unnamed(ref fields) if fields.unnamed.len() > 1 => {
            TupleStructVariantAttr::from_attributes(&variant.attrs)?.into()
        },

        // empty tuple struct variant / newtype variant
        _ => OtherVariantAttr::from_attributes(&variant.attrs)?.into(),
    };

    if !is_falsy(&attr.skip) {
        return Ok(None);
    }

    Data::struct_from_ast(&variant.fields)
        .map(|data| Some(Variant {
            attr,
            ident: &variant.ident,
            data,
        }))
}

fn variants_from_ast(
    variants: &Punctuated<syn::Variant, syn::Token![,]>,
) -> darling::Result<Vec<Variant>> {
    variants
        .iter()
        .filter_map(|variant| parse_variant(variant).transpose())
        .collect()
}
