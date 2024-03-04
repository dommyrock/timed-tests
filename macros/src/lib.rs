use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, spanned::Spanned, ItemFn};

#[derive(Debug)]
enum Time { 
    Second,
    Mili,
    Nano
}

impl Parse for Time{
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let path: syn::Path = input.parse()?;
        let ident = path.segments.last().unwrap().ident.to_string();
        match ident.as_str() {
            "Second" => Ok(Time::Second),
            "Mili" => Ok(Time::Mili),
            "Nano" => Ok(Time::Nano),
            _ => Err(syn::Error::new(path.span(), "Invalid time value")),
        }
    }
}
//v2 parse string inputs 
//ie. #[timed("Second")]]
// impl Parse for Time {
//     fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
//         let ident: syn::LitStr = input.parse()?;
//         match ident.value().as_str() {
//             "Second" => Ok(Time::Second),
//             "Mili" => Ok(Time::Mili),
//             "Nano" => Ok(Time::Nano),
//             _ => Err(syn::Error::new(ident.span(), "Invalid time value maaan")),
//         }
//     }
// }


#[proc_macro_attribute]
pub fn timed(args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let mut function = parse_macro_input!(input as ItemFn);

    // Parse the attribute arguments
    let args = parse_macro_input!(args as Time);

    let unit = match args {
        Time::Second => { quote! { as_secs }},
        Time::Mili => { quote! { as_millis }},
        Time::Nano => { quote! { as_nanos }},
    };

    // Modify the function
    let code_block = &function.block;
    function.block = syn::parse_quote!({
        let start = std::time::Instant::now();
        let result = #code_block;
        let elapsed = start.elapsed();
        println!("Time elapsed: {} {:?}", elapsed.#unit(), stringify!(#unit));
        result
    });

    // Return the modified function
    TokenStream::from(quote! { #function })
}
