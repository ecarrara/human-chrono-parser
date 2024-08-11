use chrono::{Datelike, Days, Month, NaiveDate, Utc, Weekday};
use winnow::Parser;

pub mod locales;

use locales::Locale;

pub fn parse(input: &mut &str, locale: &Locale) -> Option<NaiveDate> {
    let now = Utc::now().date_naive();
    parse_relative(input, locale, &now)
}

pub fn parse_relative(input: &mut &str, locale: &Locale, now: &NaiveDate) -> Option<NaiveDate> {
    let mut parser = locale.parser();
    match parser.parse(input) {
        Ok(expr) => expr.relative_to(now),
        Err(_) => None,
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum HumanDateKeyword {
    Today,
    Tomorrow,
    AfterTomorrow,
}

#[derive(Clone, Debug, PartialEq)]
pub enum HumanDateExpr {
    Keyword(HumanDateKeyword),
    InNDays(u64),
    ThisWeekWeekday(Weekday),
    NextWeekWeekday(Weekday),
    OrdinalWeekdayOfMonth(Ordinal, Weekday, Month),
}

impl HumanDateExpr {
    pub fn relative_to(&self, now: &NaiveDate) -> Option<NaiveDate> {
        match self {
            HumanDateExpr::Keyword(keyword) => match keyword {
                HumanDateKeyword::Today => Some(now.clone()),
                HumanDateKeyword::Tomorrow => Some(now.checked_add_days(Days::new(1)).unwrap()),
                HumanDateKeyword::AfterTomorrow => {
                    Some(now.checked_add_days(Days::new(2)).unwrap())
                }
            },
            HumanDateExpr::InNDays(n) => Some(now.checked_add_days(Days::new(*n)).unwrap()),
            HumanDateExpr::ThisWeekWeekday(weekday) => {
                let n = (7 - now.weekday().number_from_sunday() + weekday.number_from_sunday()) % 7;
                Some(now.checked_add_days(Days::new(n.into())).unwrap())
            }
            HumanDateExpr::NextWeekWeekday(weekday) => {
                let n =
                    7 + (7 - now.weekday().number_from_sunday() + weekday.number_from_sunday()) % 7;

                Some(now.checked_add_days(Days::new(n.into())).unwrap())
            }
            HumanDateExpr::OrdinalWeekdayOfMonth(ordinal, weekday, month) => {
                NaiveDate::from_weekday_of_month_opt(
                    now.year(),
                    month.number_from_month(),
                    *weekday,
                    ordinal.as_number(),
                )
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Ordinal {
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

#[cfg(test)]
mod tests {
    use chrono::{Month, NaiveDate, Weekday};

    use super::{HumanDateExpr, HumanDateKeyword, Ordinal};

    #[test]
    fn test_keywords() {
        let now = NaiveDate::from_ymd_opt(2024, 8, 13).unwrap(); // Tue
        assert_eq!(
            HumanDateExpr::Keyword(HumanDateKeyword::Today).relative_to(&now),
            NaiveDate::from_ymd_opt(2024, 8, 13)
        );
        assert_eq!(
            HumanDateExpr::Keyword(HumanDateKeyword::Tomorrow).relative_to(&now),
            NaiveDate::from_ymd_opt(2024, 8, 14)
        );
        assert_eq!(
            HumanDateExpr::Keyword(HumanDateKeyword::AfterTomorrow).relative_to(&now),
            NaiveDate::from_ymd_opt(2024, 8, 15)
        );
    }

    #[test]
    fn test_in_n_days() {
        let now = NaiveDate::from_ymd_opt(2024, 8, 13).unwrap(); // Tue
        assert_eq!(
            HumanDateExpr::InNDays(2).relative_to(&now),
            NaiveDate::from_ymd_opt(2024, 8, 15)
        );
    }

    #[test]
    fn test_this_week_weekday() {
        let now = NaiveDate::from_ymd_opt(2024, 8, 13).unwrap(); // Tue
        assert_eq!(
            HumanDateExpr::ThisWeekWeekday(Weekday::Mon).relative_to(&now),
            NaiveDate::from_ymd_opt(2024, 8, 19)
        );
        assert_eq!(
            HumanDateExpr::ThisWeekWeekday(Weekday::Tue).relative_to(&now),
            NaiveDate::from_ymd_opt(2024, 8, 13)
        );
        assert_eq!(
            HumanDateExpr::ThisWeekWeekday(Weekday::Wed).relative_to(&now),
            NaiveDate::from_ymd_opt(2024, 8, 14)
        );
        assert_eq!(
            HumanDateExpr::ThisWeekWeekday(Weekday::Thu).relative_to(&now),
            NaiveDate::from_ymd_opt(2024, 8, 15)
        );
        assert_eq!(
            HumanDateExpr::ThisWeekWeekday(Weekday::Fri).relative_to(&now),
            NaiveDate::from_ymd_opt(2024, 8, 16)
        );
        assert_eq!(
            HumanDateExpr::ThisWeekWeekday(Weekday::Sat).relative_to(&now),
            NaiveDate::from_ymd_opt(2024, 8, 17)
        );
        assert_eq!(
            HumanDateExpr::ThisWeekWeekday(Weekday::Sun).relative_to(&now),
            NaiveDate::from_ymd_opt(2024, 8, 18)
        );
    }

    #[test]
    fn test_next_week_weekday() {
        let now = NaiveDate::from_ymd_opt(2024, 8, 13).unwrap(); // Tue
        assert_eq!(
            HumanDateExpr::NextWeekWeekday(Weekday::Mon).relative_to(&now),
            NaiveDate::from_ymd_opt(2024, 8, 26)
        );
        assert_eq!(
            HumanDateExpr::NextWeekWeekday(Weekday::Tue).relative_to(&now),
            NaiveDate::from_ymd_opt(2024, 8, 20)
        );
        assert_eq!(
            HumanDateExpr::NextWeekWeekday(Weekday::Wed).relative_to(&now),
            NaiveDate::from_ymd_opt(2024, 8, 21)
        );
        assert_eq!(
            HumanDateExpr::NextWeekWeekday(Weekday::Thu).relative_to(&now),
            NaiveDate::from_ymd_opt(2024, 8, 22)
        );
        assert_eq!(
            HumanDateExpr::NextWeekWeekday(Weekday::Fri).relative_to(&now),
            NaiveDate::from_ymd_opt(2024, 8, 23)
        );
        assert_eq!(
            HumanDateExpr::NextWeekWeekday(Weekday::Sat).relative_to(&now),
            NaiveDate::from_ymd_opt(2024, 8, 24)
        );
        assert_eq!(
            HumanDateExpr::NextWeekWeekday(Weekday::Sun).relative_to(&now),
            NaiveDate::from_ymd_opt(2024, 8, 25)
        );
    }

    #[test]
    fn test_ordinal_weekday_of_month() {
        let now = NaiveDate::from_ymd_opt(2024, 8, 13).unwrap(); // Tue
        assert_eq!(
            HumanDateExpr::OrdinalWeekdayOfMonth(Ordinal::First, Weekday::Sun, Month::October)
                .relative_to(&now),
            NaiveDate::from_ymd_opt(2024, 10, 6)
        );
        assert_eq!(
            HumanDateExpr::OrdinalWeekdayOfMonth(Ordinal::Second, Weekday::Sun, Month::October)
                .relative_to(&now),
            NaiveDate::from_ymd_opt(2024, 10, 13)
        );
        assert_eq!(
            HumanDateExpr::OrdinalWeekdayOfMonth(Ordinal::Third, Weekday::Sun, Month::October)
                .relative_to(&now),
            NaiveDate::from_ymd_opt(2024, 10, 20)
        );
        assert_eq!(
            HumanDateExpr::OrdinalWeekdayOfMonth(Ordinal::Fourth, Weekday::Sun, Month::October)
                .relative_to(&now),
            NaiveDate::from_ymd_opt(2024, 10, 27)
        );
    }
}
