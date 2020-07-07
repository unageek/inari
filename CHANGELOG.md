# Changelog

❗ indicates an breaking change.

## Unreleased

### Added

- Added decoration system.
  - Added struct `DecoratedInterval`.
  - Added macro `dec_interval` for constructing a decorated interval.
  - Added enum `Decoration`.
  - Added enum variant `IntervalErrorKind::IntvlPartOfNaI`.
  - Added enum variant `OverlappingState::Undefined`.

### Changed

- ❗ Changed discriminants of enum `IntervalErrorKind`.
- ❗ Changed discriminants of enum `OverlappingState`.
- ❗ Changed method `IntervalError<T>::value` to consume and return the value.

## v0.1.0 - 2020-07-03

Initial release.
