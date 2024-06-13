//!
//! Build script for HTML entities.
//!
//! Gets definitions from `src/syntax/ponyx/entities/html_ref.json`.
//!

#![feature(closure_lifetime_binder)]
use std::{env, fs, path::Path};

macro_rules! q {
    ($($t: tt)*) => {{
        let tokens = quote::quote! { $($t)* };
        syn::parse2(tokens.clone())
            .expect(&format!("Error on {}:{}\n\nTokens: {tokens}", file!(), line!()))
    }}
}

use proc_macro2::Span;
use quote::ToTokens;

use std::collections::HashMap;

use serde::Deserialize;
use syn::punctuated::Punctuated;

#[derive(Debug, Clone, Deserialize)]
pub struct HtmlEntity {
    #[allow(dead_code)]
    codepoints: Vec<u32>,
    characters: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EntityId(String);

impl<'de> serde::Deserialize<'de> for EntityId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EntityId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "Expected a string literal here.")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                // I know this is inefficient, and a slice is better.
                // TODO: replace this with a slice.
                Ok(EntityId(v[1..v.len() - 1].to_string()))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

pub fn register_entities(span: Span, contents: String) -> impl Iterator<Item = syn::Item> {
    let map: HashMap<EntityId, HtmlEntity> =
        match serde_json::from_str(&contents).map_err(Box::<dyn std::error::Error>::from) {
            Ok(map) => map,
            Err(err) => {
                panic!("Invalid JSON at `src/ponyx/entity/html_ref.json`!\n{err:?}")
            }
        };

    let mut map = map.into_iter().collect::<Vec<_>>();
    map.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    use syn::token::Comma;
    let (codes, values): (Punctuated<_, Comma>, Punctuated<_, Comma>) = map
        .into_iter()
        .map(|(code, value)| {
            [
                syn::LitStr::new(&code.0, span),
                syn::LitStr::new(&value.characters, span),
            ]
        })
        .map(|arr| arr.map(syn::Lit::Str))
        .map(|arr| {
            arr.map(|lit| syn::ExprLit {
                attrs: Default::default(),
                lit,
            })
        })
        .map(|arr| arr.map(syn::Expr::from))
        .map(|[code, value]| (code, value))
        .unzip();

    let names = ["CODES", "VALUES"];

    // Generate code and value arrays.

    [codes, values]
        .map(|elems| {
            let expr: syn::Expr = q!(&[#elems]);
            expr
        })
        .into_iter()
        .zip(names)
        .map(|(expr, name)| {
            let ident = syn::Ident::new(name, Span::call_site());
            q!(
                pub const #ident: &[&str] = #expr;
            )
        })
        .map(syn::Item::Const)
}

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("html_entities.rs");
    let contents = fs::read_to_string("src/ponyx/entity/html_ref.json").unwrap();

    let f = syn::File {
        shebang: None,
        attrs: Default::default(),
        items: register_entities(Span::call_site(), contents).collect(),
    };

    fs::write(&dest_path, f.to_token_stream().to_string()).unwrap();
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=src/ponyx/entity/html_ref.json");
}
