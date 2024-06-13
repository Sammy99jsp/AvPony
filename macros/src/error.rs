use quote::ToTokens;
use syn::punctuated::Punctuated;

use crate::errors;

fn self_type() -> syn::Type {
    syn::Type::Path(syn::TypePath {
        qself: None,
        path: syn::Ident::new("Self", proc_macro2::Span::call_site()).into(),
    })
}

#[allow(non_snake_case)]
mod From {
    use syn::punctuated::Punctuated;

    use crate::spanned::genericize_path;

    pub fn path(ty: syn::Type) -> syn::Path {
        syn::Path {
            leading_colon: None,
            segments: Punctuated::from_iter(std::iter::once(syn::PathSegment {
                ident: syn::Ident::new("From", proc_macro2::Span::call_site()),
                arguments: syn::PathArguments::AngleBracketed(
                    syn::AngleBracketedGenericArguments {
                        colon2_token: None,
                        lt_token: Default::default(),
                        args: Punctuated::from_iter(std::iter::once(syn::GenericArgument::Type(
                            ty,
                        ))),
                        gt_token: Default::default(),
                    },
                ),
            })),
        }
    }

    mod from {
        use syn::punctuated::Punctuated;

        use crate::error::self_type;

        fn signature(value_ty: syn::Type) -> syn::Signature {
            syn::Signature {
                constness: None,
                asyncness: None,
                unsafety: None,
                abi: None,
                fn_token: Default::default(),
                ident: syn::Ident::new("from", proc_macro2::Span::call_site()),
                generics: syn::Generics {
                    lt_token: None,
                    params: Punctuated::new(),
                    gt_token: None,
                    where_clause: None,
                },
                paren_token: Default::default(),
                inputs: Punctuated::from_iter(std::iter::once(syn::FnArg::Typed(syn::PatType {
                    attrs: Default::default(),
                    pat: Box::new(syn::Pat::Ident(syn::PatIdent {
                        attrs: Default::default(),
                        by_ref: None,
                        mutability: None,
                        ident: syn::Ident::new("value", proc_macro2::Span::call_site()),
                        subpat: None,
                    })),
                    colon_token: Default::default(),
                    ty: Box::new(value_ty),
                }))),
                variadic: None,
                output: syn::ReturnType::Type(Default::default(), Box::new(self_type())),
            }
        }

        fn body(ident: &syn::Ident) -> syn::Block {
            let variant = syn::Expr::Path(syn::ExprPath {
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
            });

            let value = syn::Expr::Path(syn::ExprPath {
                attrs: Default::default(),
                qself: None,
                path: syn::Ident::new("value", proc_macro2::Span::call_site()).into(),
            });

            let var_expr = syn::Expr::Call(syn::ExprCall {
                attrs: Default::default(),
                func: Box::new(variant),
                paren_token: Default::default(),
                args: Punctuated::from_iter(std::iter::once(value)),
            });

            let stmt = syn::Stmt::Expr(var_expr, None);

            syn::Block {
                brace_token: Default::default(),
                stmts: vec![stmt],
            }
        }

        pub fn impl_for(ident: &syn::Ident, value_ty: syn::Type) -> syn::ImplItemFn {
            syn::ImplItemFn {
                attrs: Default::default(),
                vis: syn::Visibility::Inherited,
                defaultness: None,
                sig: self::signature(value_ty),
                block: self::body(ident),
            }
        }
    }

    pub fn impl_for(
        parent: syn::Path,
        ident: syn::Ident,
        generics: &syn::Generics,
    ) -> syn::ItemImpl {
        let path: syn::Path = ident.clone().into();
        let ty = syn::Type::Path(syn::TypePath {
            qself: None,
            path: genericize_path(&path, generics),
        });

        let func = syn::ImplItem::Fn(from::impl_for(&ident, ty.clone()));

        syn::ItemImpl {
            attrs: Default::default(),
            defaultness: None,
            unsafety: None,
            impl_token: Default::default(),
            generics: generics.clone(),
            trait_: Some((None, self::path(ty), Default::default())),
            self_ty: Box::new(syn::Type::Path(syn::TypePath {
                qself: None,
                path: parent,
            })),
            brace_token: Default::default(),
            items: vec![func],
        }
    }
}

pub fn impl_struct(parent: syn::Path, mut st: syn::ItemStruct) -> Vec<syn::Item> {
    st.attrs.push(syn::Attribute {
        pound_token: Default::default(),
        style: syn::AttrStyle::Outer,
        bracket_token: Default::default(),
        meta: syn::Meta::List(syn::MetaList {
            path: errors::path(["derive"]),
            delimiter: syn::MacroDelimiter::Paren(Default::default()),
            tokens: Punctuated::<syn::Path, syn::Token![,]>::from_iter([
                errors::path(["Debug"]),
                errors::path(["Clone"]),
                errors::path(["PartialEq"]),
                errors::path(["avpony_macros", "Spanned"]),
            ])
            .into_token_stream(),
        }),
    });

    vec![
        syn::Item::Struct(st.clone()),
        syn::Item::Impl(From::impl_for(parent, st.ident.clone(), &st.generics)),
    ]
}
