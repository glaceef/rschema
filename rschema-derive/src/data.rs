use darling::{
    FromAttributes,
    FromField,
};
use syn::punctuated::Punctuated;

mod field;
mod field_attr;
mod variant;
mod variant_attr;

pub use field::Field;
pub use field_attr::FieldAttr;
pub use variant::Variant;
pub use variant_attr::{
    StructVariantAttr,
    EmptyVariantAttr,
};

#[derive(Debug)]
pub enum Data {
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

    // enum
    Enum(Vec<Variant>),
}    

impl Data {
    pub fn struct_from_ast(fields: &syn::Fields) -> darling::Result<Self> {
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
                let field = fields_from_ast(&fields.unnamed)?.pop().unwrap();
                Data::NewTypeStruct(field)
            },

            // そのほかのタプル構造体
            // 順序のある複合型の配列。
            syn::Fields::Unnamed(ref fields) => {
                Data::TupleStruct(fields_from_ast(&fields.unnamed)?)
            },
        })
    }

    pub fn enum_from_ast(
        variants: &Punctuated<syn::Variant, syn::Token![,]>,
    ) -> darling::Result<Self> {
        if variants.is_empty() {
            return Err(darling::Error::custom("Rschema does not support zero-variant enums"));
        }

        let variants: darling::Result<Vec<Variant>> = variants
            .iter()
            .map(Self::parse_variant)
            .collect();

        Ok(Data::Enum(variants?))
    }

    fn parse_variant(variant: &syn::Variant) -> darling::Result<Variant> {
        let attr = match variant.fields {
            // struct variant
            syn::Fields::Named(_) => {
                StructVariantAttr::from_attributes(&variant.attrs)?.into()
            },

            // other variant
            _ => {
                EmptyVariantAttr::from_attributes(&variant.attrs)?.into()
            },
        };

        Self::struct_from_ast(&variant.fields)
            .map(|data| Variant {
                attr,
                ident: variant.ident.clone(),
                data,
            })
    }
}

fn parse_field(field: &syn::Field) -> darling::Result<Field> {
    FieldAttr::from_field(field)
        .map(|attr| Field {
            attr,
            ident: field.ident.clone(), // 参照はできるか？
            ty: field.ty.clone(),
        })
}

fn fields_from_ast(
    fields: &Punctuated<syn::Field, syn::Token![,]>,
) -> darling::Result<Vec<Field>> {
    fields
        .iter()
        .map(parse_field)
        .collect()
}
