#![feature(proc_macro_diagnostic, option_get_or_insert_default)]

use errors::generate_error_enum;
use quote::ToTokens;
use syn::{parse_macro_input, spanned::Spanned};

mod error;
mod errors;
mod keyword;
mod keywords;
mod punctuation;
mod punctuations;
mod spanned;
mod utils;

pub(crate) use utils::*;

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

        #[allow(clippy::unit_arg)]
        _ => Err(fatal_error(
            None,
            "You can only use this macro on an enum or struct declaration",
        )),
    };

    match res {
        Err(()) => Default::default(),
        Ok(tokens) => tokens.into_token_stream(),
    }
    .into()
}

///
/// ## INTERNAL-ONLY MACRO.
/// ***
///
/// ## #\[Keyword]
/// Makes a new keyword struct + parser.
///
/// ### Example
/// ```ignore
/// use avpony_macros::Keyword;
///
/// ///
/// /// # `if`
/// /// Used to denote `if` blocks (conditional blocks).
/// /// ```avpony
/// /// {#if fruit == "Apple"}
/// ///     You have chosen: Apple.
/// /// {/if}
/// /// ```
/// ///
/// #[Keyword]
/// pub struct If;
/// ```
#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Keyword(
    args: proc_macro::TokenStream,
    target: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    keyword::make_keyword(target, args)
}

///
/// ## INTERNAL-ONLY MACRO.
/// ***
///
/// ## #\[Keywords]
/// Collects all declared keywords in a module
/// into a const &[&str].
///
/// ### Example
/// ```ignore
/// use avpony_macros::Keywords;
///
///                         // All declared keywords will be put in `keywords::KEYWORDS`.
/// #[Keywords(KEYWORDS)]   // <- You can change the name of the constant here in the parenthesis.
/// mod keywords {
///     use avpony_macros::Keyword;
///     
///     /// # `if`
///     /// Used to denote `if` blocks (conditional blocks).
///     /// ```avpony
///     /// {#if fruit == "Apple"}
///     ///     You have chosen: Apple.
///     /// {/if}
///     /// ```
///     ///
///     #[Keyword]
///     pub struct If;
/// }
/// ```
#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Keywords(
    args: proc_macro::TokenStream,
    target: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    keywords::make_keywords_module(args, target)
}

///
/// ## INTERNAL-ONLY MACRO.
/// ***
///
/// ## #\[Punctuation]
/// Declares a new piece of punctuation, in a particular category (either `Syntax`, or `Operator`).
///
/// ### Example
/// ```ignore
/// use avpony_macros::Punctuation;
///
/// ///
/// /// The humble comma, a delimiter reserved for only syntax.
/// ///
/// #[Punctuation(',' @ Syntax)]
/// pub struct Comma;
///
/// ///
/// /// The inquisitive question mark: allowed for any use as (or as part of) an operator/
/// ///
/// #[Punctuation('?' @ Operator)]
/// pub struct QuestionMark;
/// ```
#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Punctuation(
    args: proc_macro::TokenStream,
    target: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    punctuation::create_punctuation_for(args, target)
}

///
/// ## INTERNAL-ONLY MACRO.
/// ***
///
/// ## #\[Punctuations]
/// Collects all declared punctuation categories
/// into a const &str per-category.
///
/// ### Example
/// ```ignore
/// use avpony_macros::Punctuations;
///
///                           // All declared punctuation will be put in `punct::SYNTAX`.
/// #[Punctuations(Syntax)]   // <- You can add extra categories here, by adding extra categories (e.g. `Syntax, Operator`).
/// mod punct {
///     use avpony_macros::Punctuation;
///     ///
///     /// The humble comma, a delimiter reserved for only syntax.
///     ///
///     #[Punctuation(',' @ Syntax)]
///     pub struct Comma;
/// }
/// ```
#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Punctuations(
    list: proc_macro::TokenStream,
    target: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    punctuations::make_module(list, target)
}

///
/// ## INTERNAL-ONLY MACRO.
/// ***
///
/// ## #\[ErrorType]
/// Auto implements all required traits for a custom `ErrorI` type (except `ErrorI` itself).
///
/// ### Example
/// ```ignore
/// use avpony_macros::{ErrorType, Errors};
///
/// use crate::utils::{Span, ErrorI};
///
/// #[ErrorType(super::Error)]
/// pub struct UnmatchedBrackets {
///     span: Span,
/// }
///
/// impl ErrorI for UnmatchedBrackets {
///     fn to_report(self) -> ariadne::Report<'_, Span> {
///         // ...
///     }
/// }
/// ```
#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn ErrorType(
    target: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let target: syn::Path = syn::parse_macro_input!(target);
    let item: syn::Item = syn::parse_macro_input!(item);

    let items_out = match item {
        syn::Item::Struct(st) => error::impl_struct(target, st),
        it => return fatal_error(it.span(), "Expected a struct declaration here!"),
    };

    let mut tokens = proc_macro2::TokenStream::new();
    items_out
        .into_iter()
        .for_each(|item| item.to_tokens(&mut tokens));

    tokens.into()
}

///
/// ## INTERNAL-ONLY MACRO.
/// ***
///
/// ## #\[Errors]
/// Auto implements `ErrorI`, and others for an enum of
/// other `ErrorI` types
///
/// ### Example
/// ```ignore
/// use avpony_macros::IntoError;
///
/// use crate::utils::{Span, ErrorI};
///
/// mod unmatched_brackets;
///
/// #[Errors]
/// pub enum Error {
///     UnmatchedBrackets(unmatched_brackets::UnmatchedBrackets),
/// }
/// ```
#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn Errors(
    _: proc_macro::TokenStream,
    inner: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut tokens = proc_macro2::TokenStream::new();
    generate_error_enum(inner)
        .into_iter()
        .for_each(|item| item.to_tokens(&mut tokens));

    tokens.into()
}
