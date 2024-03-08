use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse2, spanned::Spanned, Error, Item, Result};

/// Derives [`Display`](core::fmt::Display) and [`FromStr`](core::str::FromStr) based on the
/// the `parse()` and `unparse()` implementations of `Parsable` for this type, respectively.
#[proc_macro_derive(ParsableExt)]
pub fn derive_parsable_ext(tokens: TokenStream) -> TokenStream {
    match derive_parsable_ext_internal(tokens.into()) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error(),
    }
    .into()
}

fn derive_parsable_ext_internal(tokens: TokenStream2) -> Result<TokenStream2> {
    let item = parse2::<Item>(tokens)?;
    let (ident, generics) = match item {
        Item::Enum(item_enum) => (item_enum.ident, item_enum.generics),
        Item::Struct(item_struct) => (item_struct.ident, item_struct.generics),
        _ => return Err(Error::new(item.span(), "expected struct or enum")),
    };
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let tokens = quote! {
        impl #impl_generics core::str::FromStr for #ident #ty_generics #where_clause {
            type Err = quoth::Error;

            fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
                quoth::parse(s)
            }
        }

        impl #impl_generics core::fmt::Display for #ident #ty_generics #where_clause {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.unparse(f)
            }
        }
    };
    Ok(tokens)
}
