use proc_macro::{Diagnostic, Level};
use quote::ToTokens;
use syn::spanned::Spanned;

use crate::spanned;

fn ident(st: &'static str) -> syn::Ident {
    syn::Ident::new(st, proc_macro2::Span::call_site())
}

pub fn no_generics() -> syn::Generics {
    syn::Generics {
        lt_token: None,
        params: syn::punctuated::Punctuated::new(),
        gt_token: None,
        where_clause: None,
    }
}

fn path(iter: impl IntoIterator<Item = &'static str>) -> syn::Path {
    syn::Path {
        leading_colon: None,
        segments: syn::punctuated::Punctuated::from_iter(
            iter.into_iter().map(ident).map(syn::PathSegment::from),
        ),
    }
}

#[allow(non_snake_case)]
pub mod traits {

    pub mod PonyParser {
        use crate::keyword::path as make_path;

        pub fn path() -> syn::Path {
            make_path(["crate", "utils", "PonyParser"])
        }
    }

    pub mod ParseableExt {
        use crate::keyword::path as make_path;

        pub fn path() -> syn::Path {
            make_path(["crate", "utils", "ParseableExt"])
        }

        pub mod parser_cloneable {
            use crate::keyword::{ident, no_generics};

            fn inputs() -> impl IntoIterator<Item = syn::FnArg> {
                std::iter::empty()
            }

            fn return_type() -> syn::Type {
                syn::Type::ImplTrait(syn::TypeImplTrait {
                    impl_token: Default::default(),
                    bounds: {
                        let mut path = super::super::PonyParser::path();
                        let parser = path.segments.last_mut().unwrap();

                        parser.arguments = syn::PathArguments::AngleBracketed(
                            syn::AngleBracketedGenericArguments {
                                colon2_token: None,
                                lt_token: Default::default(),
                                args: syn::punctuated::Punctuated::from_iter([
                                    syn::GenericArgument::Type(syn::Type::Path(syn::TypePath {
                                        qself: None,
                                        path: ident("Self").into(),
                                    })),
                                ]),
                                gt_token: Default::default(),
                            },
                        );

                        let bound = syn::TypeParamBound::Trait(syn::TraitBound {
                            paren_token: None,
                            modifier: syn::TraitBoundModifier::None,
                            lifetimes: None,
                            path,
                        });
                        syn::punctuated::Punctuated::from_iter([bound, syn::TypeParamBound::Trait(syn::TraitBound {
                            paren_token: None,
                            modifier: syn::TraitBoundModifier::None,
                            lifetimes: None,
                            path: ident("Clone").into(),
                        })])
                    },
                })
            }

            fn sig() -> syn::Signature {
                syn::Signature {
                    constness: None,
                    asyncness: None,
                    unsafety: None,
                    abi: None,
                    fn_token: Default::default(),
                    ident: ident("parser_cloneable"),
                    generics: no_generics(),
                    paren_token: Default::default(),
                    inputs: syn::punctuated::Punctuated::from_iter(self::inputs()),
                    variadic: None,
                    output: syn::ReturnType::Type(
                        Default::default(),
                        Box::new(self::return_type()),
                    ),
                }
            }

            pub fn declare(stmts: impl IntoIterator<Item = syn::Stmt>) -> syn::ImplItem {
                syn::ImplItem::Fn(syn::ImplItemFn {
                    attrs: Default::default(),
                    vis: syn::Visibility::Inherited,
                    defaultness: None,
                    sig: self::sig(),
                    block: syn::Block {
                        brace_token: Default::default(),
                        stmts: stmts.into_iter().collect(),
                    },
                })
            }
        }

        pub fn impl_for(
            st: &syn::ItemStruct,
            stmts: impl IntoIterator<Item = syn::Stmt>,
        ) -> syn::ItemImpl {
            syn::ItemImpl {
                attrs: Default::default(),
                defaultness: None,
                unsafety: None,
                impl_token: Default::default(),
                generics: st.generics.clone(),
                trait_: Some((None, self::path(), Default::default())),
                self_ty: Box::new(syn::Type::Path(syn::TypePath {
                    qself: None,
                    path: st.ident.clone().into(),
                })),
                brace_token: Default::default(),
                items: vec![self::parser_cloneable::declare(stmts)],
            }
        }
    }

    pub mod Parser {
        use crate::keyword::{ident, path as make_path};

        pub fn just(s: &str) -> syn::Expr {
            syn::Expr::Call(syn::ExprCall {
                attrs: Default::default(),
                func: Box::new(syn::Expr::Path(syn::ExprPath {
                    attrs: Default::default(),
                    qself: None,
                    path: make_path(["chumsky", "primitive", "just"]),
                })),
                paren_token: Default::default(),
                args: syn::punctuated::Punctuated::from_iter(std::iter::once(syn::Expr::Lit(
                    syn::ExprLit {
                        attrs: Default::default(),
                        lit: syn::Lit::Str(syn::LitStr::new(s, proc_macro2::Span::call_site())),
                    },
                ))),
            })
        }

