use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Attribute};

#[proc_macro_attribute]
pub fn todo_app(_attr: TokenStream, item: TokenStream) -> TokenStream {
  let input = parse_macro_input!(item as DeriveInput);
  let struct_name = &input.ident;

  let fields = match &input.data {
    Data::Struct(data_struct) => {
      if let Fields::Named(fields_named) = &data_struct.fields {
        &fields_named.named
      } else {
        panic!("todo_app only works on named structs");
      }
    }
    _ => panic!("todo_app only works on structs"),
  };

  let mut new_fields = vec![];

  for field in fields {
    let field_name = field.ident.as_ref().unwrap().to_string();
    let pascal_case = to_pascal_case(&field_name);
    let renamed = format!("TodoApp{}", pascal_case);

    let _ty = &field.ty;
    let _ident = &field.ident;

    let renamed_attr: Attribute = syn::parse_quote! {
      #[serde(rename = #renamed)]
    };

    let mut new_field = field.clone();
    new_field.attrs.push(renamed_attr);

    new_fields.push(quote! { #new_field });
  }

  let expanded = quote! {
    #[derive(Debug, serde::Deserialize, serde::Serialize)]
    struct #struct_name {
      #(#new_fields),*
    }
  };

  TokenStream::from(expanded)
}

fn to_pascal_case(s: &str) -> String {
  s.split('_')
    .map(|w| {
      let mut chars = w.chars();
      match chars.next() {
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
        None => "".to_string(),
      }
    })
    .collect()
}