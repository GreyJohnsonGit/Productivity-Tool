mod utility;

extern crate proc_macro;
extern crate syn;
extern crate quote;
use proc_macro::TokenStream;

#[proc_macro_derive(NeverEq)]
pub fn never_eq(item: TokenStream) -> TokenStream {
  let string_representation = item.to_string(); 
  let ast = syn::parse_str(&string_representation).unwrap();
  let generated_code = impl_never_eq(&ast);
  generated_code
}

fn impl_never_eq(ast: &syn::DeriveInput) -> TokenStream {
  let struct_name = &ast.ident;
  
  quote::quote! {
    impl PartialEq for #struct_name<'_> {
      fn eq(&self, _other: &Self) -> bool {
        false
      }
    }
  }.into()
}