# Changelog

The ❗ icon indicates a breaking change.

## v0.7.0 - 2020-11-14

### Changed

- ❗ Renamed `DecoratedInterval` to `DecInterval`. Now its name is consistent with the macro `(const_)dec_interval!`.

## v0.6.1 - 2020-09-25

### Fixed

- Fixed the build failure on docs.rs.

## v0.6.0 - 2020-09-24

### Added

- Implement the trait `Eq` for the enum `OverlappingState`.

### Removed

- ❗ Removed the enum variant `OverlappingState::Undefined`.

### Changed

- ❗ Changed the return type of the method `DecoratedInterval::overlap` to `Option<OverlappingState>` instead of `OverlappingState`.
  - Now it returns `None` if at least one of its arguments is NaI.
- ❗ Renamed the enum variant `OverlappingState::Equal` to `Equals`.
- ❗ Changed discriminants of the enum `OverlappingState`, while users must not rely on them.

## v0.5.1 - 2020-08-31

### Fixed

- Fixed the macro `const_dec_interval!` to be usable without importing the macro `const_interval!`.

## v0.5.0 - 2020-08-31

### Changed

- ❗ Renamed the method `(Decorated)Interval::log` to `ln` to make them consistent with `f64::ln`.

## v0.4.0 - 2020-08-25

### Added

- Added type conversions between intervals and byte arrays.
  - `(Decorated)Interval::to_be_bytes`, `to_le_bytes` and `to_ne_bytes` convert intervals into byte arrays.
  - `(Decorated)Interval::try_from_be_bytes`, `try_from_le_bytes` and `try_from_ne_bytes` convert byte arrays into intervals.
- Added the type alias `Result<T> = std::result::Result<T, IntervalError<T>>`.

### Fixed

- Fixed the decoration returned by `atan2`.
  - Previously, `Def` was returned instead of `Dac` in certain cases.
- Fixed the Clippy warning `double_neg` caused by passing a negative value as the lower bound to the macro `const_interval!`.

## v0.3.1 - 2020-08-14

### Fixed

- Fixed decorations returned by constructors of `DecoratedInterval` and elementary functions `acos`, `acosh`, `asin`, `atan2`, `atanh`, `log`, `log2`, `log10` and `pow`.
  - `atan2` is still under investigation. so use it with caution.
  - Fixed broken unit tests.

## v0.3.0 - 2020-08-03

### Added

- Added the elementary functions `(Decorated)Interval::pown`, `pow`, `exp`, `exp2`, `exp10`, `log`, `log2`, `log10`, `sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `atan2`, `sinh`, `cosh`, `tanh`, `asinh`, `acosh` and `atanh`.

- Added the macros `const_interval!` and `const_dec_interval!` which let you make intervals from `f64` numbers in const contexts.

- Added the constants `(Decorated)Interval::EMPTY`, `ENTIRE` and `DecoratedInterval::NAI`.

### Removed

- ❗ Removed the functions `(Decorated)Interval::empty`, `entire` and `DecoratedInterval::nai` in favor of the new constants.

### Changed

- ❗ Changeed the representation of the struct `DecoratedInterval` to `#[repr(C)]` instead of the default one, while users must not rely on it.

## v0.2.1 - 2020-07-17

### Fixed

- Fixed the methods `Interval::rad` and `wid` to round the results up.

## v0.2.0 - 2020-07-09

### Added

- Added the decoration system.
  - Added the struct `DecoratedInterval`.
  - Added the macro `dec_interval!` that constructs a decorated interval.
  - Added the enum `Decoration`.
  - ❗ Added the enum variant `IntervalErrorKind::IntvlPartOfNaI`.
  - ❗ Added the enum variant `OverlappingState::Undefined`.
- Added the macro rule `interval!("...", exact)` that constructs an interval from an exact interval literal (`exactToInterval` in the standard).

### Changed

- ❗ Renamed the function `Interval::is_member` to `contains` and swapped the order of parameters to make it a method.
- ❗ Changed the method `IntervalError<T>::value` to take `self` instead of `&self`.
- ❗ Changed discriminants of the enum `IntervalErrorKind`, while users must not rely on them.
- ❗ Changed discriminants of the enum `OverlappingState`, while users must not rely on them.

## v0.1.0 - 2020-07-03

Initial release.
