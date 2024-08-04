/*
    This example provides the interactive way to test some existing date parsers.

    Sample of one run:
    $ cargo run --release --example cli
    > Today is: 2024-08-04
    $ + 10
    > recognized: 2024-08-14
    $ 10
    > recognized: 2024-08-10
    $ 22-04
    > recognized: 2024-04-22
    $ yesterday
    > recognized: 2024-08-03

    Try other variations, supported by the `quick::bundle` and `en::bundle_dmy` parsers out!
*/

use std::io;

use chrono::{Local, NaiveDate};
use nom::branch::alt;

use nom_date_parsers::{i18n::en, quick, types::IResult};

fn versatile_parser(input: &str) -> IResult<&str, NaiveDate> {
    // Its essential to provide parsers in the correct order due to the fact that
    // `+10` pattern can be recognized by the `numeric::dd_only` parser instead of
    // `quick::forward_from_now`
    alt((quick::bundle, en::bundle_dmy))(input)
    /*
        Uncomment these lines, comment previous one, run example and try to type `42` as input. You will see smth like:
        "unable to recognize the input as a date: Parsing Error: DayOutOfRange"
    */
    // use nom_date_parsers::prelude::dd_only;
    // dd_only(input)
}

fn main() -> anyhow::Result<()> {
    println!("Today is: {}", Local::now().date_naive());

    for line in io::stdin().lines() {
        match line {
            Ok(line) => {
                match versatile_parser(&line).map(|r| r.1) {
                    Ok(date) => {
                        /*
                           N.B. due to the nature of the nom, the non-existent date `31-02-2024` will be parsed by the
                           `dd_only` parser as `31` without throwing an error if the `31-<current month>-<current year>` date exists
                        */
                        println!("recognized: {}", date);
                    }
                    Err(err) => {
                        /*
                           Due to the fact that `bundle` parsers use `alt` combinator all related-errors are
                           shadowed by the `Parsing Error: Nom("42", Tag)` error. So, if you want to see that `42` input is out of day-part range,
                           it's impossible with `bundle` parsers, use `dd_only` instead
                        */
                        println!("unable to recognize the input as a date: {err}");
                    }
                }
            }
            Err(_) => break,
        }
    }

    Ok(())
}
