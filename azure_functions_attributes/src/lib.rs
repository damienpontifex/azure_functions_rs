#![feature(box_patterns)]
use darling::FromMeta;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::{FnArg::Typed, PatType, Type};
mod inputs;
use inputs::{TimerTriggerInputs, QueueTriggerInputs};

fn last_segment_in_path(path: &syn::Path) -> &syn::PathSegment {
    path.segments.last().expect("Expected at least one segment in path")
}

fn to_inputs(path_segment: &syn::PathSegment, _mutable: bool, _as_ref: bool) -> (Option<proc_macro2::TokenStream>, Option<proc_macro2::TokenStream>) {
    match path_segment.ident.to_string().as_str() {
        "TimerInfo" | "QueueTrigger" => (None, Some(quote!{ body.into_inner() })),
        "Logger" => (Some(quote! { let mut logger = azure_functions_types::Logger::default(); }), Some(quote! { &mut logger })),
        // TODO: handle panic better with ident name and span location
        _ => panic!("Unsupported argument of type {}", path_segment.ident.to_string()),
    }
}

fn has_parameter_of_type(func: &syn::ItemFn, type_name: &str) -> bool {
    func.sig.inputs.iter().any(|arg| {
        if let syn::FnArg::Typed(arg) = arg {
            match &*arg.ty {
                Type::Reference(tr) => {
                    if let Type::Path(tp) = &*tr.elem {
                        return last_segment_in_path(&tp.path).ident == type_name;
                    }
                }
                Type::Path(tp) => {
                    return last_segment_in_path(&tp.path).ident == type_name;
                }
                _ => {}
            }
        }
        false
    })
}

#[proc_macro_attribute]
pub fn timer_trigger(args: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as syn::ItemFn);
    let function_ident = input.sig.ident.clone();
    let vis = input.vis.clone();

    if let syn::ReturnType::Type(.., ret_type) = &input.sig.output {
        println!("Return type {:?}", ret_type);
    }

    // Extract the trigger name used to construct the path the web route should handle
    let attr_args = parse_macro_input!(args as syn::AttributeArgs);
    let TimerTriggerInputs { name, schedule, } = match FromMeta::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return e.write_errors().into();
        }
    };

    if !std::path::Path::new(&name).exists() {
        std::fs::create_dir(&name).unwrap();
        std::fs::write(format!("{}/function.json", name), serde_json::to_string_pretty(&serde_json::json!({
            "bindings": [
            {
                "name": "timer",
                "type": "timerTrigger",
                "direction": "in",
                "schedule": schedule
            }
            ]
        })).unwrap()).unwrap();

    }

    // Probably more useful where the trigger has a body such as Queue trigger
    // if !has_parameter_of_type(&input, "TimerInfo") {
    //     return syn::Error::new(proc_macro2::Span::call_site(), "Timer triggered function must have a TimerInfo argument").to_compile_error().into();
    // }

    let mut definitions = Vec::new();
    let mut arguments = Vec::new();

    for a in &input.sig.inputs {
        match a {
            Typed(PatType { ty, .. }) => {
                match ty.as_ref() {
                    Type::Path(syn::TypePath { path, .. }) => {
                        let arg_type_name = last_segment_in_path(&path);
                        let (def, arg) = to_inputs(arg_type_name, false, false);
                        if let Some(def) = def {
                            definitions.push(def);
                        }
                        if let Some(arg) = arg {
                            arguments.push(arg);
                        }
                    },
                    Type::Reference(syn::TypeReference { mutability, elem, .. }) => {
                        if let Type::Path(syn::TypePath { path, .. }) = elem.as_ref() {
                            let arg_type_name = last_segment_in_path(path);
                            let (def, arg) = to_inputs(arg_type_name, mutability.is_some(), true);
                            if let Some(def) = def {
                                definitions.push(def);
                            }
                            if let Some(arg) = arg {
                                arguments.push(arg);
                            }
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
        #vis async fn #function_ident((req, body): (actix_web::HttpRequest, actix_web::web::Json<azure_functions_types::TimerInfo>)) -> actix_web::Result<actix_web::HttpResponse> {
            #(#definitions;)*
            #user_fn_ident(#(#arguments,)*);
            let ret_body = azure_functions_types::FuncResponse::default();
            Ok(actix_web::HttpResponse::Ok()
                .content_type("application/json")
                .json(ret_body))
        }
    };

    let output = quote! {
        #outer_function
        #input
    };
    output.into()
}


#[proc_macro_attribute]
pub fn event_grid_trigger(_args: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn blob_storage_trigger(_args: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn notification_hub_trigger(_args: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn queue_trigger(args: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as syn::ItemFn);

    if !has_parameter_of_type(&input, "QueueTrigger") {
        return syn::Error::new(proc_macro2::Span::call_site(), "Queue triggered function must have a QueueTrigger argument").to_compile_error().into();
    }

    let function_ident = input.sig.ident.clone();
    let vis = input.vis.clone();

    let user_fn_ident = quote::format_ident!("user_{}", function_ident);

    // Rename the user function such that our handler can call it and we can use the old name as
    // the web handler
    input.sig.ident = syn::Ident::new(&user_fn_ident.to_string(), proc_macro2::Span::call_site());
    input.vis = syn::Visibility::Inherited;

    // Extract the trigger name used to construct the path the web route should handle
    let attr_args = parse_macro_input!(args as syn::AttributeArgs);
    let QueueTriggerInputs { name, .. } = match FromMeta::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return e.write_errors().into();
        }
    };
    let service_path = format!("/{}", name);

    let outer_function = quote! {
        #[actix_web::post(#service_path)]
        #vis async fn #function_ident((req, body): (actix_web::HttpRequest, actix_web::web::Json<azure_functions_types::TimerInfo>)) -> actix_web::Result<actix_web::HttpResponse> {
            let ret_body = azure_functions_types::FuncResponse::default();
            Ok(actix_web::HttpResponse::Ok()
                .content_type("application/json")
                .json(ret_body))
        }
    };

    let output = quote! {
        #outer_function
        #input
    };
    output.into()
}
