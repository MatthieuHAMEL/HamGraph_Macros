use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};
use serde_json;

#[proc_macro]
pub fn generate_sprites(input: TokenStream) -> TokenStream {
  // Parse the JSON file path from the input
  let input = parse_macro_input!(input as LitStr);
  let json_path = input.value();

  // Read and parse the JSON file
  let json_data = std::fs::read_to_string(json_path)
    .expect("Failed to read JSON file");
  let sprite_data: Vec<serde_json::Value> = serde_json::from_str(&json_data)
    .expect("Invalid JSON format");

  // Extract sprite names
  let mut sprite_names = Vec::new();
  for sprite_sheet in sprite_data 
  {
    if let Some(sprites) = sprite_sheet.get("sprites").and_then(|s| s.as_array()) 
    {
      for sprite in sprites 
      {
        if let Some(name) = sprite.get("name").and_then(|n| n.as_str()) 
        {
          sprite_names.push(name.to_string());
        }
      }
    }
  }

  // Generate the enum
  let enum_variants = sprite_names
    .iter()
    .map(|name| syn::Ident::new(name, proc_macro2::Span::call_site()));

  let output = quote! {
    pub enum UserSprite {
      #(#enum_variants),*
      }
  };

  TokenStream::from(output)
}