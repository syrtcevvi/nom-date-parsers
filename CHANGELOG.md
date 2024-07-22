# Changelog

All notable changes to this project will be documented in this file.

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