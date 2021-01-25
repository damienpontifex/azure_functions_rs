#![feature(box_patterns)]
use darling::FromMeta;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::{FnArg::Typed, PatType, Type};

#[derive(Debug, FromMeta)]
struct TimerTriggerInputs {
    #[darling(default)]
    name: String,
    #[darling(default)]
    schedule: String,
}

fn last_segment_in_path(path: &syn::Path) -> &syn::PathSegment {
    path.segments.last().expect("Expected at least one segment in path")
}

#[proc_macro_attribute]
pub fn timer_trigger(args: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as syn::ItemFn);
    let function_ident = input.sig.ident.clone();
    let vis = input.vis.clone();

    // Extract the trigger name used to construct the path the web route should handle
    let attr_args = parse_macro_input!(args as syn::AttributeArgs);
    let TimerTriggerInputs { name, .. } = match FromMeta::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return e.write_errors().into();
        }
    };

    for a in &input.sig.inputs {
        match a {
            Typed(PatType { ty, .. }) => {
                match ty.as_ref() {
                    Type::Path(syn::TypePath { path, .. }) => {
                        let arg_type_name = last_segment_in_path(&path);
                        println!("Got pat {:#?}", arg_type_name.ident);
                        println!("Got pat {:#?}", arg_type_name.ident == "TimerInfo");
                    },
                    Type::Reference(syn::TypeReference { mutability, elem, .. }) => {
                        if let Type::Path(syn::TypePath { path, .. }) = elem.as_ref() {
                            let arg_type_name = last_segment_in_path(path);
                            println!("Reference {:#?}, {:#?}", mutability, arg_type_name.ident);
                        }
                    },
                    _ => {},
                }
            }
            _ => {} //println!("Unknown input {:#?}", a),
        }
    }

    let user_fn_ident = quote::format_ident!("user_{}", function_ident);
    let service_path = format!("/{}", name);

    // Rename the user function such that our handler can call it and we can use the old name as
    // the web handler
    input.sig.ident = syn::Ident::new(&user_fn_ident.to_string(), proc_macro2::Span::call_site());
    input.vis = syn::Visibility::Inherited;

    let outer_function = quote! {
        #[actix_web::post(#service_path)]
        #vis async fn #function_ident((req, body): (actix_web::HttpRequest, actix_web::web::Json<func_types::TimerInfo>)) -> impl actix_web::Responder {
            println!("Before user call");
            let mut logger = func_types::Logger::default();
            #user_fn_ident(body.into_inner(), &mut logger);
            println!("After user call");
            format!("Done")
        }
    };

    let output = quote! {
        #outer_function
        #input
    };
    output.into()
}
