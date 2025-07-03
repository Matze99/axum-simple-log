extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn log_request(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_inputs = &input_fn.sig.inputs;
    let fn_output = &input_fn.sig.output;
    let fn_body = &input_fn.block;

    // Extract parameter names for logging
    let param_logs: Vec<_> = fn_inputs
        .iter()
        .filter_map(|input| {
            if let syn::FnArg::Typed(pat_type) = input {
                if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                    let param_name = &pat_ident.ident;
                    return Some(quote! {
                        log::info!("  Parameter {}: {:?}", stringify!(#param_name), #param_name);
                    });
                }
            }
            None
        })
        .collect();

    let expanded = quote! {
        pub async fn #fn_name (#fn_inputs) #fn_output {
            log::info!("Executing Endpoint: {}", stringify!(#fn_name));
            #(#param_logs)*
            let result = (async #fn_body).await;
            log::info!("Response from {}: {:?}", stringify!(#fn_name), result);
            log::info!("Completed Endpoint: {}", stringify!(#fn_name));
            result
        }
    };

    expanded.into()
}
