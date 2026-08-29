use proc_macro::TokenStream;

mod builtins_table;
mod static_strings;

#[proc_macro]
pub fn builtins_table(input: TokenStream) -> TokenStream {
    builtins_table::builtins_table(input)
}

#[proc_macro]
pub fn static_strings(input: TokenStream) -> TokenStream {
    static_strings::static_strings(input)
}
