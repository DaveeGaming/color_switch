use proc_macro::TokenStream;
use quote::quote;
use syn::Data;

extern crate proc_macro;

#[proc_macro_derive(TextureDynLoader)]
pub fn derive_texture_dyn_loader(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);

    match &input.data {
        Data::Struct( st ) => {
            let mut struct_inside = quote!{};

            for field in &st.fields {
                let name = field.ident.as_ref().unwrap();
                let path = format!("../assets/{}.png", name);
                struct_inside.extend(quote! {
                    #name: macroquad::texture::load_texture(#path).await.unwrap(),
                })
                
            }

            let struct_name = &input.ident;
            return quote!{
                impl #struct_name {
                    pub async fn new() -> #struct_name {
                        #struct_name {
                            #struct_inside
                        }
                    }
                }
            }.into()

        } 
        _ => quote!{}.into()
    }
}


#[proc_macro_derive(TextureStaticLoader)]
pub fn derive_texture_static_loader(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);

    match &input.data {
        Data::Struct( st ) => {
            let mut struct_inside = quote!{};

            for field in &st.fields {
                let name = field.ident.as_ref().unwrap();
                let path = format!("../assets/{}.png", name);
                struct_inside.extend(quote! {
                    #name: macroquad::texture::Texture2D::from_file_with_format( include_bytes!(#path), None),
                })
                
            }

            let struct_name = &input.ident;
            return quote!{
                impl #struct_name {
                    pub async fn new() -> #struct_name {
                        #struct_name {
                            #struct_inside
                        }
                    }
                }
            }.into()

        } 
        _ => quote!{}.into()
    }
}