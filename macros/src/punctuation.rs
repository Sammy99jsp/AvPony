use quote::ToTokens;
use syn::parse::Parse;

use crate::{keyword::traits::*, q, q_attr, spanned::Span, ToType};

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

    #[allow(non_snake_case)]
    let Span = Span::path().to_type();

    st.fields = syn::Fields::Named(q!({
        span: #Span
    }));

    st.attrs
        .extend(q_attr!(#[derive(Debug, Clone, avpony_macros::Spanned, PartialEq)]));

    let impl_parseable = ParseableCloned::impl_for(
        &st,
        std::iter::once(syn::Stmt::Expr(
            Parser::map_with(Parser::just(&args.ch.value().to_string()), |field, expr| {
                q!(Self {
                    #field: #expr,
                })
            }),
            None,
        )),
    );

    let mut tokens = proc_macro2::TokenStream::new();
    st.to_tokens(&mut tokens);
    impl_parseable.to_tokens(&mut tokens);

    tokens.into()
}
