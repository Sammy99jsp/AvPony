use quote::ToTokens;
use syn::spanned::Spanned;

use crate::{fatal_error, q, q_attr, spanned::Span, ToType};

#[allow(non_snake_case)]
pub mod traits {

    pub mod PonyParser {
        use crate::q;

        pub fn path() -> syn::Path {
            q!(crate::utils::PonyParser)
        }
    }

    pub mod ParseableCloned {
        use crate::{keyword::traits::PonyParser, q};

        pub fn path() -> syn::Path {
            q!(crate::utils::ParseableCloned)
        }

        pub fn impl_for(
            syn::ItemStruct { ident, .. }: &syn::ItemStruct,
            stmts: impl IntoIterator<Item = syn::Stmt>,
        ) -> syn::ItemImpl {
            let ParseableCloned = self::path();
            let PonyParser = PonyParser::path();
            let stmts = stmts.into_iter();

            q!(
                impl #ParseableCloned for #ident {
                    fn parser<'src>() -> impl #PonyParser<'src, Self> + Clone {
                        #(#stmts)*
                    }
                }
            )
        }
    }

    pub mod Parser {
        use crate::q;

        pub fn just(s: &str) -> syn::Expr {
            q!(chumsky::primitive::just(#s))
        }

        pub fn map_with(
            base: syn::Expr,
            then: impl FnOnce(syn::Ident, syn::Expr) -> syn::Expr,
        ) -> syn::Expr {
            let body = then(q!(span), q!(ctx.span()));

            q!(
                #base
                    .map_with(|_, ctx| #body)
            )
        }
    }
}

pub fn make_keyword(
    target: proc_macro::TokenStream,
    _: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let span = match syn::parse(target) {
        Ok(
            mut st @ syn::ItemStruct {
                fields: syn::Fields::Unit,
                ..
            },
        ) => {
            st.attrs
                .extend(q_attr!(#[derive(Debug, Clone, avpony_macros::Spanned, PartialEq)]));

            #[allow(non_snake_case)]
            let Span = Span::path().to_type();
            st.fields = syn::Fields::Named(q!({
                span: #Span
            }));

            use traits::Parser::*;

            let impl_parseable = traits::ParseableCloned::impl_for(
                &st,
                [syn::Stmt::Expr(
                    map_with(just(&st.ident.to_string().to_lowercase()), |field, expr| {
                        q!(Self {
                            #field: #expr
                        })
                    }),
                    None,
                )],
            );

            return {
                let mut tokens = proc_macro2::TokenStream::new();
                st.to_tokens(&mut tokens);
                impl_parseable.to_tokens(&mut tokens);

                tokens.into()
            };
        }
        Ok(st) => st.span(),
        Err(err) => err.span(),
    };

    fatal_error(span, "Expected a unit struct declaration here.")
}
