use proc_macro2::TokenStream;
use quote::quote;

pub fn validate_field_attrs(attrs: &Vec<syn::Attribute>) -> bool {
    // no attributes
    if attrs.is_empty() {
        return false;
    }

    // whether the attribute `rschema` is used
    attrs.iter().any(|attr|{
        match attr.path.segments.last() {
            Some(path) if path.ident == "rschema" => true,
            _ => false,
        }
    })
}

pub fn extract_attribute<'a>(
    attrs: &'a Vec<syn::Attribute>,
    target: &(impl AsRef<str> + ?Sized),
) -> Option<&'a syn::Attribute> {
    // no attributes
    if attrs.is_empty() {
        return None;
    }

    // whether the attribute `rschema` is used
    for attr in attrs {
        match attr.path.segments.last() {
            Some(path) if path.ident == target => {
                return Some(attr);
            },
            _ => {},
        }
    }

    attrs.iter().find(|attr|{
        match attr.path.segments.last() {
            Some(path) if path.ident == target => true,
            _ => false,
        }
    })
}

pub fn get_field_name(field: &syn::Field) -> Option<String> {
    // For a tuple structs, returns `None` because the fields have no names.
    field.ident.as_ref().map(ToString::to_string)
}

pub fn get_field_type_str(field: &syn::Field) -> Result<String, darling::Error> {
    if let syn::Type::Path(syn::TypePath{ ref path, .. }) = field.ty {
        // `path.get_ident()` also returns its identifier,
        // but not correspond to a path like `path::to::Type`.
        if let Some(ty) = path.segments.last() {
            return Ok(ty.ident.to_string());
        }
    }

    Err(darling::Error::custom("Type read error.").with_span(&field.ty))
}

pub fn create_customizing_properties_statement_token(
    field_name: &str,
    field_type: &syn::Type,
) -> TokenStream {
    quote! {
        let ref mut property = properties[#field_name];

        // Each `set_xxx()` internally verifies whether the property is an object type.
        // Is `is_object()` unnecessary?
        if property.is_object() {
            property
                .set_properties::<#field_type>()
                .set_required::<#field_type>()
                .set_additional_properties::<#field_type>();
        }
    }
}