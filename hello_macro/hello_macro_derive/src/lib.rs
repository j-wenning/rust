extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
  // take in rust code and parse it into a DeriveInput struct, panic on error
  let ast = syn::parse(input).unwrap();

  impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
  let name = &ast.ident;
  // convert inner code into something usable rather than evaluating
  let gen = quote! {
    // # denotes interpolation
    impl HelloMacro for #name {
      fn hello_macro() {
        // allocate name at compile time
        println!("Hello, Macro!  My name is {}!", stringify!(#name));
      }
    }
  };
  // performs conversion to recipient type if From is implemented for recipient type
  gen.into()
}
