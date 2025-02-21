use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::{Error, Item, Result, parse2, spanned::Spanned};

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

/// Automatically derives `Spanned` for the annotated type. This will work as long as there is
/// some struct field of type `Span`.
#[proc_macro_derive(Spanned)]
pub fn derive_spanned(tokens: TokenStream) -> TokenStream {
    match derive_spanned_internal(tokens.into()) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error(),
    }
    .into()
}

fn derive_spanned_internal(tokens: TokenStream2) -> Result<TokenStream2> {
    let item = parse2::<Item>(tokens)?;
    let (field_name, ident, generics) = match item {
        // Item::Enum(item_enum) => (item_enum.ident, item_enum.generics),
        Item::Struct(item_struct) => {
            let mut i: usize = 0;
            let field_name = item_struct
                .fields
                .iter()
                .find_map(|field| {
                    i += 1;
                    if field
                        .ty
                        .to_token_stream()
                        .to_string()
                        .trim()
                        .ends_with("Span")
                        || field.ident.to_token_stream().to_string().trim() == "span"
                    {
                        if let Some(ident) = field.ident.as_ref() {
                            Some(quote!(self.#ident))
                        } else {
                            let lit = syn::Index::from(i - 1);
                            Some(quote!(self.#lit))
                        }
                    } else {
                        None
                    }
                })
                .ok_or_else(|| {
                    Error::new(item_struct.span(), "expected a field of type `quoth::Span`")
                })?
                .clone();
            (field_name, item_struct.ident, item_struct.generics)
        }
        _ => return Err(Error::new(item.span(), "expected struct")),
    };
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let tokens = quote! {
        impl #impl_generics quoth::Spanned for #ident #ty_generics #where_clause {
            fn span(&self) -> quoth::Span {
                #field_name.clone()
            }
        }
    };
    Ok(tokens)
}
