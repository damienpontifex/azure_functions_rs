use quote::quote;
use proc_macro::TokenStream;
use syn::parse_macro_input;
use darling::FromMeta;

#[derive(Debug, FromMeta)]
struct TimerTriggerInputs {
    #[darling(default)]
    name: String,
    #[darling(default)]
    schedule: String,
}

#[proc_macro_attribute]
pub fn timer(args: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as syn::ItemFn);
    let function_ident = input.sig.ident.clone();
    let attr_args = parse_macro_input!(args as syn::AttributeArgs);
    let TimerTriggerInputs { name, schedule } = match FromMeta::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return e.write_errors().into();
        }
    };

    //for a in &input.sig.inputs {
    //    println!("{:?}", a);
    //}
    //
    //println!("Target {}, {}", name, schedule);

    let user_fn_ident = quote::format_ident!("user_{}", function_ident);
    let service_fn = quote::format_ident!("{}_service", function_ident);
    let service_path = format!("/{}", name);
    //let actix_route = quote::format_ident!(#[post("/#name")]);

    input.sig.ident = syn::Ident::new(&user_fn_ident.to_string(), proc_macro2::Span::call_site());
    //input.block.stmts.insert(0, syn::parse_quote!(println!("Entered function {}", #function_name);));

    let outer_function = quote! {
        async fn #service_fn((req, bytes): (HttpRequest, web::Bytes)) -> impl Responder {
            println!("Before user call");
            println!("Got body {}", String::from_utf8(bytes.to_vec()).unwrap());
            let mut logger = func_types::Logger::default();
            #user_fn_ident(&mut logger);
            println!("After user call");
            format!("Done")
        }

        fn #function_ident() -> actix_web::Resource {
            println!("Registering {}", #service_path);
            web::resource(#service_path).route(web::post().to(#service_fn))
        }
    };

    let output = quote!{ 
        #outer_function
        #input 
    };
    output.into()
}