        pub fn map_with_span(
            base: syn::Expr,
            then: impl FnOnce(syn::Ident, syn::Expr) -> syn::Expr,
        ) -> syn::Expr {
            let span = ident("span");

            syn::Expr::MethodCall(syn::ExprMethodCall {
                attrs: Default::default(),
                receiver: Box::new(base),
                dot_token: Default::default(),
                method: ident("map_with_span"),
                turbofish: None,
                paren_token: Default::default(),
                args: syn::punctuated::Punctuated::from_iter(std::iter::once(syn::Expr::Closure(
                    syn::ExprClosure {
                        attrs: Default::default(),
                        lifetimes: None,
                        constness: None,
                        movability: None,
                        asyncness: None,
                        capture: None,
                        or1_token: Default::default(),
                        inputs: syn::punctuated::Punctuated::from_iter([
                            syn::Pat::Wild(syn::PatWild {
                                attrs: Default::default(),
                                underscore_token: Default::default(),
                            }),
                            syn::Pat::Ident(syn::PatIdent {
                                attrs: Default::default(),
                                by_ref: None,
                                mutability: None,
                                ident: span.clone(),
                                subpat: None,
                            }),
                        ]),
                        or2_token: Default::default(),
                        output: syn::ReturnType::Default,
                        body: Box::new(then(
                            span.clone(),
                            syn::Expr::Path(syn::ExprPath {
                                attrs: Default::default(),
                                qself: None,
                                path: span.into(),
                            }),
                        )),
                    },
                ))),
            })
        }
    }
}

#[allow(non_snake_case)]
pub mod macros {
    use super::ident;

    pub fn Spanned() -> syn::Path {
        super::path(["avpony_macros", "Spanned"])
    }

    pub fn PartialEq() -> syn::Path {
        ident("PartialEq").into()
    }

    pub fn Clone() -> syn::Path {
        ident("Clone").into()
    }

    pub fn Debug() -> syn::Path {
        ident("Debug").into()
    }

    pub mod derive {
        use quote::ToTokens;

        use crate::keyword::ident;

        fn path() -> syn::Path {
            ident("derive").into()
        }

        pub fn for_macros(iter: impl IntoIterator<Item = syn::Path>) -> syn::Attribute {
            syn::Attribute {
                pound_token: Default::default(),
                style: syn::AttrStyle::Outer,
                bracket_token: Default::default(),
                meta: syn::Meta::List(syn::MetaList {
                    path: self::path(),
                    delimiter: syn::MacroDelimiter::Paren(Default::default()),
                    tokens: syn::punctuated::Punctuated::<_, syn::Token![,]>::from_iter(
                        iter.into_iter().map(|path| {
                            syn::Expr::Path(syn::ExprPath {
                                attrs: Default::default(),
                                qself: None,
                                path,
                            })
                        }),
                    )
                    .into_token_stream(),
                }),
            }
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
            st.attrs.push(macros::derive::for_macros([
                macros::Debug(),
                macros::Clone(),
                macros::Spanned(),
                macros::PartialEq(),
            ]));
            st.fields = syn::Fields::Named(syn::FieldsNamed {
                brace_token: Default::default(),
                named: syn::punctuated::Punctuated::from_iter(std::iter::once(syn::Field {
                    ident: Some(syn::Ident::new("span", proc_macro2::Span::call_site())),
                    vis: syn::Visibility::Inherited,
                    mutability: syn::FieldMutability::None,
                    attrs: Default::default(),
                    colon_token: Default::default(),
                    ty: syn::Type::Path(syn::TypePath {
                        path: spanned::path_to_span(),
                        qself: None,
                    }),
                })),
            });

            let impl_parseable = traits::ParseableExt::impl_for(
                &st,
                std::iter::once(syn::Stmt::Expr(
                    traits::Parser::map_with_span(
                        traits::Parser::just(&st.ident.to_string().to_lowercase()),
                        |field, expr| {
                            syn::Expr::Struct(syn::ExprStruct {
                                attrs: Default::default(),
                                qself: None,
                                path: ident("Self").into(),
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

            return {
                let mut tokens = st.into_token_stream();
                tokens.extend(impl_parseable.into_token_stream());

                tokens.into()
            };
        }
        Ok(st) => st.span(),
        Err(err) => err.span(),
    };

    Diagnostic::spanned(
        span.unwrap(),
        Level::Error,
        "Expected a unit struct declaration here.",
    )
    .emit();

    Default::default()
}
