use proc_macro2::TokenStream as TokenStream2;
use quote::{
    ToTokens,
    quote,
};

use crate::Field;

mod fn_type;
pub use fn_type::*;

mod fn_defs_map;
pub use fn_defs_map::*;

mod utils;
pub(self) use utils::*;

// struct ImplSchematic {
//     fn_type: FnType,
//     fn_defs_map: FnDefsMap,
// }

/*
mod fn_defs_map {
    use super::*;

    struct FnDefsMapBody;

    struct FnDefsMap {
        body: FnDefsMapBody,
    }
}
*/

pub struct CallType<'a>(pub &'a Field<'a>);

impl<'a> ToTokens for CallType<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Field { attr, ty, .. } = self.0;

        let min_length = quote_option(&attr.min_length);
        let max_length = quote_option(&attr.max_length);
        let format = quote_option_str(&attr.format);
        let pattern = quote_option_str(&attr.pattern);
        let minimum = quote_option(&attr.minimum);
        let maximum = quote_option(&attr.maximum);
        let multiple_of = quote_option(&attr.multiple_of);
        let exclusive_minimum = quote_option(&attr.exclusive_minimum);
        let exclusive_maximum = quote_option(&attr.exclusive_maximum);
        let min_items = quote_option(&attr.min_items);
        let max_items = quote_option(&attr.max_items);
        let unique_items = quote_option(&attr.unique_items);

        tokens.extend(quote! {
            <#ty as Schematic>::__type(
                #min_length,
                #max_length,
                #pattern,
                #format,
                #minimum,
                #maximum,
                #multiple_of,
                #exclusive_minimum,
                #exclusive_maximum,
                #min_items,
                #max_items,
                #unique_items,
            )
        });
    }
}
