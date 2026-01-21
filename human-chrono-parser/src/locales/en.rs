use std::str::FromStr;

use chrono::{Month, Weekday};
use winnow::{
    ascii::{digit1, space1},
    combinator::{alt, opt},
    error::ContextError,
    PResult, Parser,
};

use crate::{HumanDateExpr, HumanDateKeyword, Ordinal};

pub struct HumanDateParserEnglishParser;

impl HumanDateParserEnglishParser {
    pub fn new() -> Self {
        HumanDateParserEnglishParser {}
    }
}

impl Parser<&str, HumanDateExpr, ContextError> for HumanDateParserEnglishParser {
    fn parse_next(&mut self, input: &mut &str) -> PResult<HumanDateExpr> {
        let mut parser = alt((
            keyword.map(HumanDateExpr::Keyword),
            in_n_days.map(HumanDateExpr::InNDays),
            ordinal_weekday_of_month.map(|(ordinal, weekday, month)| {
                HumanDateExpr::OrdinalWeekdayOfMonth(ordinal, weekday, month)
            }),
            this_week_weekday.map(HumanDateExpr::ThisWeekWeekday),
            next_week_weekday.map(HumanDateExpr::NextWeekWeekday),
        ));
        parser.parse_next(input)
    }
}

fn keyword(input: &mut &str) -> PResult<HumanDateKeyword> {
    alt((
        "today".value(HumanDateKeyword::Today),
        "tomorrow".value(HumanDateKeyword::Tomorrow),
        "day after tomorrow".value(HumanDateKeyword::AfterTomorrow),
    ))
    .parse_next(input)
}

fn in_n_days(input: &mut &str) -> PResult<u64> {
    let (_, n, _) = (
        (alt(("in", "after")), space1),
        number,
        (space1, "day", opt('s')),
    )
        .parse_next(input)?;
    Ok(n)
}

fn this_week_weekday(input: &mut &str) -> PResult<Weekday> {
    let (_, weekday) = (opt((this, space1)), weekday).parse_next(input)?;
    Ok(weekday)
}

fn next_week_weekday(input: &mut &str) -> PResult<Weekday> {
    let (_, _, weekday) = (next, space1, weekday).parse_next(input)?;
    Ok(weekday)
}

fn ordinal_weekday_of_month(input: &mut &str) -> PResult<(Ordinal, Weekday, Month)> {
    let (ordinal, _, weekday, _, _, _, month) =
        (ordinal, space1, weekday, space1, "of", space1, month).parse_next(input)?;
    Ok((ordinal, weekday, month))
}

fn this(input: &mut &str) -> PResult<()> {
    alt(("this", "the current"))
        .void()
        .parse_next(input)
}

fn next(input: &mut &str) -> PResult<()> {
    alt(("next", "the next", "the following"))
        .void()
        .parse_next(input)
}

fn ordinal(input: &mut &str) -> PResult<Ordinal> {
    alt((
        "first".value(Ordinal::First),
        "second".value(Ordinal::Second),
        "third".value(Ordinal::Third),
        "fourth".value(Ordinal::Fourth),
        "fifth".value(Ordinal::Fifth),
    ))
    .parse_next(input)
}

fn number(input: &mut &str) -> PResult<u64> {
    alt((
        digit1.try_map(FromStr::from_str),
        "twenty".value(20),
        "nineteen".value(19),
        "eighteen".value(18),
        "seventeen".value(17),
        "sixteen".value(16),
        "fifteen".value(15),
        "fourteen".value(14),
        "thirteen".value(13),
        "twelve".value(12),
        "eleven".value(11),
        "ten".value(10),
        "nine".value(9),
        "eight".value(8),
        "seven".value(7),
        "six".value(6),
        "five".value(5),
        "four".value(4),
        "three".value(3),
        "two".value(2),
        "one".value(1),
    ))
    .parse_next(input)
}

fn weekday(input: &mut &str) -> PResult<Weekday> {
    alt((
        alt(("monday", "mon")).value(Weekday::Mon),
        alt(("tuesday", "tue")).value(Weekday::Tue),
        alt(("wednesday", "wed")).value(Weekday::Wed),
        alt(("thursday", "thu")).value(Weekday::Thu),
        alt(("friday", "fri")).value(Weekday::Fri),
        alt(("saturday", "sat")).value(Weekday::Sat),
        alt(("sunday", "sun")).value(Weekday::Sun),
    ))
    .parse_next(input)
}

