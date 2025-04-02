
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input,
    DataStruct, FieldsNamed, Ident, Type,
};

#[proc_macro_derive(StringImpls)]
pub fn string_impls_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    
    // Get the struct name
    let struct_name = &ast.ident;
    
    // Get the field name (assuming it's a single field named "inner")
    let fields = if let syn::Data::Struct(DataStruct {
        fields: FieldsNamed { named, .. },
        ..
    }) = ast.data {
        named
    } else {
        panic!("Only structs with named fields are supported");
    };

    // Get the type of the "inner" field
    let inner_field = fields.first()
        .expect("Struct must have at least one field")
        .clone();
    let inner_ident = inner_field.ident.expect("Field must be named");
    let inner_type = inner_field.ty;

    // Generate all the trait implementations
    let expanded = quote! {
        impl #struct_name {
            pub fn new(value: #inner_type) -> Self {
                Self { #inner_ident: value }
            }
        }

        impl AsRef<#inner_type> for #struct_name {
            fn as_ref(&self) -> &#inner_type {
                &self.#inner_ident
            }
        }

        // Implement From trait for String and other types
        impl From<#inner_type> for #struct_name {
            fn from(value: #inner_type) -> Self {
                Self { #inner_ident: value }
            }
        }

        // Implement Into trait
        impl Into<#inner_type> for #struct_name {
            fn into(self) -> #inner_type {
                self.#inner_ident
            }
        }

        // Deref implementation
        impl std::ops::Deref for #struct_name {
            type Target = #inner_type;

            fn deref(&self) -> &Self::Target {
                &self.#inner_ident
            }
        }

        // DerefMut implementation
        impl std::ops::DerefMut for #struct_name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.#inner_ident
            }
        }

        // Display and Debug implementations
        impl std::fmt::Display for #struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.#inner_ident)
            }
        }

        impl std::fmt::Debug for #struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self.#inner_ident)
            }
        }

        // FromStr implementation
        impl std::str::FromStr for #struct_name {
            type Err = <#inner_type as std::str::FromStr>::Err;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                s.parse().map(|v| Self { #inner_ident: v })
            }
        }

        // Serialize and Deserialize implementations
        impl serde::Serialize for #struct_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                self.#inner_ident.serialize(serializer)
            }
        }

        impl<'de> serde::Deserialize<'de> for #struct_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value = <#inner_type as serde::Deserialize>::deserialize(deserializer)?;
                Ok(Self { #inner_ident: value })
            }
        }
    };

    expanded.into()
}