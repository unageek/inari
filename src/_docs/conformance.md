# Conformance to the standard

inari implements a subset of IEEE Std 1788-2015. This page describes the current status of conformance to the standard.

## Implemented operations

This section lists the interval operations defined in the standard along with their implementations. Most of the operations are implemented in the same manner for [`Interval`] (bare intervals) and [`DecInterval`] (decorated intervals). Any differences in the API for these types is noted explicitly.

Some operations are only available when the crate is built with the conditional feature `gmp`, which is enabled by default.

### Required operations

#### Interval constants

| Operation | Implementation       | Requires `gmp` |
| --------- | -------------------- | -------------- |
| empty()   | [`Interval::EMPTY`]  | -              |
| entire()  | [`Interval::ENTIRE`] | -              |

#### Forward-mode elementary functions

| Operation            | Implementation                                             | Requires `gmp` |
| -------------------- | ---------------------------------------------------------- | -------------- |
| neg(_x_)             | `-x`                                                       | -              |
| add(_x_, _y_)        | `x + y`                                                    | -              |
| sub(_x_, _y_)        | `x - y`                                                    | -              |
| mul(_x_, _y_)        | `x * y`                                                    | -              |
| div(_x_, _y_)        | `x / y`                                                    | -              |
| recip(_x_)           | [`x.recip()`](`Interval::recip`)                           | -              |
| sqr(_x_)             | [`x.sqr()`](`Interval::sqr`)                               | -              |
| sqrt(_x_)            | [`x.sqrt()`](`Interval::sqrt`)                             | -              |
| fma(_x_, _y_, _z_)   | [`x.mul_add(y, z)`](`Interval::mul_add`)                   | -              |
| pown(_x_, _n_)       | [`x.powi(n)`](`Interval::powi`)                            | Yes            |
| pow(_x_, _y_)        | [`x.pow(y)`](`Interval::pow`)                              | Yes            |
| exp(_x_)             | [`x.exp()`](`Interval::exp`)                               | Yes            |
| exp2(_x_)            | [`x.exp2()`](`Interval::exp2`)                             | Yes            |
| exp10(_x_)           | [`x.exp10()`](`Interval::exp10`)                           | Yes            |
| log(_x_)             | [`x.ln()`](`Interval::ln`)                                 | Yes            |
| log2(_x_)            | [`x.log2()`](`Interval::log2`)                             | Yes            |
| log10(_x_)           | [`x.log10()`](`Interval::log10`)                           | Yes            |
| sin(_x_)             | [`x.sin()`](`Interval::sin`)                               | Yes            |
| cos(_x_)             | [`x.cos()`](`Interval::cos`)                               | Yes            |
| tan(_x_)             | [`x.tan()`](`Interval::tan`)                               | Yes            |
| asin(_x_)            | [`x.asin()`](`Interval::asin`)                             | Yes            |
| acos(_x_)            | [`x.acos()`](`Interval::acos`)                             | Yes            |
| atan(_x_)            | [`x.atan()`](`Interval::atan`)                             | Yes            |
| atan2(_y_, _x_)      | [`y.atan2(x)`](`Interval::atan2`)                          | Yes            |
| sinh(_x_)            | [`x.sinh()`](`Interval::sinh`)                             | Yes            |
| cosh(_x_)            | [`x.cosh()`](`Interval::cosh`)                             | Yes            |
| tanh(_x_)            | [`x.tanh()`](`Interval::tanh`)                             | Yes            |
| asinh(_x_)           | [`x.asinh()`](`Interval::asinh`)                           | Yes            |
| acosh(_x_)           | [`x.acosh()`](`Interval::acosh`)                           | Yes            |
| atanh(_x_)           | [`x.atanh()`](`Interval::atanh`)                           | Yes            |
| sign(_x_)            | [`x.sign()`](`Interval::sign`)                             | -              |
| ceil(_x_)            | [`x.ceil()`](`Interval::ceil`)                             | -              |
| floor(_x_)           | [`x.floor()`](`Interval::floor`)                           | -              |
| trunc(_x_)           | [`x.trunc()`](`Interval::trunc`)                           | -              |
| roundTiesToEven(_x_) | [`x.round_ties_even()`](`Interval::round_ties_even`) | -              |
| roundTiesToAway(_x_) | [`x.round()`](`Interval::round`)                           | -              |
| abs(_x_)             | [`x.abs()`](`Interval::abs`)                               | -              |
| min(_x_, _y_)        | [`x.min(y)`](`Interval::min`)                              | -              |
| max(_x_, _y_)        | [`x.max(y)`](`Interval::max`)                              | -              |

