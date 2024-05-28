use std::collections::HashMap;

use crate::keyword::no_generics;

pub fn collect_punctuation_structs<'a, 'iter: 'a, I: ToString + 'static>(
    structs: impl Iterator<Item = &'a syn::ItemStruct>,
    buckets: impl Iterator<Item = &'a I>,
) -> impl Iterator<Item = syn::ItemConst> {
    let mut hash_map: HashMap<String, String> = HashMap::new();
    structs
        .filter_map(|st| {
            st.attrs.iter().find_map(|attr| {
                attr.path()
                    .segments
                    .last()
                    .and_then(|seg| (seg.ident == "Punctuation").then_some(&attr.meta))
            })
        })
        .filter_map(|meta| match meta {
            syn::Meta::List(a) => Some(&a.tokens),
            _ => None,
        })
        .filter_map(|args| syn::parse2::<crate::punctuation::Params>(args.clone()).ok())
        .for_each(|params| {
            hash_map
                .entry(params.tag.to_string())
                .or_default()
                .push(params.ch.value());
        });

    buckets.map(I::to_string).filter_map(move |bucket| {
        hash_map.get(&bucket).map(|chars| syn::ItemConst {
            attrs: Default::default(),
            vis: syn::Visibility::Public(Default::default()),
            const_token: Default::default(),
            ident: syn::Ident::new(&bucket.to_uppercase(), proc_macro2::Span::call_site()),
            generics: no_generics(),
            colon_token: Default::default(),
            ty: Box::new(syn::Type::Reference(syn::TypeReference {
                and_token: Default::default(),
                lifetime: None,
                mutability: None,
                elem: Box::new(syn::Type::Path(syn::TypePath {
                    qself: None,
                    path: syn::Ident::new("str", proc_macro2::Span::call_site()).into(),
                })),
            })),
            eq_token: Default::default(),
            expr: Box::new(syn::Expr::Lit(syn::ExprLit {
                attrs: Default::default(),
                lit: syn::Lit::Str(syn::LitStr::new(chars, proc_macro2::Span::call_site())),
            })),
            semi_token: Default::default(),
        })
    })
}
