use proc_macro::TokenStream;
use quote::quote;
use syn::DataStruct;
use syn::Fields;
use syn::parse_macro_input;

#[proc_macro_derive(StringHolder)]
pub fn string_holder_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);

    // Get the struct name
    let struct_name = &ast.ident;

    // Get the field name (assuming it's a single field named "inner")
    let fields = if let syn::Data::Struct(DataStruct {
        fields: Fields::Named(named),
        ..
    }) = ast.data
    {
        named
    } else {
        panic!("Only structs with named fields are supported");
    };

    // Get the type of the "inner" field
    let inner_field = fields
        .named
        .first()
        .expect("Struct must have at least one field")
        .clone();
    let inner_ident = inner_field.ident.expect("Field must be named");
    let inner_type = inner_field.ty;

    // Generate all the trait implementations
    let expanded = quote! {
        impl #struct_name {
            pub fn new(value: impl Into<#inner_type>) -> Self {
                Self { #inner_ident: value.into() }
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
            type Err = eyre::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                // s.parse().map(|v| Self { #inner_ident: v })
                let rc_str = std::rc::Rc::from(s); // Create an Rc<str> from the input &str
                Ok(Self { #inner_ident: rc_str })
            }
        }

        // PartialEq and Eq implementations
        impl PartialEq for #struct_name {
            fn eq(&self, other: &Self) -> bool {
                self.#inner_ident == other.#inner_ident
            }
        }
        impl Eq for #struct_name {}
        // PartialOrd and Ord implementations
        impl PartialOrd for #struct_name {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                self.#inner_ident.partial_cmp(&other.#inner_ident)
            }
        }
        impl Ord for #struct_name {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.#inner_ident.cmp(&other.#inner_ident)
            }
        }

        // Hash implementation
        impl std::hash::Hash for #struct_name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.#inner_ident.hash(state);
            }
        }

        // Clone implementation
        impl Clone for #struct_name {
            fn clone(&self) -> Self {
                Self {
                    #inner_ident: self.#inner_ident.clone(),
                }
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
