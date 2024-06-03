use quote::ToTokens;
use syn::{parse::Parse, punctuated::Punctuated};

use crate::keyword::macros;

use super::keyword::traits;

#[derive(Debug, Clone)]
pub struct Params {
    pub ch: syn::LitChar,
    pub _at: syn::Token![@],
    pub tag: syn::Ident,
}

impl Parse for Params {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            ch: input.parse()?,
            _at: input.parse()?,
            tag: input.parse()?,
        })
    }
}

pub fn create_punctuation_for(
    args: proc_macro::TokenStream,
    target: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args: Params = syn::parse_macro_input!(args);
    let mut st: syn::ItemStruct = syn::parse_macro_input!(target);

    st.fields = syn::Fields::Named(syn::FieldsNamed {
        brace_token: Default::default(),
        named: Punctuated::from_iter(std::iter::once(syn::Field {
            attrs: Default::default(),
            vis: syn::Visibility::Inherited,
            mutability: syn::FieldMutability::None,
            ident: Some(syn::Ident::new("span", proc_macro2::Span::call_site())),
            colon_token: Default::default(),
            ty: syn::Type::Path(syn::TypePath {
                qself: None,
                path: super::spanned::path_to_span(),
            }),
        })),
    });

    st.attrs.push(macros::derive::for_macros([
        macros::Debug(),
        macros::Clone(),
        macros::Spanned(),
        macros::PartialEq(),
    ]));

    let impl_parseable = traits::ParseableExt::impl_for(
        &st,
        std::iter::once(syn::Stmt::Expr(
            traits::Parser::map_with_span(
                traits::Parser::just(&args.ch.value().to_string()),
                |field, expr| {
                    syn::Expr::Struct(syn::ExprStruct {
                        attrs: Default::default(),
                        qself: None,
                        path: syn::Ident::new("Self", proc_macro2::Span::call_site()).into(),
                        brace_token: Default::default(),
                        fields: syn::punctuated::Punctuated::from_iter(std::iter::once(
                            syn::FieldValue {
                                attrs: Default::default(),
                                member: syn::Member::Named(field),
                                colon_token: Default::default(),
                                expr,
                            },
                        )),
                        dot2_token: None,
                        rest: None,
                    })
                },
            ),
            None,
        )),
    );

    proc_macro::TokenStream::from_iter(
        proc_macro::TokenStream::from(st.into_token_stream())
            .into_iter()
            .chain(proc_macro::TokenStream::from(
                impl_parseable.into_token_stream(),
            )),
    )
}
