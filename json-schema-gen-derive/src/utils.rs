pub fn get_type_str(ty: &syn::Type) -> String {
    if let syn::Type::Path(syn::TypePath{ ref path, .. }) = ty {
        if let Some(ident) = path.get_ident() {
            return ident.to_string();
        }
    }

    panic!("Can not retrieve field type: {:?}", ty);
}