use proc_macro::TokenStream;

mod builtins_table;

#[proc_macro]
pub fn builtins_table(input: TokenStream) -> TokenStream {
    builtins_table::builtins_table(input)
}
