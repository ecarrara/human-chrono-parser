use chrono::{Days, NaiveDate};
use human_chrono_parser::locales::Locale;

fn main() {
    let now = NaiveDate::from_ymd_opt(2024, 8, 13).unwrap(); // Example: Tuesday, August 13, 2024

    let tommorow =
        human_chrono_parser::parse_relative(&mut "amanhã", &Locale::BrazilianPortuguese, &now);
    println!("{:?}", tommorow);
    // outputs: Some(2024-08-14)

    assert_eq!(tommorow, now.checked_add_days(Days::new(1)));
}
