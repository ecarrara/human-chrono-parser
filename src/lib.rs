use chrono::NaiveDate;

pub mod locales;

pub trait HumanDateParser {
    fn parse_relative(text: &str, now: NaiveDate) -> Option<NaiveDate>;
}

#[derive(Clone)]
enum Ordinal {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
}

impl Ordinal {
    fn as_number(&self) -> u8 {
        match self {
            Ordinal::First => 1,
            Ordinal::Second => 2,
            Ordinal::Third => 3,
            Ordinal::Fourth => 4,
            Ordinal::Fifth => 5,
        }
    }
}
