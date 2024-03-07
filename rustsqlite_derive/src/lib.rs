use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(ToSqlMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_tosql_macro(&ast)
}

fn impl_tosql_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl rusqlite::ToSql for #name {
            #[inline]
            fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
                Ok(rusqlite::types::ToSqlOutput::from(self.to_string()))
            }
        }
    };
    gen.into()
}
