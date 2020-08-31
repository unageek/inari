# Changelog

The ❗ icon indicates a breaking change.

## v0.5.1 - 2020-08-31

### Fixed

- Fix `const_dec_interval!` macro to be usable without declaring `use inari::const_interval;`.

## v0.5.0 - 2020-08-31

### Changed

- ❗ Rename `(Decorated)Interval::log` to `ln` to make them consistent with `f64::ln`.

## v0.4.0 - 2020-08-25

### Added

- Add type conversions between intervals and byte arrays.
  - `(Decorated)Interval::to_be_bytes`, `to_le_bytes` and `to_ne_bytes` convert intervals into byte arrays.
  - `(Decorated)Interval::try_from_be_bytes`, `try_from_le_bytes` and `try_from_ne_bytes` convert byte arrays into intervals.
- Add a type alias: `Result<T> = std::result::Result<T, IntervalError<T>>`.

### Fixed

- Fix decoration returned by `atan2`; `Def` was returned instead of `Dac` in certain cases.
- Fix Clippy warning `double_neg` caused by passing a negative value to the lower bound of `const_interval!` macro.

## v0.3.1 - 2020-08-14

### Fixed

- Fix decoration returned by the constructors of `DecoratedInterval` and elementary functions `acos`, `acosh`, `asin`, `atan2`, `atanh`, `log`, `log2`, `log10` and `pow`.
  - `atan2` is still under investigation, so use it with caution.
  - Fix broken unit tests.

## v0.3.0 - 2020-08-03

### Added

- Add elementary functions `pown`, `pow`, `exp`, `exp2`, `exp10`, `log`, `log2`, `log10`, `sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `atan2`, `sinh`, `cosh`, `tanh`, `asinh`, `acosh` and `atanh`.

- Add macros `const_interval!` and `const_dec_interval!` which let you make intervals from `f64` numbers in a const context.

- Add constants `(Decorated)Interval::EMPTY`, `(Decorated)Interval::ENTIRE` and `DecoratedInterval::NAI`.

### Removed

- ❗ Remove functions `(Decorated)Interval::empty()`, `(Decorated)Interval::entire()` and `DecoratedInterval::nai()`.

### Changed

- ❗ Change the representation of struct `DecoratedInterval` from the default one to `#[repr(C)]`.

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
