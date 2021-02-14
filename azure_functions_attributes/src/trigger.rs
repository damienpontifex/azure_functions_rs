use darling::FromMeta;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::{FnArg::Typed, PatType, Type};
use crate::inputs::Binding;

fn last_segment_in_path(path: &syn::Path) -> &syn::PathSegment {
    path.segments.last().expect("Expected at least one segment in path")
}

fn to_inputs(path_segment: &syn::PathSegment, _mutable: bool, _as_ref: bool) -> (String, Option<proc_macro2::TokenStream>, Option<proc_macro2::TokenStream>) {
    match path_segment.ident.to_string().as_str() {
        "TimerInfo" | "QueueTrigger" => (path_segment.ident.to_string(), None, Some(quote!{ body.into_inner() })),
        "Logger" => (path_segment.ident.to_string(), Some(quote! { let mut logger = azure_functions_types::Logger::default(); }), Some(quote! { &mut logger })),
        // TODO: handle panic better with ident name and span location
        _ => panic!("Unsupported argument of type {}", path_segment.ident.to_string()),
    }
}

fn get_trigger_type(func: &syn::ItemFn, type_name: &str) -> Option<syn::TypePath> {
    func.sig.inputs.iter().map(|arg| {
        if let syn::FnArg::Typed(arg) = arg {
            match &*arg.ty {
                Type::Reference(tr) => {
                    if let Type::Path(tp) = &*tr.elem {
                        let ident = &last_segment_in_path(&tp.path).ident;
                        if ident == type_name {
                            return Some(tp.clone());
                        }
                    }
                }
                Type::Path(tp) => {
                    let ident = &last_segment_in_path(&tp.path).ident;
                    if ident == type_name {
                        return Some(tp.clone());
                    }
                }
                _ => {}
            }
        }
        None
    }).filter(|x| x.is_some()).next().flatten()
}

pub(crate) fn impl_trigger<A>(args: TokenStream, item: TokenStream, trigger_type: &str) -> TokenStream where A: Binding + FromMeta {
    let mut input = parse_macro_input!(item as syn::ItemFn);
    let function_ident = input.sig.ident.clone();
    let vis = input.vis.clone();

    let trigger_type_ident = get_trigger_type(&input, trigger_type);
    if trigger_type_ident.is_none() {
        return syn::Error::new(proc_macro2::Span::call_site(), format!("Must have a {} argument", trigger_type)).to_compile_error().into();
    }
    let trigger_type_ident = trigger_type_ident.unwrap();

    let mut returns_result_type = false;
    // TODO: handle $return type out bindings
    if let syn::ReturnType::Type(.., ret_type) = &input.sig.output {
        if let syn::Type::Path(syn::TypePath { path, .. }) = ret_type.as_ref() {
            if last_segment_in_path(&path).ident == "Result" {
                returns_result_type = true;
            }
        }
    }

    // Extract the trigger name used to construct the path the web route should handle
    let attr_args = parse_macro_input!(args as syn::AttributeArgs);
    let trigger_inputs: A = match FromMeta::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return e.write_errors().into();
        }
    };

    let name = trigger_inputs.function_name();

    if !std::path::Path::new(&name).exists() {
        std::fs::create_dir(&name).unwrap();
        std::fs::write(format!("{}/function.json", name), serde_json::to_string_pretty(&serde_json::json!({
            "bindings": [
                trigger_inputs.generate_json()
            ]
        })).unwrap()).unwrap();
    }

    let mut definitions = Vec::new();
    let mut arguments = Vec::new();
    let mut has_logger = false;

    // Match types for user function arguments to be passed through
    for a in &input.sig.inputs {
        match a {
            Typed(PatType { ty, .. }) => {
                match ty.as_ref() {
                    Type::Path(syn::TypePath { path, .. }) => {
                        let arg_type_name = last_segment_in_path(&path);
                        let (type_name, def, arg) = to_inputs(arg_type_name, false, false);
                        if let Some(def) = def {
                            definitions.push(def);
                        }
                        if let Some(arg) = arg {
                            arguments.push(arg);
                        }
                        if type_name == "Logger" {
                            has_logger = true;
                        }
                    },
                    Type::Reference(syn::TypeReference { mutability, elem, .. }) => {
                        if let Type::Path(syn::TypePath { path, .. }) = elem.as_ref() {
                            let arg_type_name = last_segment_in_path(path);
                            let (type_name, def, arg) = to_inputs(arg_type_name, mutability.is_some(), true);
                            if let Some(def) = def {
                                definitions.push(def);
                            }
                            if let Some(arg) = arg {
                                arguments.push(arg);
                            }
                            if type_name == "Logger" {
                                has_logger = true;
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

    // let trigger_type_ident = quote::format_ident!("{}", trigger_type);

    let mut user_fn_invocation = quote! {
        #user_fn_ident(#(#arguments,)*);
    };
    if returns_result_type {
        user_fn_invocation = quote! {
            let result = #user_fn_invocation
        };
    }

    let response = if returns_result_type {
        quote! {
            match result {
                Ok(_) => {
                    actix_web::HttpResponse::Ok()
                },
                Err(_) => {
                    actix_web::HttpResponse::InternalServerError()
                },
            }
        }
    } else {
        quote! {
            actix_web::HttpResponse::Ok()
        }
    };

    let log_assignment = if has_logger {
        quote! {
            ret_body.logs = logger.messages;
        }
    } else {
        quote! {}
    };

    let outer_function = quote! {
        #[actix_web::post(#service_path)]
        #vis async fn #function_ident((req, body): (actix_web::HttpRequest, actix_web::web::Json<azure_functions_types::#trigger_type_ident>)) -> actix_web::Result<actix_web::HttpResponse> {
            #(#definitions;)*
            #user_fn_invocation
            let mut ret_body = azure_functions_types::FuncResponse::default();
            #log_assignment
            Ok(#response
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