#### Reverse-mode elementary functions

Not implemented.

#### Two-output division

| Operation              | Implementation                | Requires `gmp` |
| ---------------------- | ----------------------------- | -------------- |
| mulRevToPair(_x_, _y_) | [`Interval::mul_rev_to_pair`] | -              |

#### Cancellative addition and subtraction

| Operation             | Implementation             | Requires `gmp` |
| --------------------- | -------------------------- | -------------- |
| cancelMinus(_x_, _y_) | [`Interval::cancel_minus`] | -              |
| cancelPlus(_x_, _y_)  | [`Interval::cancel_plus`]  | -              |

#### Set operations

| Operation              | Implementation                                  | Requires `gmp` |
| ---------------------- | ----------------------------------------------- | -------------- |
| intersection(_x_, _y_) | [`x.intersection(y)`](`Interval::intersection`) | -              |
| convexHull(_x_, _y_)   | [`x.convex_hull(y)`](`Interval::convex_hull`)   | -              |

#### Constructors

| Operation                | Implementation                                                                                                                                                                       | Requires `gmp` |
| ------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | -------------- |
| numsToInterval(_a_, _b_) | [`interval!(a, b)`](`interval!`)<br>[`const_interval!(a, b)`](`const_interval!`)<br>[`dec_interval!(a, b)`](`dec_interval!`)<br>[`const_dec_interval!(a, b)`](`const_dec_interval!`) | -              |
| textToInterval(_s_)      | [`interval!(s)`](`interval!`)<br>[`dec_interval!(s)`](`dec_interval!`)                                                                                                               | Yes            |

#### Numeric functions of intervals

| Operation | Implementation               | Requires `gmp` |
| --------- | ---------------------------- | -------------- |
| inf(_x_)  | [`x.inf()`](`Interval::inf`) | -              |
| sup(_x_)  | [`x.sup()`](`Interval::sup`) | -              |
| mid(_x_)  | [`x.mid()`](`Interval::mid`) | -              |
| wid(_x_)  | [`x.wid()`](`Interval::wid`) | -              |
| rad(_x_)  | [`x.rad()`](`Interval::rad`) | -              |
| mag(_x_)  | [`x.mag()`](`Interval::mag`) | -              |
| mig(_x_)  | [`x.mig()`](`Interval::mig`) | -              |

#### Boolean functions of intervals

| Operation                | Implementation                                            | Requires `gmp` |
| ------------------------ | --------------------------------------------------------- | -------------- |
| isEmpty(_x_)             | [`x.is_empty()`](`Interval::is_empty`)                    | -              |
| isEntire(_x_)            | [`x.is_entire()`](`Interval::is_entire`)                  | -              |
| isNaI(_x_)               | [`x.is_nai()`](`DecInterval::is_nai`) for [`DecInterval`] | -              |
| equal(_x_, _y_)          | `x == y`                                                  | -              |
| subset(_x_, _y_)         | [`x.subset(y)`](`Interval::subset`)                       | -              |
| less(_x_, _y_)           | [`x.less(y)`](`Interval::less`)                           | -              |
| precedes(_x_, _y_)       | [`x.precedes(y)`](`Interval::precedes`)                   | -              |
| interior(_x_, _y_)       | [`x.interior(y)`](`Interval::interior`)                   | -              |
| strictLess(_x_, _y_)     | [`x.strict_less(y)`](`Interval::strict_less`)             | -              |
| strictPrecedes(_x_, _y_) | [`x.strict_precedes(y)`](`Interval::strict_precedes`)     | -              |
| disjoint(_x_, _y_)       | [`x.disjoint(y)`](`Interval::disjoint`)                   | -              |

