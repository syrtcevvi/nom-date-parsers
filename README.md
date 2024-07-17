# Numeric and language-specific date parsers

The bunch of combinators written with help of [nom](https://docs.rs/nom/latest/nom/) to parse the 
`numeric` and `language-specific` dates. `Russian` and `English` are currently supported.

This crate can be used to write other parsers or standalone to parse [`chrono::NaiveDate`](https://docs.rs/chrono/latest/chrono/struct.NaiveDate.html)s from strings.

## Why?
I work on my telegram bot and provide to my users the way to get some information by the date in the format: dd-mm-yyyy. Indeed some of them get the correct format and successfully receive what they want. Others just throw something vaguely reminiscent of the date and complain that bot can't understand them. So, this crate tries to solve this problem.

## Numeric dates
The following patterns can be recognized
- dd/mm/yyyy
- mm/dd/yyyy
- yyyy/mm/dd
- dd
- dd/mm
- mm/dd

Instead of the `/` symbol the other separators can be used: 
- `/`
- `-`
- `.`
- any number of spaces (whitespaces and tabs)

It's not necessary that separators match in every place, so the `dd/mm-yyyy` or even `dd     mm.yyyy` are acceptable formats

To parse the [`chrono::NaiveDate`] value you can use the following methods:
- dd_only
- dd_mm_only
- mm_dd_only
- dd/mm/yyyy
- mm/dd/yyyy
- yyyy/mm/dd

In case of absence of any date part the corresponding date part of today is used.

## Language-specific days
Each language-specific parsers are put behind the corresponding `feature-flag`, except the `en` which is available by default.

Sometimes its cool to receive a date for a `relative day` or `weekday` (`short_weekday`):
- yesterday
- tomorrow
- short_named_weekday
- full_named_weekday
- named_weekday
- current_named_weekday_only

For more info of usage see the methods documentation