use std::str::FromStr;

use chrono::{Month, Weekday};
use winnow::{
    ascii::{digit1, space1},
    combinator::{alt, opt},
    error::ContextError,
    PResult, Parser,
};

use crate::{HumanDateExpr, HumanDateKeyword, Ordinal};

pub struct HumanDateParserBrazillianPortugueseParser;

impl HumanDateParserBrazillianPortugueseParser {
    pub fn new() -> Self {
        HumanDateParserBrazillianPortugueseParser {}
    }
}

impl Parser<&str, HumanDateExpr, ContextError> for HumanDateParserBrazillianPortugueseParser {
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

fn ordinal_weekday_of_month(input: &mut &str) -> PResult<(Ordinal, Weekday, Month)> {
    let (ordinal, _, weekday, _, _, _, month) =
        (ordinal, space1, weekday, space1, "de", space1, month).parse_next(input)?;
    Ok((ordinal, weekday, month))
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

fn ordinal(input: &mut &str) -> PResult<Ordinal> {
    alt((
        alt(("primeira", "primeiro")).value(Ordinal::First),
        alt(("segunda", "segundo")).value(Ordinal::Second),
        alt(("terceira", "terceiro")).value(Ordinal::Third),
        alt(("quarta", "quarto")).value(Ordinal::Fourth),
        alt(("quinta", "quinto")).value(Ordinal::Fifth),
    ))
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

fn month(input: &mut &str) -> PResult<Month> {
    alt((
        alt(("janeiro", "jan.", "jan")).value(Month::January),
        alt(("fevereiro", "fev.", "fev")).value(Month::February),
        alt(("março", "marco", "mar.", "mar")).value(Month::March),
        alt(("abril", "abr.", "abr")).value(Month::April),
        alt(("maio", "mai.", "maio")).value(Month::May),
        alt(("junho", "jun.", "jun")).value(Month::June),
        alt(("julho", "jul.", "jul")).value(Month::July),
        alt(("agosto", "ago.", "ago")).value(Month::August),
        alt(("setembro", "set.", "set")).value(Month::September),
        alt(("outubro", "out.", "out")).value(Month::October),
        alt(("novembro", "nov.", "nov")).value(Month::November),
        alt(("dezembro", "dez.", "dez")).value(Month::December),
    ))
    .parse_next(input)
}

#[cfg(test)]
mod tests {
    use crate::{HumanDateExpr, HumanDateKeyword, Ordinal};
    use chrono::{Month, Weekday};
    use winnow::Parser;

    use super::{next, number, this, weekday, HumanDateParserBrazillianPortugueseParser};

    #[test]
    fn text_keywords() {
        let mut parser = HumanDateParserBrazillianPortugueseParser::new();
        assert_eq!(
            parser.parse_peek("hoje"),
            Ok(("", HumanDateExpr::Keyword(HumanDateKeyword::Today)))
        );
        assert_eq!(
            parser.parse_peek("amanhã"),
            Ok(("", HumanDateExpr::Keyword(HumanDateKeyword::Tomorrow)))
        );
        assert_eq!(
            parser.parse_peek("depois de amanhã"),
            Ok(("", HumanDateExpr::Keyword(HumanDateKeyword::AfterTomorrow)))
        );
    }

    #[test]
    fn text_in_n_days() {
        let mut parser = HumanDateParserBrazillianPortugueseParser::new();
        assert_eq!(
            parser.parse_peek("daqui 2 dias"),
            Ok(("", HumanDateExpr::InNDays(2)))
        );
        assert_eq!(
            parser.parse_peek("em 2 dias"),
            Ok(("", HumanDateExpr::InNDays(2)))
        );
        assert_eq!(
            parser.parse_peek("daqui dois dias"),
            Ok(("", HumanDateExpr::InNDays(2)))
        );
        assert_eq!(
            parser.parse_peek("em dois dias"),
            Ok(("", HumanDateExpr::InNDays(2)))
        );
    }

    #[test]
    fn test_this_week_weekday() {
        let mut parser = HumanDateParserBrazillianPortugueseParser::new();
        assert_eq!(
            parser.parse_peek("essa segunda"),
            Ok(("", HumanDateExpr::ThisWeekWeekday(Weekday::Mon)))
        );
        assert_eq!(
            parser.parse_peek("esta terça"),
            Ok(("", HumanDateExpr::ThisWeekWeekday(Weekday::Tue)))
        );
        assert_eq!(
            parser.parse_peek("esta quarta"),
            Ok(("", HumanDateExpr::ThisWeekWeekday(Weekday::Wed)))
        );
        assert_eq!(
            parser.parse_peek("esta quinta"),
            Ok(("", HumanDateExpr::ThisWeekWeekday(Weekday::Thu)))
        );
        assert_eq!(
            parser.parse_peek("esta sexta"),
            Ok(("", HumanDateExpr::ThisWeekWeekday(Weekday::Fri)))
        );
        assert_eq!(
            parser.parse_peek("este sábado"),
            Ok(("", HumanDateExpr::ThisWeekWeekday(Weekday::Sat)))
        );
        assert_eq!(
            parser.parse_peek("esse domingo"),
            Ok(("", HumanDateExpr::ThisWeekWeekday(Weekday::Sun)))
        );
    }

    #[test]
    fn test_next_week_weekday() {
        let mut parser = HumanDateParserBrazillianPortugueseParser::new();
        assert_eq!(
            parser.parse_peek("próxima segunda"),
            Ok(("", HumanDateExpr::NextWeekWeekday(Weekday::Mon)))
        );
        assert_eq!(
            parser.parse_peek("próxima terça"),
            Ok(("", HumanDateExpr::NextWeekWeekday(Weekday::Tue)))
        );
        assert_eq!(
            parser.parse_peek("próxima quarta"),
            Ok(("", HumanDateExpr::NextWeekWeekday(Weekday::Wed)))
        );
        assert_eq!(
            parser.parse_peek("próxima quinta"),
            Ok(("", HumanDateExpr::NextWeekWeekday(Weekday::Thu)))
        );
        assert_eq!(
            parser.parse_peek("próxima sexta"),
            Ok(("", HumanDateExpr::NextWeekWeekday(Weekday::Fri)))
        );
        assert_eq!(
            parser.parse_peek("próximo sábado"),
            Ok(("", HumanDateExpr::NextWeekWeekday(Weekday::Sat)))
        );
        assert_eq!(
            parser.parse_peek("próximo domingo"),
            Ok(("", HumanDateExpr::NextWeekWeekday(Weekday::Sun)))
        );
    }

    #[test]
    fn test_ordinal_weekday_of_month() {
        let mut parser = HumanDateParserBrazillianPortugueseParser::new();
        assert_eq!(
            parser.parse_peek("primeiro dom. de setembro"),
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
            parser.parse_peek("primeira quinta de setembro"),
            Ok((
                "",
                HumanDateExpr::OrdinalWeekdayOfMonth(
                    Ordinal::First,
                    Weekday::Thu,
                    Month::September
                )
            ))
        );
        assert_eq!(
            parser.parse_peek("segundo domingo de setembro"),
            Ok((
                "",
                HumanDateExpr::OrdinalWeekdayOfMonth(
                    Ordinal::Second,
                    Weekday::Sun,
                    Month::September
                )
            ))
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
