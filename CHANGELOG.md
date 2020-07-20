# Changelog

❗ indicates an breaking change.

## Unreleased

### Added

- Add macros `const_interval!` and `const_dec_interval!` for making constant intervals evaluated at compile time.

## v0.2.1 - 2020-07-17

### Fixed

- Fix interval methods `rad` and `wid` that were not rounding the result up.

## v0.2.0 - 2020-07-09

### Added

- Add the decoration system.
  - Add a struct `DecoratedInterval`.
  - Add a macro `dec_interval!` that constructs a decorated interval.
  - Add an enum `Decoration`.
  - ❗ Add an enum variant `IntervalErrorKind::IntvlPartOfNaI`.
  - ❗ Add an enum variant `OverlappingState::Undefined`.
- Add a macro rule `interval!("...", exact)` that constructs an interval from an exact interval literal (`exactToInterval` in the standard).

### Changed

- ❗ Rename interval method `is_member` to `contains` and swapped the order of the parameters.
- ❗ Change discriminants of enum `IntervalErrorKind`.
- ❗ Change discriminants of enum `OverlappingState`.
- ❗ Make method `IntervalError<T>::value` to consume and return the value.

## v0.1.0 - 2020-07-03

Initial release.