fn month(input: &mut &str) -> PResult<Month> {
    alt((
        alt(("january", "jan")).value(Month::January),
        alt(("february", "feb")).value(Month::February),
        alt(("march", "mar")).value(Month::March),
        alt(("april", "apr")).value(Month::April),
        "may".value(Month::May),
        alt(("june", "jun")).value(Month::June),
        alt(("july", "jul")).value(Month::July),
        alt(("august", "aug")).value(Month::August),
        alt(("september", "sep")).value(Month::September),
        alt(("october", "oct")).value(Month::October),
        alt(("november", "nov")).value(Month::November),
        alt(("december", "dec")).value(Month::December),
    ))
    .parse_next(input)
}

#[cfg(test)]
mod tests {
    use crate::{HumanDateExpr, HumanDateKeyword, Ordinal};
    use chrono::{Month, Weekday};
    use winnow::Parser;

    use super::{next, number, this, weekday, HumanDateParserEnglishParser};

    #[test]
    fn test_keywords() {
        let mut parser = HumanDateParserEnglishParser::new();
        assert_eq!(
            parser.parse_peek("today"),
            Ok(("", HumanDateExpr::Keyword(HumanDateKeyword::Today)))
        );
        assert_eq!(
            parser.parse_peek("tomorrow"),
            Ok(("", HumanDateExpr::Keyword(HumanDateKeyword::Tomorrow)))
        );
        assert_eq!(
            parser.parse_peek("day after tomorrow"),
            Ok(("", HumanDateExpr::Keyword(HumanDateKeyword::AfterTomorrow)))
        );
    }

    #[test]
    fn test_in_n_days() {
        let mut parser = HumanDateParserEnglishParser::new();
        assert_eq!(
            parser.parse_peek("in 2 days"),
            Ok(("", HumanDateExpr::InNDays(2)))
        );
        assert_eq!(
            parser.parse_peek("after 2 days"),
            Ok(("", HumanDateExpr::InNDays(2)))
        );
        assert_eq!(
            parser.parse_peek("in two days"),
            Ok(("", HumanDateExpr::InNDays(2)))
        );
        assert_eq!(
            parser.parse_peek("after two days"),
            Ok(("", HumanDateExpr::InNDays(2)))
        );
    }

    #[test]
    fn test_this_week_weekday() {
        let mut parser = HumanDateParserEnglishParser::new();
        assert_eq!(
            parser.parse_peek("this monday"),
            Ok(("", HumanDateExpr::ThisWeekWeekday(Weekday::Mon)))
        );
        assert_eq!(
            parser.parse_peek("tuesday"),
            Ok(("", HumanDateExpr::ThisWeekWeekday(Weekday::Tue)))
        );
        assert_eq!(
            parser.parse_peek("this wednesday"),
            Ok(("", HumanDateExpr::ThisWeekWeekday(Weekday::Wed)))
        );
        assert_eq!(
            parser.parse_peek("thursday"),
            Ok(("", HumanDateExpr::ThisWeekWeekday(Weekday::Thu)))
        );
        assert_eq!(
            parser.parse_peek("this friday"),
            Ok(("", HumanDateExpr::ThisWeekWeekday(Weekday::Fri)))
        );
        assert_eq!(
            parser.parse_peek("saturday"),
            Ok(("", HumanDateExpr::ThisWeekWeekday(Weekday::Sat)))
        );
        assert_eq!(
            parser.parse_peek("this sunday"),
            Ok(("", HumanDateExpr::ThisWeekWeekday(Weekday::Sun)))
        );
    }

    #[test]
    fn test_next_week_weekday() {
        let mut parser = HumanDateParserEnglishParser::new();
        assert_eq!(
            parser.parse_peek("next monday"),
            Ok(("", HumanDateExpr::NextWeekWeekday(Weekday::Mon)))
        );
        assert_eq!(
            parser.parse_peek("next tuesday"),
            Ok(("", HumanDateExpr::NextWeekWeekday(Weekday::Tue)))
        );
        assert_eq!(
            parser.parse_peek("next wednesday"),
            Ok(("", HumanDateExpr::NextWeekWeekday(Weekday::Wed)))
        );
        assert_eq!(
            parser.parse_peek("next thursday"),
            Ok(("", HumanDateExpr::NextWeekWeekday(Weekday::Thu)))
        );
        assert_eq!(
            parser.parse_peek("next friday"),
            Ok(("", HumanDateExpr::NextWeekWeekday(Weekday::Fri)))
        );
        assert_eq!(
            parser.parse_peek("next saturday"),
            Ok(("", HumanDateExpr::NextWeekWeekday(Weekday::Sat)))
        );
        assert_eq!(
            parser.parse_peek("next sunday"),
            Ok(("", HumanDateExpr::NextWeekWeekday(Weekday::Sun)))
        );
    }

