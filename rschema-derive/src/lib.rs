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
    create_customizing_properties_statement_token,
    extract_attribute,
    get_field_name,
    get_field_type_str,
};

#[proc_macro_derive(Schematic, attributes(rschema))]
pub fn derive_schematic(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemStruct);
    expand_derive_schematic(item)
        .unwrap_or_else(|e| e.write_errors().into() )
}

fn expand_derive_schematic(
    item: syn::ItemStruct,
) -> Result<TokenStream, darling::Error> {
    let struct_name = item.ident;
    let struct_generics = item.generics;
    let struct_attr = StructAttr::from_attributes(&item.attrs)?;

    let mut properties = Properties::new();
    let mut customizing_properties_statement_tokens = vec![];
    let mut required_props = vec![];

    for field in item.fields {
        // This is used to indicate the location of the error.
        let attr = match extract_attribute(&field.attrs, "rschema") {
            Some(v) => v,
            None => {
                // Skip if the `rschema` attribute is not used.
                continue;
            },
        };

        let field_name = get_field_name(&field).ok_or(
            darling::Error::custom("Tuple structs are not supported. Please specify the corresponding different type with the `type` property.")
        )?;

        let mut field_attr = FieldAttr::from_attributes(&field.attrs)
            .map_err(|err| err.with_span(&attr.path) )?;

        let field_type_str = get_field_type_str(&field)?;
        field_attr.field.set_type(&field_type_str);

        let field_str = serde_json::to_string(&field_attr.field).unwrap();
        let property: Property = serde_json::from_str(&field_str).unwrap();

        if field_attr.field.is_object() {
            customizing_properties_statement_tokens.push(
                create_customizing_properties_statement_token(
                    &field_name,
                    &field.ty,
                )
            );
        }

        if field_attr.required {
            required_props.push(field_name.clone());
        }
        properties.insert(field_name, property);
    }

    let properties_str = serde_json::to_string(&properties).unwrap();
    let additional_properties = struct_attr.additional_properties;

    let token = quote!{
        impl #struct_generics rschema::Schematic for #struct_name #struct_generics {
            const PROPERTIES_STR: &'static str = #properties_str;
            const REQUIRED: &'static[&'static str] = &[
                #(
                    #required_props,
                )*
            ];
            const ADDITIONAL_PROPERTIES: bool = #additional_properties;

            fn properties() -> rschema::Properties {
                let mut properties = Self::restore_properties();

                #(
                    #customizing_properties_statement_tokens
                )*

                properties
            }
        }
    }.into();

    Ok(token)
}
