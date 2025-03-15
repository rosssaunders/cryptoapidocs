use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Lit};

#[proc_macro_derive(ProcessorRegistration, attributes(processor, exchange))]
pub fn derive_processor_registration(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let name_str = name.to_string();
    let reg_name = name_str.chars()
        .enumerate()
        .map(|(i, c)| {
            if i > 0 && c.is_uppercase() { format!("_{}", c) } else { c.to_string() }
        })
        .collect::<String>()
        .to_uppercase();
    
    // Extract processor name from attributes
    let processor_name = input.attrs.iter()
        .find(|attr| attr.path().is_ident("processor"))
        .and_then(|attr| attr.parse_args::<Lit>().ok())
        .map(|lit| match lit {
            Lit::Str(s) => s.value(),
            _ => panic!("processor attribute must be a string literal"),
        })
        .unwrap_or_else(|| name.to_string().to_lowercase());

    // Extract exchange name from attributes (mandatory)
    let exchange_name = input.attrs.iter()
        .find(|attr| attr.path().is_ident("exchange"))
        .and_then(|attr| attr.parse_args::<Lit>().ok())
        .map(|lit| match lit {
            Lit::Str(s) => s.value(),
            _ => panic!("exchange attribute must be a string literal"),
        })
        .expect("exchange attribute is required");

    let registration_var = quote::format_ident!("__INVENTORY_REGISTRATION_{}", reg_name);
    
    let expanded = quote! {
        #[doc(hidden)]
        #[used]
        static #registration_var: std::sync::LazyLock<()> = std::sync::LazyLock::new(|| {
            inventory::submit! {
                crate::exchanges::processor_registry::ProcessorRegistration {
                    name: #processor_name,
                    exchange: #exchange_name,
                    create_fn: || Box::new(<#name>::default()),
                }
            }
        });
    };

    TokenStream::from(expanded)
}
