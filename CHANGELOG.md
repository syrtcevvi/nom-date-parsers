# Changelog

All notable changes to this project will be documented in this file.

## unreleased
### Added
- `quick::bundle` parser which combines the capabilities of the `forward_from_now` and `backward_from_now` parsers
- New parsers:
    - `short_named_weekday_dot`

## Changed
- `types` module now is `public`
- `named_weekday` parser was extended with `short_named_weekday_dot` parser

## Deleted
- Optional variants of parsers introduced in the `0.2.0`. Now parsers return either *parsed date* or `Error`

## 0.2.0 - 2024-07-22
### Added
- Module `quick` with the following functions:
    - `forward_from_now_opt`
    - `backward_from_now_opt`
- Bundle parsers:
    - `ru` module: `bundle` parser
    - `en` module: `bundle_dmy` and `bundle_mdy` parsers
- Additional `language specific` parsers in the `ru` module:
    - `day_before_yesterday`
    - `day_after_tomorrow`
- Optional versions of the following parsers:
    - `day_before_yesterday` -> `day_before_yesterday_opt` (`ru` module)
    - `yesterday` -> `yesterday_opt`
    - `tomorrow` -> `tomorrow_opt`
    - `day_after_tomorrow` -> `day_after_tomorrow_opt` (`ru` module)
    - `current_named_weekday_only` -> `current_named_weekday_only_opt`