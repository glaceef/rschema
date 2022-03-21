use rschema_core::{
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

mod field_attr;
use field_attr::FieldAttr;

mod struct_attr;
use struct_attr::StructAttr;

mod utils;
use utils::{
    get_field_name,
    get_field_type,
};

#[proc_macro_derive(Schematic, attributes(rschema))]
pub fn derive_schema(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemStruct);

    let struct_name = item.ident;
    let struct_attr = StructAttr::from_attributes(&item.attrs).unwrap();

    let mut properties = Properties::new();
    let mut customizing_properties_tokens = vec![];
    let mut required_props = vec![];

    for field in item.fields {
        // タプル構造体の場合、フィールド名がないためNoneになる。
        let field_name = get_field_name(&field);
        let mut field_attr = match FieldAttr::from_attributes(&field.attrs) {
            Ok(v)  => v,
            Err(_) => continue,
        };

        let field_type_str = get_field_type(&field);
        field_attr.field.set_type(&field_type_str);

        let field_str = serde_json::to_string(&field_attr.field).unwrap();
        let property: Property = serde_json::from_str(&field_str).unwrap();

        if field_attr.field.is_object() {
            let field_type = field.ty;
            customizing_properties_tokens.push(
                quote! {
                    let ref mut property = properties[#field_name];
                    property.set_properties(
                        <#field_type as Schematic>::properties()
                    );
                    property.set_required(
                        <#field_type as Schematic>::REQUIRED
                    );
                    property.set_additional_properties(
                        <#field_type as Schematic>::ADDITIONAL_PROPERTIES
                    );
                }
            );
        }

        if field_attr.required {
            required_props.push(field_name.clone());
        }
        properties.insert(field_name, property); // field_attrをそのまま渡してもよいかもしれない
    }

    let properties_str = serde_json::to_string_pretty(&properties).unwrap();
    let additional_properties = struct_attr.additional_properties;

    quote!{
        impl rschema::Schematic for #struct_name {
            const PROPERTIES_STR: &'static str = #properties_str;
            const REQUIRED: &'static[&'static str] = &[
                #(
                    #required_props,
                )*
            ];
            const ADDITIONAL_PROPERTIES: bool = #additional_properties;

            fn properties() -> Properties {
                let mut properties = Self::restore_properties();

                #(
                    #customizing_properties_tokens
                )*

                properties
            }
        }
    }.into()
}