### Recommended operations

#### Forward-mode elementary functions

Not implemented.

#### Slope functions

Not implemented.

#### Boolean functions of intervals

| Operation             | Implementation                                             | Requires `gmp` |
| --------------------- | ---------------------------------------------------------- | -------------- |
| isCommonInterval(_x_) | [`x.is_common_interval()`](`Interval::is_common_interval`) | -              |
| isSingleton(_x_)      | [`x.is_singleton()`](`Interval::is_singleton`)             | -              |
| isMember(_x_, _y_)    | [`y.contains(x)`](`Interval::contains`)                    | -              |

#### Extended interval comparison

| Operation         | Implementation                                                                                        | Requires `gmp` |
| ----------------- | ----------------------------------------------------------------------------------------------------- | -------------- |
| overlap(_x_, _y_) | [`x.overlap(y)`](`Interval::overlap`)<br>[`x.overlap(y)`](`DecInterval::overlap`) for [`DecInterval`] | -              |

#### Exact reduction operations

Not implemented.

### Operations on/with decorations

| Operation           | Implementation                                          | Requires `gmp` |
| ------------------- | ------------------------------------------------------- | -------------- |
| newDec(_x_)         | [`DecInterval::new(x)`](`DecInterval::new`)             | -              |
| intervalPart(_x_)   | [`x.interval()`](`DecInterval::interval`)               | -              |
| decorationPart(_x_) | [`x.decoration()`](`DecInterval::decoration`)           | -              |
| setDec(_x_, _dx_)   | [`DecInterval::set_dec(x, dx)`](`DecInterval::set_dec`) | -              |

Comparison of decorations is implemented as defined in the standard.

### Input and output (I/O) of intervals

#### Input

