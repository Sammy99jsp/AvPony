//!
//! ## Keywords
//!
//! All the keywords in AvPony.
//!

pub fn is_keyword(st: &str) -> bool {
    KEYWORDS.iter().copied().any(|kw| st == kw)
}

#[avpony_macros::Keywords(KEYWORDS)]
mod kw {
    use avpony_macros::Keyword;
    use chumsky::Parser;

    #[Keyword]
    pub struct If;

    #[Keyword]
    pub struct Else;

    #[Keyword]
    pub struct Await;

    #[Keyword]
    pub struct Then;

    #[Keyword]
    pub struct Match;

    #[Keyword]
    pub struct Case;

    #[Keyword]
    pub struct For;

    #[Keyword]
    pub struct In;

    #[Keyword]
    pub struct Key;

    #[Keyword]
    pub struct Debug;
    
    #[Keyword]
    pub struct True;
    
    #[Keyword]
    pub struct False;
}

pub use kw::*;
