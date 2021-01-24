use quote::quote;
use proc_macro::TokenStream;
use syn::parse_macro_input;
use darling::FromMeta;

#[derive(Default)]
pub struct Logger {
    messages: Vec<String>,
}

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
    let function_name = function_ident.to_string();
    let attr_args = parse_macro_input!(args as syn::AttributeArgs);
    let TimerTriggerInputs { name, schedule } = match FromMeta::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return e.write_errors().into();
        }
    };
    
    println!("Target {}, {}", name, schedule);

    let user_fn_ident = quote::format_ident!("user_{}", function_ident);

    input.sig.ident = syn::Ident::new(&user_fn_ident.to_string(), proc_macro2::Span::call_site());
    //input.block.stmts.insert(0, syn::parse_quote!(println!("Entered function {}", #function_name);));

    let outer_function = quote! {
        fn #function_ident() {
            println!("Before user call");
            #user_fn_ident();
            println!("After user call");
        }
    };

    let output = quote!{ 
        #outer_function
        #input 
    };
    output.into()
}