    #[test]
    fn test_ordinal_weekday_of_month() {
        let mut parser = HumanDateParserEnglishParser::new();
        assert_eq!(
            parser.parse_peek("first sun of september"),
            Ok((
                "",
                HumanDateExpr::OrdinalWeekdayOfMonth(
                    Ordinal::First,
                    Weekday::Sun,
                    Month::September
                )
            ))
        );
        assert_eq!(
            parser.parse_peek("second thursday of september"),
            Ok((
                "",
                HumanDateExpr::OrdinalWeekdayOfMonth(
                    Ordinal::Second,
                    Weekday::Thu,
                    Month::September
                )
            ))
        );
        assert_eq!(
            parser.parse_peek("third sunday of september"),
            Ok((
                "",
                HumanDateExpr::OrdinalWeekdayOfMonth(
                    Ordinal::Third,
                    Weekday::Sun,
                    Month::September
                )
            ))
        );
    }

    #[test]
    fn test_weekday() {
        assert_eq!(weekday.parse_peek("monday"), Ok(("", Weekday::Mon)));
        assert_eq!(weekday.parse_peek("mon"), Ok(("", Weekday::Mon)));
        assert_eq!(weekday.parse_peek("tuesday"), Ok(("", Weekday::Tue)));
        assert_eq!(weekday.parse_peek("tue"), Ok(("", Weekday::Tue)));
        assert_eq!(weekday.parse_peek("wednesday"), Ok(("", Weekday::Wed)));
        assert_eq!(weekday.parse_peek("wed"), Ok(("", Weekday::Wed)));
        assert_eq!(weekday.parse_peek("thursday"), Ok(("", Weekday::Thu)));
        assert_eq!(weekday.parse_peek("thu"), Ok(("", Weekday::Thu)));
        assert_eq!(weekday.parse_peek("friday"), Ok(("", Weekday::Fri)));
        assert_eq!(weekday.parse_peek("fri"), Ok(("", Weekday::Fri)));
        assert_eq!(weekday.parse_peek("saturday"), Ok(("", Weekday::Sat)));
        assert_eq!(weekday.parse_peek("sat"), Ok(("", Weekday::Sat)));
        assert_eq!(weekday.parse_peek("sunday"), Ok(("", Weekday::Sun)));
        assert_eq!(weekday.parse_peek("sun"), Ok(("", Weekday::Sun)));
    }

    #[test]
    fn test_this() {
        assert_eq!(this.parse_peek("this"), Ok(("", ())));
        assert_eq!(this.parse_peek("the current"), Ok(("", ())));
    }

    #[test]
    fn test_next() {
        assert_eq!(next.parse_peek("next"), Ok(("", ())));
        assert_eq!(next.parse_peek("the next"), Ok(("", ())));
        assert_eq!(next.parse_peek("the following"), Ok(("", ())));
    }

    #[test]
    fn test_number() {
        assert_eq!(number(&mut "1"), Ok(1));
        assert_eq!(number(&mut "01"), Ok(1));
        assert_eq!(number(&mut "one"), Ok(1));
        assert_eq!(number(&mut "two"), Ok(2));
        assert_eq!(number(&mut "three"), Ok(3));
        assert_eq!(number(&mut "four"), Ok(4));
        assert_eq!(number(&mut "five"), Ok(5));
        assert_eq!(number(&mut "six"), Ok(6));
        assert_eq!(number(&mut "seven"), Ok(7));
        assert_eq!(number(&mut "eight"), Ok(8));
        assert_eq!(number(&mut "nine"), Ok(9));
        assert_eq!(number(&mut "ten"), Ok(10));
        assert_eq!(number(&mut "eleven"), Ok(11));
        assert_eq!(number(&mut "twelve"), Ok(12));
        assert_eq!(number(&mut "thirteen"), Ok(13));
        assert_eq!(number(&mut "fourteen"), Ok(14));
        assert_eq!(number(&mut "fifteen"), Ok(15));
        assert_eq!(number(&mut "sixteen"), Ok(16));
        assert_eq!(number(&mut "seventeen"), Ok(17));
        assert_eq!(number(&mut "eighteen"), Ok(18));
        assert_eq!(number(&mut "nineteen"), Ok(19));
        assert_eq!(number(&mut "twenty"), Ok(20));
    }
}
