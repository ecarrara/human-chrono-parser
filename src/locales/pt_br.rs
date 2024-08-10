use std::str::FromStr;

use chrono::{Datelike, Days, Local, NaiveDate, Weekday};
use winnow::{
    ascii::{digit1, space1},
    combinator::{alt, opt},
    PResult, Parser,
};

use crate::HumanDateParser;

pub struct HumanDateParserBrazillianPortuguese;

impl HumanDateParserBrazillianPortuguese {
    pub fn parse(text: &str) -> Option<NaiveDate> {
        let now = Local::now().naive_local().date();
        HumanDateParserBrazillianPortuguese::parse_relative(text, now)
    }
}

impl HumanDateParser for HumanDateParserBrazillianPortuguese {
    fn parse_relative(text: &str, now: NaiveDate) -> Option<NaiveDate> {
        match parse(text) {
            Ok((_, expr)) => match expr {
                HumanDateExpr::Keyword(keyword) => match keyword {
                    HumanDateKeyword::Today => Some(now),
                    HumanDateKeyword::Tomorrow => Some(now.checked_add_days(Days::new(1)).unwrap()),
                    HumanDateKeyword::AfterTomorrow => {
                        Some(now.checked_add_days(Days::new(2)).unwrap())
                    }
                },
                HumanDateExpr::InNDays(n) => Some(now.checked_add_days(Days::new(n)).unwrap()),
                HumanDateExpr::ThisWeekWeekday(weekday) => {
                    let n =
                        (7 - now.weekday().number_from_sunday() + weekday.number_from_sunday()) % 7;
                    Some(now.checked_add_days(Days::new(n.into())).unwrap())
                }
                HumanDateExpr::NextWeekWeekday(weekday) => {
                    let n = 7
                        + (7 - now.weekday().number_from_sunday() + weekday.number_from_sunday())
                            % 7;

                    Some(now.checked_add_days(Days::new(n.into())).unwrap())
                }
            },
            Err(err) => {
                eprintln!("error: {}", err);
                None
            }
        }
    }
}

#[derive(Clone)]
enum HumanDateKeyword {
    Today,
    Tomorrow,
    AfterTomorrow,
}

enum HumanDateExpr {
    Keyword(HumanDateKeyword),
    InNDays(u64),
    ThisWeekWeekday(Weekday),
    NextWeekWeekday(Weekday),
}

fn parse(input: &str) -> PResult<(&str, HumanDateExpr)> {
    let mut parser = alt((
        keyword.map(HumanDateExpr::Keyword),
        in_n_days.map(HumanDateExpr::InNDays),
        this_week_weekday.map(HumanDateExpr::ThisWeekWeekday),
        next_week_weekday.map(HumanDateExpr::NextWeekWeekday),
    ));
    let (_, expr) = parser.parse_peek(input)?;
    Ok((input, expr))
}

fn keyword(input: &mut &str) -> PResult<HumanDateKeyword> {
    alt((
        "hoje".value(HumanDateKeyword::Today),
        "amanhã".value(HumanDateKeyword::Tomorrow),
        "depois de amanhã".value(HumanDateKeyword::AfterTomorrow),
    ))
    .parse_next(input)
}

