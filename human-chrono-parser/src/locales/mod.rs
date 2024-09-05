use winnow::{error::ContextError, Parser};

use crate::HumanDateExpr;

pub mod en;
pub mod pt_br;

use en::HumanDateParserEnglishParser;
use pt_br::HumanDateParserBrazillianPortugueseParser;

pub enum Locale {
    BrazilianPortuguese,
    English,
}

impl Locale {
    pub fn parser(&self) -> Box<dyn Parser<&str, HumanDateExpr, ContextError>> {
        match self {
            Self::BrazilianPortuguese => Box::new(HumanDateParserBrazillianPortugueseParser::new()),
            Self::English => Box::new(HumanDateParserEnglishParser::new()),
        }
    }
}
