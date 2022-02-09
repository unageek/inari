# Changelog

The icon ⚠️ indicates a breaking change.

## v0.13.0 - 2021-02-09

### Changed

- ⚠️ Renamed `(Dec)Interval::pown` to `(Dec)Interval::powi` so that it matches with the standard method `f64::powi`.

### Added

- Implemented the standard arithmetic operators `std::ops::{Neg, Add, Sub, Mul, Div}` and the corresponding assignment operators `std::ops::{AddAssign, SubAssign, MulAssign, DivAssign}` for `(Dec)Interval` for the cases when one or both of the arguments are given by reference.

## v0.12.0 - 2021-12-16

The crate requires a nightly toolchain >= `nightly-2021-12-16`.

### Fixed

- Fixed build failure on the latest nightly toolchain.

## v0.11.0 - 2021-12-07

The crate requires a nightly toolchain >= `nightly-2021-10-05`.

### Fixed

- Downstream crates are no longer required to specify the `static_assertions` crate as their dependency.

## v0.10.9 - 2021-10-10

- Now inari supports `x86_64-pc-windows-gnu` as a target.

## v0.10.0 - 2021-06-30

### Added

- Implemented the trait `Hash` for `Interval`, `Decoration` and `Overlap`.

### Removed

- ⚠️ Removed the method `IntervalError::value`.

  Previously, the following values were returned by the method:

  | `err.kind()`                                    | `err.value()`                           |
  | ----------------------------------------------- | --------------------------------------- |
  | `IntervalErrorKind::UndefinedOperation`         | `Interval::EMPTY` or `DecInterval::NAI` |
  | `IntervalErrorKind::PossiblyUndefinedOperation` | `(Dec)Interval::ENTIRE`                 |

### Changed

- ⚠️ Changed the return type of the functions `(Dec)Interval::try_from_be_bytes`, `try_from_le_bytes` and `try_from_ne_bytes` to `Option<(Dec)Interval>`.
- ⚠️ Changed the struct `IntervalError` to be non-generic due to the removal of the `value` method.

## v0.9.9 - 2021-06-18

### Fixed

- Fixed the macros `interval!(s)` and `dec_interval!(s)` to return an `Err` with `IntervalErrorKind::PossiblyUndefinedOperation` when the mantissa has too many digits after the decimal point.

## v0.9.8 - 2021-06-06

### Fixed

- Fixed build failure with the latest nightly rust compiler: `rustc 1.54.0-nightly (6c2dd251b 2021-06-05)`.

## v0.9.4 - 2021-05-09

### Fixed

- Fixed the functions `(Dec)Interval::try_from_be_bytes`, `try_from_le_bytes` and `try_from_ne_bytes` to correctly deserialize the empty interval.

## v0.9.2 - 2021-03-09

### Added

- Experimental support for AArch64.

## v0.9.0 - 2021-02-20

### Changed

- ⚠️ Explicitly check the target CPU features during compilation. From this version, you need to specify `RUSTFLAGS='-Ctarget-cpu=<CPU>'`, where `<CPU>` must be `haswell` or a newer microarchitecture; otherwise, build fails.

## v0.8.0 - 2020-11-24

### Added

- Experimental support for AVX-512F.
  - To try it out, run cargo with `RUSTFLAGS='-Ctarget-feature=+avx512f'`.
  - Basic arithmetic operations are expected to get faster, but it is not benchmarked yet.

### Removed

- ⚠️ Removed the enum variant `IntervalErrorKind::IntvlPartOfNaI`.

### Changed

- ⚠️ Renamed the method `(Dec)Interval::round_ties_to_away` to `round`.
- ⚠️ Renamed the methods `DecInterval::interval_part` and `decoration_part` to `interval` and `decoration`, respectively.
- ⚠️ Changed the return type of the method `DecInterval::interval` (former `interval_part`) to `Option<Interval>` from `Result<Interval>`.
- ⚠️ Renamed the enum `OverlappingState` to `Overlap`.

## v0.7.0 - 2020-11-14

### Changed

- ⚠️ Renamed `DecoratedInterval` to `DecInterval`. Now its name is consistent with the macro `(const_)dec_interval!`.

## v0.6.1 - 2020-09-25

### Fixed

- Fixed the build failure on docs.rs.

## v0.6.0 - 2020-09-24

### Added

- Implemented the trait `Eq` for the enum `OverlappingState`.

### Removed

- ⚠️ Removed the enum variant `OverlappingState::Undefined`.

### Changed

- ⚠️ Changed the return type of the method `DecoratedInterval::overlap` to `Option<OverlappingState>` from `OverlappingState`.
  - Now it returns `None` if at least one of its arguments is NaI.
- ⚠️ Renamed the enum variant `OverlappingState::Equal` to `Equals`.

## v0.5.1 - 2020-08-31

### Fixed

- Fixed the macro `const_dec_interval!` to be usable without importing the macro `const_interval!`.

## v0.5.0 - 2020-08-31

### Changed

- ⚠️ Renamed the method `(Decorated)Interval::log` to `ln` to make them consistent with `f64::ln`.

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

- ⚠️ Removed the functions `(Decorated)Interval::empty`, `entire` and `DecoratedInterval::nai` in favor of the new constants.

### Changed

- ⚠️ Changeed the representation of the struct `DecoratedInterval` to `#[repr(C)]` from the default one, while users must not rely on it.

## v0.2.1 - 2020-07-17

### Fixed

- Fixed the methods `Interval::rad` and `wid` to round the results up.

## v0.2.0 - 2020-07-09

### Added

- Added the decoration system.
  - Added the struct `DecoratedInterval`.
  - Added the macro `dec_interval!` that constructs a decorated interval.
  - Added the enum `Decoration`.
  - ⚠️ Added the enum variant `IntervalErrorKind::IntvlPartOfNaI`.
  - ⚠️ Added the enum variant `OverlappingState::Undefined`.
- Added the macro rule `interval!("...", exact)` that constructs an interval from an exact interval literal (`exactToInterval` in the standard).

### Changed

- ⚠️ Renamed the function `Interval::is_member` to `contains` and swapped the order of parameters to make it a method.
- ⚠️ Changed the method `IntervalError<T>::value` to take `self` instead of `&self`.

## v0.1.0 - 2020-07-03

Initial release.
