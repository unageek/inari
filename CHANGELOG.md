# Changelog

❗ indicates an breaking change.

## Unreleased

### Added

- Added decoration system.
  - Added a struct `DecoratedInterval`.
  - Added a macro `dec_interval` for constructing a decorated interval.
  - Added an enum `Decoration`.
  - Added an enum variant `IntervalErrorKind::IntvlPartOfNaI`.
  - Added an enum variant `OverlappingState::Undefined`.
- Added a macro rule `interval!("...", exact)` that constructs an interval from an exact interval literal (`exacttointerval` in the standard).

### Changed

- ❗ Renamed `Interval::isMember` to `contains` and swapped the order of the parameters.
- ❗ Changed discriminants of enum `IntervalErrorKind`.
- ❗ Changed discriminants of enum `OverlappingState`.
- ❗ Changed method `IntervalError<T>::value` to consume and return the value.

## v0.1.0 - 2020-07-03

Initial release.
