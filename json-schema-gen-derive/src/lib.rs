use json_schema_gen_core::{
    Properties,
    Property,
};

use darling::FromAttributes;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    ItemStruct,
    parse_macro_input,
};

mod field;
use field::Field;

mod utils;

#[derive(Debug, FromAttributes)]
#[darling(attributes(schema))]
// #[darling(attributes(schema), forward_attrs(cfg))]
struct Schema {
    field: Field,

    #[darling(default)]
    required: bool,
}

#[proc_macro_derive(Schema, attributes(schema))]
pub fn derive(input: TokenStream) -> TokenStream {
    // Parse into AST
    let item = parse_macro_input!(input as ItemStruct);
    let struct_name = item.ident;

    let mut properties = Properties::new();
    let mut properties_registration_token_list = vec![];
    let mut required_props = vec![];

    for struct_field in item.fields {
        // タプル構造体の場合、フィールド名がないためNoneになる。
        let field_name = struct_field.ident.as_ref().unwrap().to_string();
        // dbg!(&field_name);

        // dbg!(&struct_field.attrs);
        let mut schema = match Schema::from_attributes(&struct_field.attrs) {
            Ok(v)  => v,
            Err(_) => continue,
        };
        // dbg!(&schema);

        let type_str = utils::get_type_str(&struct_field.ty);
        // dbg!(&type_str);
        schema.field.set_type(&type_str);

        let field_str = serde_json::to_string(&schema.field).unwrap();
        // let field = serde_json::to_string_pretty(&field).unwrap();
        // println!("{}", field);

        let property: Property = serde_json::from_str(&field_str).unwrap();
        // dbg!(&property);

        if schema.field.is_object() {
            // let field_name = struct_field.ident;
            let field_type = struct_field.ty;
            properties_registration_token_list.push(
                quote! {
                    let ref mut property = properties[#field_name];
                    property.set_properties(
                        <#field_type as ToProperties>::to_properties()
                    );
                }
            );
        }

        if schema.required {
            required_props.push(field_name.clone());
        }
        properties.insert(field_name, property); // field_attrをそのまま渡してもよいかもしれない
    }

    let properties_str = serde_json::to_string_pretty(&properties).unwrap();

    // dbg!(&properties_registration_token_list);
    dbg!(required_props);

    quote!{
        impl ToProperties for #struct_name {
            const PROPERTIES_STR: &'static str = #properties_str;

            fn to_properties() -> Properties {
                let mut properties = Self::restore_properties();

                #(
                    #properties_registration_token_list
                )*

                properties
            }
        }
    }.into()
}
