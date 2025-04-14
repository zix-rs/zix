use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemImpl, LitStr, Path, parse::Parser, Attribute};

/**
 * info!(
 *   name = "plugin",
 *   typo(grid, list, ...),
 *   desc = "description",
 *   sflag = "-p",
 *   flag = "--plugin",
 * )
 * SOON
**/

#[proc_macro_attribute]
pub fn plugin(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);
    let self_ty = &input.self_ty;

    let expanded = quote! {
        #input

        #[no_mangle]
        pub extern "C" fn create_plugin() -> *mut dyn ::zix_core::plugin::Plugin {
            let boxed: Box<dyn ::zix_core::plugin::Plugin> = Box::new(#self_ty::default());
            Box::into_raw(boxed)
        }
    };

    TokenStream::from(expanded)
}
