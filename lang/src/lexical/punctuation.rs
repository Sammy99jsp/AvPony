//!
//! ## Punctuation
//! We define punctuation as ASCII non-(letter or digit) characters, that are split into:
//! * syntax punctuation (one of `,:;`), which are reserved for syntax
//! * operator punctuation (one of `!$%^&*?/#~|@`), see [crate::syntax::operator].
//! 
//! ### Grammar
//! ```text
//! syntax_punct :=  `,` | `:` | `;` 
//! operator_punct := `!` | `$` | `%` | `^` | `&` | `*` | `?`| `/` | `#` | `~` | `|` | `@` | `<` | `>`
//! punct := synatx_punct | operator_punct
//! ```
//! 

use avpony_macros::Punctuations;

#[Punctuations(Syntax)]
mod syntax {
    use avpony_macros::Punctuation;
    use chumsky::Parser;
    
    #[Punctuation(',' @ Syntax)]
    pub struct Comma;
    
    #[Punctuation(':' @ Syntax)]
    pub struct Colon;
    
    #[Punctuation(';' @ Syntax)]
    pub struct Semicolon;
    
    #[Punctuation('.' @ Syntax)]
    pub struct Dot;

    #[Punctuation('=' @ Syntax)]
    pub struct Equals;
}

pub use syntax::*;

#[Punctuations(Operator)]
mod operator {
    use avpony_macros::Punctuation;
    use chumsky::Parser;

    #[Punctuation('!' @ Operator)]
    pub struct Exclaim;

    #[Punctuation('$' @ Operator)]
    pub struct Dollar;

    #[Punctuation('%' @ Operator)]
    pub struct Percent;

    #[Punctuation('^' @ Operator)]
    pub struct Caret;

    #[Punctuation('&' @ Operator)]
    pub struct Ampersand;

    #[Punctuation('*' @ Operator)]
    pub struct Asterisk;

    #[Punctuation('?' @ Operator)]
    pub struct Question;

    #[Punctuation('/' @ Operator)]
    pub struct Slash;

    #[Punctuation('#' @ Operator)]
    pub struct Pound;

    #[Punctuation('~' @ Operator)]
    pub struct Tilde;
    
    #[Punctuation('|' @ Operator)]
    pub struct Pipe;

    #[Punctuation('@' @ Operator)]
    pub struct At;

    #[Punctuation('<' @ Operator)]
    pub struct Lt;

    #[Punctuation('>' @ Operator)]
    pub struct Gt;
}

pub use operator::*;