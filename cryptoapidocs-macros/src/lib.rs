use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Lit};

#[proc_macro_derive(ProcessorRegistration, attributes(processor))]
pub fn derive_processor_registration(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    // Extract processor name from attributes
    let processor_name = input.attrs.iter()
        .find(|attr| attr.path().is_ident("processor"))
        .and_then(|attr| attr.parse_args::<Lit>().ok())
        .map(|lit| match lit {
            Lit::Str(s) => s.value(),
            _ => panic!("processor attribute must be a string literal"),
        })
        .unwrap_or_else(|| name.to_string().to_lowercase());

    let registration_var = quote::format_ident!("__INVENTORY_REGISTRATION_{}", name);
    
    let expanded = quote! {
        #[doc(hidden)]
        #[allow(non_upper_case_globals)]
        const #registration_var: () = {
            inventory::submit! {
                crate::exchanges::processor_registry::ProcessorRegistration {
                    name: #processor_name,
                    create_fn: || Box::new(<#name>::default()),
                }
            }
        };
    };

    TokenStream::from(expanded)
}
