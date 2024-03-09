use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, ItemFn};

#[derive(Debug)]
enum Time {
    Second,
    Mili,
    Micro,
    Nano,
}

impl Parse for Time {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = if let Ok(path) = input.parse::<syn::Path>() {  //#[timed(Time:Mili)]
            path.segments.last().unwrap().ident.to_string()
        } else if let Ok(lit) = input.parse::<syn::LitStr>() {      //#[timed("Mili")]
            lit.value()
        } else {
            return Err(syn::Error::new(input.span(), "Expected identifier or string literal"));
        };

        match ident.as_str() {
            "Second" => Ok(Time::Second),
            "Mili" => Ok(Time::Mili),
            "Micro" => Ok(Time::Micro),
            "Nano" => Ok(Time::Nano),
            _ => Err(syn::Error::new(input.span(), "Invalid time value")),
        }
    }
}

#[proc_macro_attribute]
pub fn timed(args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let mut function = parse_macro_input!(input as ItemFn);

    // Parse Attribute arguments
    let args = parse_macro_input!(args as Time);

    let (unit_func, unit_str) = match args {
        Time::Second => (quote! { as_secs }, "seconds"),
        Time::Mili => (quote! { as_millis }, "ms"),
        Time::Micro => (quote! { as_micros}, "microseconds"),
        Time::Nano => (quote! { as_nanos}, "nanoseconds"),
    };

    // Modify the function
    let code_block = &function.block;
    function.block = syn::parse_quote!({
        use pretty_assertions::assert_eq;
        let start = std::time::Instant::now();
        let result = #code_block;
        let elapsed = start.elapsed();
        print!("Time elapsed: {:.2} {}", elapsed.as_secs_f32() * 1_000_000.0, "µs (microseconds) | ",);
        println!("{} {:?}", elapsed.#unit_func(), #unit_str);
        result
    });

    TokenStream::from(quote! { #function })
}
