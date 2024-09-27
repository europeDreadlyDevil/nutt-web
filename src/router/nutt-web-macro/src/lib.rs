use proc_macro::TokenStream;
use proc_macro2::Ident;
use syn::__private::quote::quote;
use syn::{FnArg, ItemFn, Lit, Pat, PatType};

fn get_fn_and_args_from_stream(attr: TokenStream, item: TokenStream) -> (ItemFn, Ident, String, Vec<FnArg>, Vec<Ident>) {
    let item = syn::parse::<ItemFn>(item.clone()).unwrap();
    let attr = syn::parse::<Lit>(attr).unwrap();
    let mut path = String::new();
    if let Lit::Str(lit) = attr {
        path = lit.value()
    }
    else { panic!("Path should be string") }
    let ident = item.clone().sig.ident;
    let args = item.clone().sig.inputs.into_iter().collect::<Vec<FnArg>>();
    let mut args_ident = vec![];
    for arg in &args {
        if let FnArg::Typed(PatType{pat,..}) = arg {
            if let Pat::Ident(ident) = *pat.clone() {
                args_ident.push(ident.ident.clone())
            }
        }
    }
    (item, ident, path, args, args_ident)
}

#[proc_macro_attribute]
pub fn get(attr: TokenStream, item: TokenStream) -> TokenStream {
    let (item, ident, path, args, args_ident) = get_fn_and_args_from_stream(attr, item);
    let stream = quote! {

        fn #ident() -> Route {
            use std::future::Future;
            use std::pin::Pin;
            use nutt_web::http::method::Method;
            use nutt_web::http::request::Request;
            let f = |req: Request| -> Pin<Box<dyn Future<Output = Response> + Send + Sync>> {
                #item
                #(
                    let #args = req.body_json().unwrap();
                )*
                Box::pin(#ident(#(#args_ident,)*))
            } as fn(Request) -> _;

            return Route::new(Method::GET, #path, f)
        }
    };

    stream.into()
}

#[proc_macro_attribute]
pub fn post(attr: TokenStream, item: TokenStream) -> TokenStream {
    let (item, ident, path, args, args_ident) = get_fn_and_args_from_stream(attr, item);
    let stream = quote! {

        fn #ident() -> Route {
            use std::future::Future;
            use std::pin::Pin;
            use nutt_web::http::method::Method;
            use nutt_web::http::request::Request;
            let f = |req: Request| -> Pin<Box<dyn Future<Output = Response> + Send + Sync>> {
                #item
                #(
                    let #args = req.body_json().unwrap();
                )*
                Box::pin(#ident(#(#args_ident,)*))
            } as fn(Request) -> _;

            return Route::new(Method::POST, #path, f)
        }
    };

    stream.into()
}