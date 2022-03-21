pub fn get_field_name(field: &syn::Field) -> String {
    if let Some(ref ident) = field.ident {
        return ident.to_string();
    }

    panic!("Could not retrieve field type: {:?}", field);
}

pub fn get_field_type(field: &syn::Field) -> String {
    if let syn::Type::Path(syn::TypePath{ ref path, .. }) = field.ty {
        if let Some(ident) = path.get_ident() {
            return ident.to_string();
        }
    }

    panic!("Could not retrieve field type: {:?}", field);
}