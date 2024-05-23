#![feature(proc_macro_diagnostic)]

use proc_macro::{Diagnostic, Level, Span};
use quote::ToTokens;
use syn::parse_macro_input;

mod spanned;

///
/// ## INTERNAL-ONLY MACRO.
/// ***
/// ## #\[derive(Spanned)]
/// Automatically derives span for supporting:
/// * Structs &mdash; must have a field with the `crate::utils::Span` type.
/// * Enums &mdash; must have variants which are 1-length tuples.
///
/// ### Example
/// ```ignore
/// use avpony_macros::Spanned;
/// use avpony_lang::Span;
///
/// #[derive(Debug, Clone, Spanned)]
/// pub struct Plus(Span);
/// ```
///
#[proc_macro_derive(Spanned)]
pub fn derive_spanned(target: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item: syn::Item = parse_macro_input!(target);
    let res = match item {
        syn::Item::Struct(st) => spanned::struct_impl(st),
        syn::Item::Enum(en) => spanned::enum_impl(en),
        _ => {
            Diagnostic::spanned(
                Span::call_site(),
                Level::Error,
                "You can only use this macro on an enum or struct declaration",
            )
            .emit();
            Err(())
        }
    };

    match res {
        Err(()) => Default::default(),
        Ok(tokens) => tokens.into_token_stream(),
    }
    .into()
}
