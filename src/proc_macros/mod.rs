use proc_macro::TokenStream;

extern crate proc_macro;
use darling::FromMeta;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemFn, Signature};

#[derive(Debug, FromMeta)]
struct MacroArgs {
    max_attempts: u16,
    sleep_seconds: u64,
}

#[proc_macro_attribute]
pub fn retryable(args: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(args as AttributeArgs);
    let args = match MacroArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let max_attempts = args.max_attempts;
    let sleep_seconds = args.sleep_seconds;

    let function = parse_macro_input!(item as ItemFn);
    let function_signature = function.sig.clone();

    let ItemFn {
        attrs,
        vis,
        block,
        sig,
        ..
    } = function;

    let Signature {
        output: return_type,
        inputs: params,
        unsafety,
        asyncness,
        constness,
        abi,
        ident,
        generics:
            syn::Generics {
                params: gen_params,
                where_clause,
                ..
            },
        ..
    } = function_signature;

    if sig.asyncness.is_some() {
        quote!(
            #(#attrs) *
            #vis #constness #unsafety #asyncness #abi fn #ident<#gen_params>(#params) #return_type
            #where_clause
            {
                let mut counter: u16 = 0;
                loop {
                    match #block {
                        Ok(result) => return Ok(result),
                        Err(err) if counter < #max_attempts => {
                            counter += 1;
                            ::retryable::async_std::task::sleep(std::time::Duration::from_secs(#sleep_seconds)).await
                        },
                        Err(err) if counter == #max_attempts => {
                            break Err(err)
                        },
                        Err(err) => break Err(err)
                    }
                }
            }).into()
    } else {
        quote!(
        #(#attrs) *
        #vis #constness #unsafety #asyncness #abi fn #ident<#gen_params>(#params) #return_type
        #where_clause
        {
            let mut counter: u16 = 0;
            loop {
                match #block {
                    Ok(result) => return Ok(result),
                    Err(err) if counter < #max_attempts => {
                        counter += 1;
                        std::thread::sleep(std::time::Duration::from_secs(#sleep_seconds));
                    },
                    Err(err) if counter == #max_attempts => {
                        break Err(err)
                    },
                    Err(err) => break Err(err)
                }
            }

        })
        .into()
    }
}
