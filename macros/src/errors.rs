use proc_macro::{Diagnostic, Level};
use proc_macro2::Span;
use quote::ToTokens;
use syn::punctuated::Punctuated;

fn ident(a: &'static str) -> syn::Ident {
    syn::Ident::new(a, Span::call_site())
}

pub fn path(iter: impl IntoIterator<Item = &'static str>) -> syn::Path {
    syn::Path {
        leading_colon: None,
        segments: Punctuated::from_iter(iter.into_iter().map(ident).map(syn::PathSegment::from)),
    }
}

#[allow(non_snake_case)]
mod ErrorI {
    use syn::punctuated::Punctuated;

    use crate::keyword::no_generics;

    use super::ident;

    pub fn path() -> syn::Path {
        syn::Path {
            leading_colon: None,
            segments: Punctuated::from_iter(
                ["crate", "utils", "ErrorI"]
                    .map(ident)
                    .map(syn::PathSegment::from),
            ),
        }
    }

    pub mod to_report {
        use proc_macro2::Span;
        use syn::punctuated::Punctuated;

        use crate::{errors::ident, keyword::no_generics, spanned::path_to_span};

        pub fn call(expr: syn::Expr) -> syn::Expr {
            let path = syn::Expr::Path(syn::ExprPath {
                attrs: Default::default(),
                path: syn::Path {
                    leading_colon: None,
                    segments: Punctuated::from_iter(
                        ["crate", "utils", "ErrorI", "to_report"]
                            .map(ident)
                            .map(syn::PathSegment::from),
                    ),
                },
                qself: None,
            });

            syn::Expr::Call(syn::ExprCall {
                attrs: Default::default(),
                func: Box::new(path),
                paren_token: Default::default(),
                args: Punctuated::from_iter(std::iter::once(expr)),
            })
        }

        fn signature() -> syn::Signature {
            syn::Signature {
                constness: None,
                asyncness: None,
                unsafety: None,
                abi: None,
                fn_token: Default::default(),
                ident: ident("to_report"),
                generics: no_generics(),
                paren_token: Default::default(),
                inputs: Punctuated::from_iter(std::iter::once(syn::FnArg::Receiver(
                    syn::Receiver {
                        attrs: Default::default(),
                        reference: None,
                        mutability: None,
                        self_token: Default::default(),
                        colon_token: None,
                        ty: Box::new(syn::Type::Path(syn::TypePath {
                            qself: None,
                            path: ident("Self").into(),
                        })),
                    },
                ))),
                variadic: None,
                output: syn::ReturnType::Type(
                    Default::default(),
                    Box::new(syn::Type::Path(syn::TypePath {
                        qself: None,
                        path: syn::Path {
                            leading_colon: None,
                            segments: Punctuated::from_iter([
                                ident("ariadne").into(),
                                syn::PathSegment {
                                    ident: ident("Report"),
                                    arguments: syn::PathArguments::AngleBracketed(
                                        syn::AngleBracketedGenericArguments {
                                            colon2_token: None,
                                            lt_token: Default::default(),
                                            args: Punctuated::from_iter([
                                                syn::GenericArgument::Lifetime(syn::Lifetime::new(
                                                    "'static",
                                                    Span::call_site(),
                                                )),
                                                syn::GenericArgument::Type(syn::Type::Path(
                                                    syn::TypePath {
                                                        qself: None,
                                                        path: path_to_span(),
                                                    },
                                                )),
                                            ]),
                                            gt_token: Default::default(),
                                        },
                                    ),
                                },
                            ]),
                        },
                    })),
                ),
            }
        }

        fn body_for(iter: impl IntoIterator<Item = syn::Ident>) -> syn::Block {
            let iter = iter.into_iter();
            let variant = |v_ident: syn::Ident| {
                (
                    syn::Pat::TupleStruct(syn::PatTupleStruct {
                        attrs: Default::default(),
                        qself: None,
                        path: syn::Path {
                            leading_colon: None,
                            segments: Punctuated::from_iter(
                                [ident("Self"), v_ident].map(syn::PathSegment::from),
                            ),
                        },
                        paren_token: Default::default(),
                        elems: Punctuated::from_iter(std::iter::once(syn::Pat::Ident(
                            syn::PatIdent {
                                attrs: Default::default(),
                                by_ref: None,
                                mutability: None,
                                ident: ident("v"),
                                subpat: None,
                            },
                        ))),
                    }),
                    syn::Expr::Path(syn::ExprPath {
                        attrs: Default::default(),
                        qself: None,
                        path: ident("v").into(),
                    }),
                )
            };

            let expr = syn::Expr::Match(syn::ExprMatch {
                attrs: Default::default(),
                match_token: Default::default(),
                expr: Box::new(syn::Expr::Path(syn::ExprPath {
                    attrs: Default::default(),
                    qself: None,
                    path: ident("self").into(),
                })),
                brace_token: Default::default(),
                arms: iter
                    .map(variant)
                    .map(|(pat, v)| syn::Arm {
                        attrs: Default::default(),
                        pat,
                        guard: None,
                        fat_arrow_token: Default::default(),
                        body: Box::new(self::call(v)),
                        comma: Some(Default::default()),
                    })
                    .collect(),
            });

            let stmt = syn::Stmt::Expr(expr, None);

            syn::Block {
                brace_token: Default::default(),
                stmts: vec![stmt],
            }
        }

        pub fn impl_for(iter: impl IntoIterator<Item = syn::Ident>) -> syn::ImplItemFn {
            syn::ImplItemFn {
                attrs: Default::default(),
                vis: syn::Visibility::Inherited,
                defaultness: None,
                sig: self::signature(),
                block: self::body_for(iter),
            }
        }
    }

    pub fn impl_for(en: &syn::ItemEnum) -> syn::ItemImpl {
        let vars = en
            .variants
            .iter()
            .map(|syn::Variant { ref ident, .. }| ident)
            .cloned();

        syn::ItemImpl {
            attrs: Default::default(),
            defaultness: None,
            unsafety: None,
            impl_token: Default::default(),
            generics: no_generics(),
            trait_: Some((None, self::path(), Default::default())),
            self_ty: Box::new(syn::Type::Path(syn::TypePath {
                qself: None,
                path: en.ident.clone().into(),
            })),
            brace_token: Default::default(),
            items: vec![syn::ImplItem::Fn(self::to_report::impl_for(vars))],
        }
    }
}

pub fn generate_error_enum(tokens: proc_macro::TokenStream) -> Vec<syn::Item> {
    let Ok(mut en) = syn::parse::<syn::ItemEnum>(tokens) else {
        Diagnostic::spanned(
            Span::call_site().unwrap(),
            Level::Error,
            "Expected an enum declaration here",
        )
        .emit();

        return vec![];
    };

    en.attrs.push(syn::Attribute {
        pound_token: Default::default(),
        style: syn::AttrStyle::Outer,
        bracket_token: Default::default(),
        meta: syn::Meta::List(syn::MetaList {
            path: path(["derive"]),
            delimiter: syn::MacroDelimiter::Paren(Default::default()),
            tokens: Punctuated::<syn::Path, syn::token::Comma>::from_iter([
                path(["Debug"]),
                path(["Clone"]),
                path(["PartialEq"]),
                path(["avpony_macros", "Spanned"]),
            ])
            .into_token_stream(),
        }),
    });

    let impl_errori = ErrorI::impl_for(&en);

    vec![syn::Item::Enum(en), syn::Item::Impl(impl_errori)]
}
