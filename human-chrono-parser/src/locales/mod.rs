use pt_br::HumanDateParserBrazillianPortugueseParser;
use winnow::{error::ContextError, Parser};

use crate::HumanDateExpr;

pub mod pt_br;

pub enum Locale {
    BrazilianPortuguese,
}

impl Locale {
    pub fn parser(&self) -> Box<dyn Parser<&str, HumanDateExpr, ContextError>> {
        match self {
            Self::BrazilianPortuguese => Box::new(HumanDateParserBrazillianPortugueseParser::new()),
        }
    }
}
