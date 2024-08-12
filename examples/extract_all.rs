use chrono::NaiveDate;
use human_chrono_parser::locales::Locale;

fn main() {
    let now = NaiveDate::from_ymd_opt(2024, 8, 13).unwrap(); // Example: Tuesday, August 13, 2024

    for expr in human_chrono_parser::extract_all(
        &mut "hoje e depois de amanhã e quinta-feira",
        &Locale::BrazilianPortuguese,
    ) {
        println!("{:?}", expr.relative_to(&now));
    }
}
