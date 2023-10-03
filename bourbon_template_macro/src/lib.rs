extern crate proc_macro;

use bourbon_lexer::tokenize;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::fs;
use syn::{parse_macro_input, AttributeArgs, Data, DeriveInput, Fields, Meta, NestedMeta };

#[proc_macro_attribute]
pub fn template(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let input = parse_macro_input!(input as DeriveInput);

    let mut template_text = None;
    let mut template_file = None;

    for arg in args {
        if let NestedMeta::Meta(Meta::NameValue(nv)) = arg {
            if nv.path.is_ident("text") {
                if let syn::Lit::Str(lit) = nv.lit {
                    template_text = Some(lit.value());
                }
            } else if nv.path.is_ident("file") {
                if let syn::Lit::Str(lit) = nv.lit {
                    template_file = Some(lit.value());
                }
            }
        }
    }

    if let Some(file_path) = template_file {
        template_text = fs::read_to_string(file_path).ok();
    }

    let template_text = template_text.expect("Template text must be provided either inline or in a file");

    let name = &input.ident;

    let mut field_match_arms = Vec::new();
    let mut field_types = Vec::new();
    let mut field_idents = Vec::new();
    let mut field_value_pairs = Vec::new();

    if let Data::Struct(ref data_struct) = input.data {
        if let Fields::Named(fields) = &data_struct.fields {
            for field in &fields.named {
                if let Some(ident) = &field.ident {
                    field_idents.push(ident);
                    let field_name = ident.to_string();

                    // Call to_string_value on each field, regardless of its type
                    field_match_arms.push(quote! {
                        #field_name => {
                            let value = self.#ident.to_string_value();
                            result.extend(value.as_bytes());
                        },
                    });
                    field_types.push(&field.ty);
                    field_value_pairs.push(quote! {
                        (#field_name, self.#ident.to_string_value())
                    });
                }
            }
        }
    }

    let values_type = quote! { (#(#field_types),*) };
    let tokens = tokenize(template_text.as_bytes()).unwrap();

    let token_handling: Vec<TokenStream2> = tokens.iter().map(|token| {
        match token {
            ::bourbon_lexer::Token::FunctionCall(func_name, args) => {
                let args_str = String::from_utf8_lossy(args).into_owned();
                let field_value_tokens = quote! {
                    field_map.get(#args_str).cloned().unwrap_or_else(|| "".to_string())
                };
                let func_name_ident = syn::Ident::new(func_name, proc_macro2::Span::call_site());
                quote! {
                    match funcs::#func_name_ident(#field_value_tokens) {
                        Ok(call_result) => {
                            result.extend(call_result.as_bytes());
                        },
                        Err(e) => return Err(e),
                    }
                }
            }
            ::bourbon_lexer::Token::Placeholder(name) => {
                let name_str = String::from_utf8(name.clone()).unwrap();
                quote! {
                    let name_str = #name_str;
                    match name_str {
                        #(#field_match_arms)*
                        _ => {}
                    }
                }
            }
            ::bourbon_lexer::Token::Text(text) => {
                let text_str = String::from_utf8(text.clone()).unwrap();
                quote! {
                    result.extend(#text_str.as_bytes());
                }
            }
        }
    }).collect();

    let expanded: TokenStream2 = quote! {
        impl ::bourbon::Renderable for #name {
            type Values = #values_type;

            fn new(values: Self::Values) -> Self {
                let (#(#field_idents),*) = values;
                Self { #(#field_idents),* }
            }

            fn render(&self) -> Result<String, String> {
                let field_map: std::collections::HashMap<String, String> = [
                    #(#field_value_pairs),*
                ].into_iter().map(|(k, v)| (k.to_string(), v)).collect();

                let mut result = Vec::new();
                #(#token_handling)*
                Ok(String::from_utf8(result).unwrap())
            }
        }

        #input
    };

    expanded.into()
}
