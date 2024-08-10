use chrono::NaiveDate;

pub mod locales;

pub trait HumanDateParser {
    fn parse_relative(text: &str, now: NaiveDate) -> Option<NaiveDate>;
}
