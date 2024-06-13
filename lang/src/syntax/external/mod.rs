use std::fmt::Debug;

use crate::utils::PonyParser;

pub mod typescript;

pub trait External: PartialEq + Debug + Clone {
    const ID: &'static str;

    type Module: PartialEq + Clone + Debug;
    type Expression: PartialEq + Clone + Debug;

    fn module<'src>() -> impl PonyParser<'src, Self::Module>;
    fn expression<'src>() -> impl PonyParser<'src, Self::Expression>;
}
