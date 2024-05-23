use proc_macro::{Diagnostic, Level};
use syn::{punctuated::Punctuated, spanned::Spanned};

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

pub fn path_to_span() -> syn::Path {
    syn::Path {
        leading_colon: None,
        segments: Punctuated::from_iter(
            ["crate", "utils", "Span"]
                .into_iter()
                .map(|seg| syn::Ident::new(seg, proc_macro2::Span::call_site()))
                .map(syn::PathSegment::from),
        ),
    }
}

#[allow(clippy::module_inception)] // I'd like this to follow the real path's structure itself.
mod spanned {
    pub fn path() -> syn::Path {
        syn::Path {
            leading_colon: None,
            segments: syn::punctuated::Punctuated::from_iter(
                ["crate", "utils", "Spanned"]
                    .into_iter()
                    .map(|seg| syn::Ident::new(seg, proc_macro2::Span::call_site()))
                    .map(syn::PathSegment::from),
            ),
        }
    }

    pub mod span {
        pub fn path() -> syn::Path {
            syn::Path {
                leading_colon: None,
                segments: syn::punctuated::Punctuated::from_iter(
                    ["crate", "utils", "Spanned", "span"]
                        .into_iter()
                        .map(|seg| syn::Ident::new(seg, proc_macro2::Span::call_site()))
                        .map(syn::PathSegment::from),
                ),
            }
        }

        pub fn call_for(expr: syn::Expr) -> syn::Expr {
            syn::Expr::Call(syn::ExprCall {
                attrs: Default::default(),
                func: Box::new(syn::Expr::Path(syn::ExprPath {
                    attrs: Default::default(),
                    qself: None,
                    path: self::path(),
                })),
                paren_token: Default::default(),
                args: syn::punctuated::Punctuated::from_iter(std::iter::once(expr)),
            })
        }

        pub fn signature() -> syn::Signature {
            syn::Signature {
                constness: None,
                asyncness: None,
                unsafety: None,
                abi: None,
                fn_token: Default::default(),
                ident: syn::Ident::new("span", proc_macro2::Span::call_site()),
                generics: syn::Generics::default(),
                paren_token: Default::default(),
                inputs: syn::punctuated::Punctuated::from_iter(std::iter::once(
                    syn::FnArg::Receiver(syn::Receiver {
                        attrs: Default::default(),
                        reference: Some((Default::default(), None)),
                        mutability: None,
                        self_token: Default::default(),
                        colon_token: None,
                        ty: Box::new(syn::Type::Reference(syn::TypeReference {
                            and_token: Default::default(),
                            lifetime: None,
                            mutability: None,
                            elem: Box::new(syn::Type::Path(syn::TypePath {
                                qself: None,
                                path: syn::Ident::new("Self", proc_macro2::Span::call_site())
                                    .into(),
                            })),
                        })),
                    }),
                )),
                variadic: None,
                output: syn::ReturnType::Type(
                    Default::default(),
                    Box::new(syn::Type::Path(syn::TypePath {
                        qself: None,
                        path: super::super::path_to_span(),
                    })),
                ),
            }
        }
    }

