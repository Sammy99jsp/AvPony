use std::collections::HashMap;

use quote::ToTokens;

use crate::{ident, q};

fn collect_punctuation_structs<'a, 'iter: 'a, I: ToString + 'static>(
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
        hash_map.get(&bucket).map(|chars| {
            let bucket = ident(&bucket.to_uppercase());
            q!(
                pub const #bucket: &str = #chars;
            )
        })
    })
}

pub fn make_module(
    list: proc_macro::TokenStream,
    target: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let buckets: syn::punctuated::Punctuated<syn::Ident, syn::Token![,]> =
        syn::parse_macro_input!(list with syn::punctuated::Punctuated::parse_separated_nonempty);
    let mut module: syn::ItemMod = syn::parse_macro_input!(target);

    let consts: Vec<_> = collect_punctuation_structs(
        module
            .content
            .as_ref()
            .map(|(_, items)| {
                items.iter().filter_map(|item| match item {
                    syn::Item::Struct(s) => Some(s),
                    _ => None,
                })
            })
            .into_iter()
            .flatten(),
        buckets.iter(),
    )
    .collect();

    if let Some((_, items)) = module.content.as_mut() {
        items.extend(consts.into_iter().map(syn::Item::Const))
    }

    module.into_token_stream().into()
}
