use proc_macro::{Diagnostic, Level};
use quote::quote;
use syn::spanned::Spanned;

pub fn is_span_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(syn::TypePath {
        path: syn::Path { ref segments, .. },
        ..
    }) = ty
    {
        return segments
            .last()
            .map(|syn::PathSegment { ident, .. }| ident == "Span")
            .unwrap_or(false);
    }

    false
}

pub fn path_to_span() -> proc_macro2::TokenStream {
    quote! {
        crate::utils::Span
    }
}

pub fn path_to_spanned() -> proc_macro2::TokenStream {
    quote! {
        crate::utils::Spanned
    }
}

pub fn path_to_spanned_method() -> proc_macro2::TokenStream {
    let spanned = path_to_spanned();
    quote! {
        #spanned::span
    }
}

pub fn struct_impl(st: syn::ItemStruct) -> Result<proc_macro2::TokenStream, ()> {
    enum Index<'a> {
        Ident(&'a syn::Ident),
        Index(syn::Index),
    }

    impl<'a> quote::ToTokens for Index<'a> {
        fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
            match self {
                Self::Ident(ident) => ident.to_tokens(tokens),
                Self::Index(index) => index.to_tokens(tokens),
            }
        }
    }

    let span_field = st
        .fields
        .iter()
        .enumerate()
        .map(|(index, field)| (index as u32, field))
        .filter_map(
            |(
                index,
                syn::Field {
                    ref ident, ref ty, ..
                },
            )| {
                is_span_type(ty).then_some(ident.as_ref().map(Index::Ident).unwrap_or(
                    Index::Index(syn::Index {
                        index,
                        span: proc_macro2::Span::call_site(),
                    }),
                ))
            },
        )
        .next();

    match span_field {
        Some(field) => {
            let ident = &st.ident;
            let span = path_to_span();
            let spanned = path_to_spanned();
            let span_method = path_to_spanned_method();
            Ok(quote! {
                impl #spanned for #ident {
                    fn span(&self) -> #span {
                        #span_method(&self.#field)
                    }
                }
            })
        }
        None => {
            Diagnostic::spanned(
                st.span().unwrap(),
                Level::Error,
                "There should be one field with the `crate::utils::Span` type.",
            )
            .emit();
            Err(())
        }
    }
}

pub fn enum_impl(en: syn::ItemEnum) -> Result<proc_macro2::TokenStream, ()> {
    let has_invalid_variants = en
        .variants
        .iter()
        .filter(|syn::Variant { fields, .. }| !matches!(fields, syn::Fields::Unnamed(_)))
        .map(|var| {
            Diagnostic::spanned(
                var.span().unwrap(),
                Level::Error,
                "This variant contain only a length-1 tuple.",
            )
            .emit();
        })
        .count()
        > 0;

    if has_invalid_variants {
        return Err(());
    }

    let span = path_to_span();
    let spanned = path_to_spanned();
    let span_method = path_to_spanned_method();

    let arms = en.variants.iter().map(|syn::Variant { ident, .. }| {
        quote! {
            Self::#ident(ref s) => #span_method(s)
        }
    });

    let ident = &en.ident;

    Ok(quote! {
        impl #spanned for #ident {
            fn span(&self) -> #span {
                match self {
                    #(#arms),*,
                }
            }
        }
    })
}
