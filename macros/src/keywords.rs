use quote::ToTokens;

use crate::{fatal_error, q};

fn get_keyword_structs(kw_mod: &syn::ItemMod) -> impl Iterator<Item = String> + '_ {
    kw_mod
        .content
        .as_ref()
        .map(|(_, a)| a)
        .into_iter()
        .flatten()
        .filter_map(|it| match it {
            syn::Item::Struct(st) => Some(st),
            _ => None,
        })
        .map(|syn::ItemStruct { ident, .. }| ident.to_string().to_lowercase())
}

pub fn make_keywords_module(
    args: proc_macro::TokenStream,
    target: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let ident: syn::Ident = match syn::parse(args) {
        Ok(ident) => ident,
        Err(_) => {
            return fatal_error(
                None,
                "Missing identifier. Syntax expected: `#![keywords(IDENT)]`.",
            )
        }
    };

    let mut kw_mod: syn::ItemMod = match syn::parse(target) {
        Ok(kw_mod) => kw_mod,
        Err(_) => {
            return fatal_error(None, "Only call this macro on an inline module!");
        }
    };

    let keywords = syn::punctuated::Punctuated::<_, syn::token::Comma>::from_iter(
        get_keyword_structs(&kw_mod).map(|keyword| {
            let expr: syn::Expr = q!(#keyword);
            expr
        }),
    );

    kw_mod.content.get_or_insert_default().1.push(q!(
        pub const #ident: &[&str] = &[#keywords];
    ));

    kw_mod.into_token_stream().into()
}
