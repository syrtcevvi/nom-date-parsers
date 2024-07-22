# Numeric and language-specific date parsers

The bunch of combinators written with help of [nom](https://docs.rs/nom/latest/nom/) to parse the 
`numeric` and `language-specific` dates. `Russian` (`ru` feature flag) and `English` (`en` feature flag, enabled by default) are currently supported.

This crate can be used to write other parsers or standalone to parse [`chrono::NaiveDate`](https://docs.rs/chrono/latest/chrono/struct.NaiveDate.html)s from strings.

## Why?
I work on my telegram bot and provide to my users the way to get some information by the date in the format: dd-mm-yyyy. Indeed some of them get the correct format and successfully receive what they want. Others just throw something vaguely reminiscent of the date and complain that bot can't understand them. So, this crate tries to solve this problem.

## Numeric dates
The following patterns can be recognized
- dd/mm/y4
- mm/dd/y4
- y4/mm/dd
- dd
- dd/mm
- mm/dd

Instead of the `/` symbol the other separators can be used: 
- `/`
- `-`
- `.`
- any number of spaces (whitespaces and tabs)

It's not necessary that separators match in every place, so the `dd/mm-y4` or even `dd mm.y4` are acceptable formats

To parse the [`chrono::NaiveDate`] value you can use the following methods:
- [dd_only](https://docs.rs/nom-date-parsers/latest/nom_date_parsers/numeric/fn.dd_only.html)
- [dd_mm_only](https://docs.rs/nom-date-parsers/latest/nom_date_parsers/numeric/fn.dd_mm_only.html)
- [mm_dd_only](https://docs.rs/nom-date-parsers/latest/nom_date_parsers/numeric/fn.mm_dd_only.html)
- [dd_mm_y4](https://docs.rs/nom-date-parsers/latest/nom_date_parsers/numeric/fn.dd_mm_y4.html)
- [mm_dd_y4](https://docs.rs/nom-date-parsers/latest/nom_date_parsers/numeric/fn.mm_dd_y4.html)
- [y4_mm_dd](https://docs.rs/nom-date-parsers/latest/nom_date_parsers/numeric/fn.y4_mm_dd.html)

In case of absence of any date part the corresponding date part of today is used.

## Language-specific days
Each language-specific parsers are put behind the corresponding `feature flag` (`ru` or `en`), except the `en` which is available by default.

Sometimes its cool to receive a date for a `relative day` or `weekday` (`fully` or `shortly` named):
- [yesterday](https://docs.rs/nom-date-parsers/latest/nom_date_parsers/i18n/en/fn.yesterday.html)
- [tomorrow](https://docs.rs/nom-date-parsers/latest/nom_date_parsers/i18n/en/fn.tomorrow.html)
- [short_named_weekday](https://docs.rs/nom-date-parsers/latest/nom_date_parsers/i18n/en/fn.short_named_weekday.html)
- [full_named_weekday](https://docs.rs/nom-date-parsers/latest/nom_date_parsers/i18n/en/fn.full_named_weekday.html)
- [named_weekday](https://docs.rs/nom-date-parsers/latest/nom_date_parsers/i18n/en/fn.named_weekday.html)
- [current_named_weekday_only](https://docs.rs/nom-date-parsers/latest/nom_date_parsers/i18n/en/fn.current_named_weekday_only.html)

For more info of usage see the documentation of functions