fn in_n_days(input: &mut &str) -> PResult<u64> {
    let (_, n, _) = (
        (alt(("daqui", "em")), space1),
        number,
        (space1, "dia", opt('s')),
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

fn this(input: &mut &str) -> PResult<()> {
    alt(("esta", "essa", "esse", "este"))
        .void()
        .parse_next(input)
}

fn next(input: &mut &str) -> PResult<()> {
    alt((
        "próxima", "proxima", "próximo", "proximo", "próx.", "prox.", "próx", "prox",
    ))
    .void()
    .parse_next(input)
}

fn number(input: &mut &str) -> PResult<u64> {
    alt((
        digit1.try_map(FromStr::from_str),
        "dezessete".value(17),
        "dezesseis".value(16),
        "dezenove".value(19),
        alt(("quatorze", "catorze")).value(14),
        "dezoito".value(18),
        "quinze".value(15),
        "vinte".value(20),
        "treze".value(13),
        "quatro".value(4),
        "três".value(3),
        "onze".value(11),
        "doze".value(12),
        "cinco".value(5),
        "sete".value(7),
        "seis".value(6),
        "oito".value(8),
        "nove".value(9),
        "dois".value(2),
        "dez".value(10),
        "um".value(1),
    ))
    .parse_next(input)
}

fn weekday(input: &mut &str) -> PResult<Weekday> {
    alt((
        alt(("segunda-feira", "segunda feira", "segunda", "seg.", "seg")).value(Weekday::Mon),
        alt((
            "terça-feira",
            "terca-feira",
            "terça feira",
            "terca feira",
            "terça",
            "terca",
            "ter.",
            "ter",
        ))
        .value(Weekday::Tue),
        alt(("quarta-feira", "quarta feira", "quarta", "qua.", "qua")).value(Weekday::Wed),
        alt(("quinta-feira", "quinta feira", "quinta", "qui.", "qui")).value(Weekday::Thu),
        alt(("sexta-feira", "sexta feira", "sexta", "sex.", "sex")).value(Weekday::Fri),
        alt(("sábado", "sabado", "sáb.", "sab.", "sáb", "sab")).value(Weekday::Sat),
        alt(("domingo", "dom.", "dom")).value(Weekday::Sun),
    ))
    .parse_next(input)
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, Weekday};
    use winnow::Parser;

    use super::{
        next, number, this, weekday, HumanDateParser, HumanDateParserBrazillianPortuguese,
    };

    #[test]
    fn text_keywords() {
        let now = NaiveDate::from_ymd_opt(2024, 8, 13).unwrap(); // Tue

        assert_eq!(
            HumanDateParserBrazillianPortuguese::parse_relative("hoje", now),
            NaiveDate::from_ymd_opt(2024, 8, 13)
        );

        assert_eq!(
            HumanDateParserBrazillianPortuguese::parse_relative("amanhã", now),
            NaiveDate::from_ymd_opt(2024, 8, 14)
        );

        assert_eq!(
            HumanDateParserBrazillianPortuguese::parse_relative("depois de amanhã", now),
            NaiveDate::from_ymd_opt(2024, 8, 15)
        );
    }

    #[test]
    fn text_in_n_days() {
        let now = NaiveDate::from_ymd_opt(2024, 8, 13).unwrap(); // Tue

        assert_eq!(
            HumanDateParserBrazillianPortuguese::parse_relative("daqui 2 dias", now),
            NaiveDate::from_ymd_opt(2024, 8, 15)
        );
        assert_eq!(
            HumanDateParserBrazillianPortuguese::parse_relative("em 2 dias", now),
            NaiveDate::from_ymd_opt(2024, 8, 15)
        );
        assert_eq!(
            HumanDateParserBrazillianPortuguese::parse_relative("daqui dois dias", now),
            NaiveDate::from_ymd_opt(2024, 8, 15)
        );
        assert_eq!(
            HumanDateParserBrazillianPortuguese::parse_relative("em três dias", now),
            NaiveDate::from_ymd_opt(2024, 8, 16)
        );
    }

    #[test]
    fn test_this_week_weekday() {
        let now = NaiveDate::from_ymd_opt(2024, 8, 13).unwrap(); // Tue

        assert_eq!(
            HumanDateParserBrazillianPortuguese::parse_relative("esta terça", now),
            NaiveDate::from_ymd_opt(2024, 8, 13)
        );
        assert_eq!(
            HumanDateParserBrazillianPortuguese::parse_relative("esta quarta", now),
            NaiveDate::from_ymd_opt(2024, 8, 14)
        );
        assert_eq!(
            HumanDateParserBrazillianPortuguese::parse_relative("esta quinta", now),
            NaiveDate::from_ymd_opt(2024, 8, 15)
        );
        assert_eq!(
            HumanDateParserBrazillianPortuguese::parse_relative("esta sexta", now),
            NaiveDate::from_ymd_opt(2024, 8, 16)
        );
        assert_eq!(
            HumanDateParserBrazillianPortuguese::parse_relative("este sábado", now),
            NaiveDate::from_ymd_opt(2024, 8, 17)
        );
        assert_eq!(
            HumanDateParserBrazillianPortuguese::parse_relative("este domingo", now),
            NaiveDate::from_ymd_opt(2024, 8, 18)
        );
        assert_eq!(
            HumanDateParserBrazillianPortuguese::parse_relative("esta segunda", now),
            NaiveDate::from_ymd_opt(2024, 8, 19)
        );
    }

    #[test]
    fn test_next_week_weekday() {
        let now = NaiveDate::from_ymd_opt(2024, 8, 13).unwrap(); // Tue

        assert_eq!(
            HumanDateParserBrazillianPortuguese::parse_relative("próxima terça", now),
            NaiveDate::from_ymd_opt(2024, 8, 20)
        );
        assert_eq!(
            HumanDateParserBrazillianPortuguese::parse_relative("próxima quarta", now),
            NaiveDate::from_ymd_opt(2024, 8, 21)
        );
        assert_eq!(
            HumanDateParserBrazillianPortuguese::parse_relative("próxima quinta", now),
            NaiveDate::from_ymd_opt(2024, 8, 22)
        );
        assert_eq!(
            HumanDateParserBrazillianPortuguese::parse_relative("próxima sexta", now),
            NaiveDate::from_ymd_opt(2024, 8, 23)
        );
        assert_eq!(
            HumanDateParserBrazillianPortuguese::parse_relative("próximo sábado", now),
            NaiveDate::from_ymd_opt(2024, 8, 24)
        );
        assert_eq!(
            HumanDateParserBrazillianPortuguese::parse_relative("próximo domingo", now),
            NaiveDate::from_ymd_opt(2024, 8, 25)
        );
        assert_eq!(
            HumanDateParserBrazillianPortuguese::parse_relative("próxima segunda", now),
            NaiveDate::from_ymd_opt(2024, 8, 26)
        );
    }

    #[test]
    fn test_weekday() {
        assert_eq!(weekday.parse_peek("segunda-feira"), Ok(("", Weekday::Mon)));
        assert_eq!(weekday.parse_peek("segunda feira"), Ok(("", Weekday::Mon)));
        assert_eq!(weekday.parse_peek("seg."), Ok(("", Weekday::Mon)));
        assert_eq!(weekday.parse_peek("seg"), Ok(("", Weekday::Mon)));
        assert_eq!(weekday.parse_peek("terça-feira"), Ok(("", Weekday::Tue)));
        assert_eq!(weekday.parse_peek("terca-feira"), Ok(("", Weekday::Tue)));
        assert_eq!(weekday.parse_peek("terça feira"), Ok(("", Weekday::Tue)));
        assert_eq!(weekday.parse_peek("terca feira"), Ok(("", Weekday::Tue)));
        assert_eq!(weekday.parse_peek("ter."), Ok(("", Weekday::Tue)));
        assert_eq!(weekday.parse_peek("ter"), Ok(("", Weekday::Tue)));
        assert_eq!(weekday.parse_peek("quarta-feira"), Ok(("", Weekday::Wed)));
        assert_eq!(weekday.parse_peek("quarta feira"), Ok(("", Weekday::Wed)));
        assert_eq!(weekday.parse_peek("qua."), Ok(("", Weekday::Wed)));
        assert_eq!(weekday.parse_peek("qua"), Ok(("", Weekday::Wed)));
        assert_eq!(weekday.parse_peek("quinta-feira"), Ok(("", Weekday::Thu)));
        assert_eq!(weekday.parse_peek("quinta feira"), Ok(("", Weekday::Thu)));
        assert_eq!(weekday.parse_peek("qui."), Ok(("", Weekday::Thu)));
        assert_eq!(weekday.parse_peek("qui"), Ok(("", Weekday::Thu)));
        assert_eq!(weekday.parse_peek("sexta-feira"), Ok(("", Weekday::Fri)));
        assert_eq!(weekday.parse_peek("sexta feira"), Ok(("", Weekday::Fri)));
        assert_eq!(weekday.parse_peek("sex."), Ok(("", Weekday::Fri)));
        assert_eq!(weekday.parse_peek("sex"), Ok(("", Weekday::Fri)));
        assert_eq!(weekday.parse_peek("sábado"), Ok(("", Weekday::Sat)));
        assert_eq!(weekday.parse_peek("sabado"), Ok(("", Weekday::Sat)));
        assert_eq!(weekday.parse_peek("sáb."), Ok(("", Weekday::Sat)));
        assert_eq!(weekday.parse_peek("sab."), Ok(("", Weekday::Sat)));
        assert_eq!(weekday.parse_peek("sáb"), Ok(("", Weekday::Sat)));
        assert_eq!(weekday.parse_peek("domingo"), Ok(("", Weekday::Sun)));
        assert_eq!(weekday.parse_peek("dom."), Ok(("", Weekday::Sun)));
        assert_eq!(weekday.parse_peek("dom"), Ok(("", Weekday::Sun)));
    }

    #[test]
    fn test_this() {
        assert_eq!(this.parse_peek("esta"), Ok(("", ())));
        assert_eq!(this.parse_peek("essa"), Ok(("", ())));
        assert_eq!(this.parse_peek("esse"), Ok(("", ())));
        assert_eq!(this.parse_peek("este"), Ok(("", ())));
    }

    #[test]
    fn test_next() {
        assert_eq!(next.parse_peek("próxima"), Ok(("", ())));
        assert_eq!(next.parse_peek("proxima"), Ok(("", ())));
        assert_eq!(next.parse_peek("próximo"), Ok(("", ())));
        assert_eq!(next.parse_peek("proximo"), Ok(("", ())));
        assert_eq!(next.parse_peek("próx."), Ok(("", ())));
        assert_eq!(next.parse_peek("prox."), Ok(("", ())));
        assert_eq!(next.parse_peek("prox"), Ok(("", ())));
    }

    #[test]
    fn test_number() {
        assert_eq!(number(&mut "1"), Ok(1));
        assert_eq!(number(&mut "01"), Ok(1));
        assert_eq!(number(&mut "um"), Ok(1));
        assert_eq!(number(&mut "dois"), Ok(2));
        assert_eq!(number(&mut "três"), Ok(3));
        assert_eq!(number(&mut "quatro"), Ok(4));
        assert_eq!(number(&mut "cinco"), Ok(5));
        assert_eq!(number(&mut "seis"), Ok(6));
        assert_eq!(number(&mut "sete"), Ok(7));
        assert_eq!(number(&mut "oito"), Ok(8));
        assert_eq!(number(&mut "nove"), Ok(9));
        assert_eq!(number(&mut "dez"), Ok(10));
        assert_eq!(number(&mut "onze"), Ok(11));
        assert_eq!(number(&mut "doze"), Ok(12));
        assert_eq!(number(&mut "treze"), Ok(13));
        assert_eq!(number(&mut "quatorze"), Ok(14));
        assert_eq!(number(&mut "catorze"), Ok(14)); // before "Acordo Ortográfico da Língua Portuguesa de 1990)"
        assert_eq!(number(&mut "quinze"), Ok(15));
        assert_eq!(number(&mut "dezesseis"), Ok(16));
        assert_eq!(number(&mut "dezessete"), Ok(17));
        assert_eq!(number(&mut "dezoito"), Ok(18));
        assert_eq!(number(&mut "dezenove"), Ok(19));
        assert_eq!(number(&mut "vinte"), Ok(20));
    }
}