    pub fn impl_for(
        ident: &syn::Ident,
        generics: &syn::Generics,
        expr: syn::Expr,
    ) -> syn::ItemImpl {
        let method = syn::ImplItem::Fn(syn::ImplItemFn {
            attrs: Default::default(),
            vis: syn::Visibility::Inherited,
            defaultness: None,
            sig: self::span::signature(),
            block: syn::Block {
                brace_token: Default::default(),
                stmts: vec![syn::Stmt::Expr(expr, None)],
            },
        });

        let path = if let syn::Generics {
            lt_token: Some(_),
            params,
            gt_token: Some(_),
            ..
        } = generics
        {
            let args =
                syn::punctuated::Punctuated::from_iter(params.iter().cloned().map(|param| {
                    match param {
                        syn::GenericParam::Lifetime(syn::LifetimeParam { lifetime, .. }) => {
                            syn::GenericArgument::Lifetime(lifetime)
                        }
                        syn::GenericParam::Type(syn::TypeParam { ident, .. }) => {
                            syn::GenericArgument::Type(syn::Type::Path(syn::TypePath {
                                qself: None,
                                path: ident.into(),
                            }))
                        }
                        syn::GenericParam::Const(syn::ConstParam { ident, .. }) => {
                            syn::GenericArgument::Const(syn::Expr::Path(syn::ExprPath {
                                attrs: Default::default(),
                                qself: None,
                                path: ident.into(),
                            }))
                        }
                    }
                }));
            syn::Path {
                leading_colon: None,
                segments: syn::punctuated::Punctuated::from_iter(std::iter::once(
                    syn::PathSegment {
                        ident: ident.clone(),
                        arguments: syn::PathArguments::AngleBracketed(
                            syn::AngleBracketedGenericArguments {
                                colon2_token: None,
                                lt_token: Default::default(),
                                args,
                                gt_token: Default::default(),
                            },
                        ),
                    },
                )),
            }
        } else {
            syn::Path::from(ident.clone())
        };

        syn::ItemImpl {
            attrs: Default::default(),
            defaultness: None,
            unsafety: None,
            impl_token: Default::default(),
            generics: generics.clone(),
            trait_: Some((None, self::path(), Default::default())),
            self_ty: Box::new(syn::Type::Path(syn::TypePath { qself: None, path })),
            brace_token: Default::default(),
            items: vec![method],
        }
    }
}

fn self_expr() -> syn::Expr {
    syn::Expr::Path(syn::ExprPath {
        attrs: Default::default(),
        qself: None,
        path: syn::Path::from(syn::Ident::new("self", proc_macro2::Span::call_site())),
    })
}

pub fn struct_impl(st: syn::ItemStruct) -> Result<syn::ItemImpl, ()> {
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
                is_span_type(ty)
                    .then_some(match ident.as_ref() {
                        Some(ident) => syn::Member::Named(ident.clone()),
                        None => syn::Member::Unnamed(syn::Index {
                            span: proc_macro2::Span::call_site(),
                            index,
                        }),
                    })
                    .map(|member| {
                        syn::Expr::Reference(syn::ExprReference {
                            attrs: Default::default(),
                            and_token: Default::default(),
                            mutability: None,
                            expr: Box::new(syn::Expr::Field(syn::ExprField {
                                attrs: Default::default(),
                                base: Box::new(self_expr()),
                                dot_token: Default::default(),
                                member,
                            })),
                        })
                    })
            },
        )
        .next();

    match span_field {
        Some(expr) => Ok(spanned::impl_for(
            &st.ident,
            &st.generics,
            spanned::span::call_for(expr),
        )),
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

pub fn enum_impl(en: syn::ItemEnum) -> Result<syn::ItemImpl, ()> {
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

    const INNER_IDENT: &str = "v";
    let inner_ident = syn::Ident::new(INNER_IDENT, proc_macro2::Span::call_site());

    let arms = en
        .variants
        .iter()
        .map(|syn::Variant { ident, .. }| syn::Arm {
            attrs: Default::default(),
            pat: syn::Pat::TupleStruct(syn::PatTupleStruct {
                attrs: Default::default(),
                qself: None,
                path: syn::Path {
                    leading_colon: None,
                    segments: Punctuated::from_iter(
                        [
                            syn::Ident::new("Self", proc_macro2::Span::call_site()),
                            ident.clone(),
                        ]
                        .map(syn::PathSegment::from),
                    ),
                },
                paren_token: Default::default(),
                elems: Punctuated::from_iter(std::iter::once(syn::Pat::Ident(syn::PatIdent {
                    attrs: Default::default(),
                    by_ref: Some(Default::default()),
                    mutability: None,
                    ident: inner_ident.clone(),
                    subpat: None,
                }))),
            }),
            guard: None,
            fat_arrow_token: Default::default(),
            body: Box::new(spanned::span::call_for(syn::Expr::Path(syn::ExprPath {
                attrs: Default::default(),
                qself: None,
                path: inner_ident.clone().into(),
            }))),
            comma: Some(Default::default()),
        });

    let expr = syn::Expr::Match(syn::ExprMatch {
        attrs: Default::default(),
        match_token: Default::default(),
        expr: Box::new(self_expr()),
        brace_token: Default::default(),
        arms: arms.collect(),
    });

    Ok(spanned::impl_for(&en.ident, &en.generics, expr))
}