See [Constructors](#constructors).

#### Output

| Operation           | Implementation                                                                                                                               | Requires `gmp` |
| ------------------- | -------------------------------------------------------------------------------------------------------------------------------------------- | -------------- |
| intervalToText(_x_) | `format!("{}", x)` (fixed-point, lowercase)<br>`format!("{:e}", x)` (scientific, lowercase)<br>`format!("{:x}", x)` (hexadecimal, lowercase) | Yes            |

#### Exact text representation

| Operation            | Implementation                            | Requires `gmp` |
| -------------------- | ----------------------------------------- | -------------- |
| intervalToExact(_x_) | `format!("{:x}", x)`                      | Yes            |
| exactToInterval(_s_) | [`interval_exact!(s)`](`interval_exact!`) | Yes            |

### Interchange representations and encodings

Implemented as [`x.to_be_bytes()`](`Interval::to_be_bytes`), [`x.to_le_bytes()`](`Interval::to_le_bytes`), [`x.to_ne_bytes()`](`Interval::to_ne_bytes`), [`x.try_from_be_bytes()`](`Interval::try_from_be_bytes`), [`x.try_from_le_bytes()`](`Interval::try_from_le_bytes`) and [`x.try_from_ne_bytes()`](`Interval::try_from_ne_bytes`). These operations do not require `gmp`.

## Implementation conformance questionnaire

a. Implementation-defined behavior

1. What status flags or other means to signal the occurrence of certain decoration values in computations does the implementation provide if any?

   inari does not signal occurrence of decoration values.

Does the implementation provide the set-based flavor? If so answer the following set of questions.

b. Documentation of behavior

1. If the implementation supports implicit interval types, how is the interval hull operation realized?

   inari does not have implicit interval types.

2. What accuracy is achieved (i.e., tightest, accurate, or valid) for each of the implementation’s interval operations?

   Unless otherwise noted, all operations return the tightest results.

3. Under what conditions is a constructor unable to determine whether a Level 1 value exists that corresponds to the supplied inputs?

   [`interval!(s)`](`interval!`) and [`dec_interval!(s)`](`dec_interval!`) return an [`Err`] with [`IntervalErrorKind::PossiblyUndefinedOperation`] when the exponent does not fit within the range of [`i32`], or the mantissa has impractically many digits after the decimal point.

4. How are cases for rounding a Level 1 value to an F-number handled that are not covered by the rules given in 12.12.8?

   See the documentation of [`Interval::mid`] and [`Interval::rad`].

5. How are interval datums converted to their exact text representations?

   The user can use `format!("{:x}", x)` to convert an interval to its exact representation. The result is a hexadecimal interval literal, the detail of which may depend on implementation of MPFR.

c. Implementation-defined behavior

1. Does the implementation include the interval overlapping function? If so, how is it made available to the user?

   Yes, it is provided by [`Interval::overlap`] and [`DecInterval::overlap`]. Note that the former returns [`Overlap`], while the latter returns [`Option<Overlap>`].

2. Does the implementation store additional information in a NaI? What functions are provided for the user to set and read this information?

   No, no additional information is stored in NaIs.

3. What means if any does the implementation provide for an exception to be signaled when a NaI is produced?

   inari does not signal production of NaIs.

4. What interval types are supported besides the required ones?

   inari does not have additional interval types.

5. What mechanisms of exception handling are used in exception handlers provided by the implementation? What additional exception handling is provided by the implementation?

   Some operations that may fail return [`Option<T>`] or [`Result<T>`], which can be handled with the Rust's standard error handling mechanism.

6. What is the tie-breaking method used in rounding of supported number formats F that are not IEEE 754 conforming?

   inari does not support non IEEE 754 number formats.

7. Does the implementation include different versions of the same operation for a given type and how are these provided to the user?

   There are no such operations at the moment.

8. What combinations of formats are supported in interval constructors?

   [`interval!(a, b)`](`interval!`), [`dec_interval!(a, b)`](`dec_interval!`) and their `const_*` versions take two [`f64`] values. [`interval!(s)`](`interval!`) and [`dec_interval!(s)`](`dec_interval!`) take a string slice.

9. What is the tightness of the result of constructor calls in cases where the standard does not specify it?

   There are no such cases for the supported interval types.

10. What methods are used to read or write strings from or to character streams? Does the implementation employ variations in locales (such as specific character case matching)? This includes the syntax used in the strings for reading and writing.

    Rust's UTF-8-encoded string types are used. Only the syntax specified in the standard are supported as an input/output. For output, there are a few options for controlling the format.

11. What is the tightness of the string to interval conversion for non IEEE 754 conforming interval types and the tightness for the interval to string conversion for all interval types?

    inari does not support non IEEE 754 interval types. The tightness of output is _valid_.

12. What is the result of Level 3 operations for invalid inputs?

    inari prevents creation of invalid interval datums. For invalid inputs, bare interval constructors return an [`Err`] with either [`IntervalErrorKind::UndefinedOperation`] or [`IntervalErrorKind::PossiblyUndefinedOperation`]. See also the answers to b.3 and c.5.

13. What are the interchange representations of the fields of the standard Level 3 representation listed in 14.4?

    The user can choose from the big-endian, the little-endian and the target platform's native byte orders as the interchange representation of the fields.

14. What decorations does the implementation provide and what is their mathematical definition? How are these decorations mapped when converting an interval to the interchange format?

    The decorations `com`, `dac`, `def`, `trv` and `ill` are provided. See [intro] for their definitions. Each decoration is mapped to `16`, `12`, `8`, `4` and `0`, respectively, in the interchange representations.

15. What interchange formats if any are provided for non IEEE 754 interval formats and on non IEEE 754 systems? How are these provided to the user?

    No such interval formats/number systems are supported.

Does the implementation support the compressed arithmetic sub-profile of the set-based profile? If so answer the following set of questions.

No, compressed interval formats are not supported.

Does the implementation provide non-standard flavors not defined in this standard? If so answer the following questions for each additional flavor.

No, non-standard flavors are not supported.
