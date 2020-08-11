# inari-graph

![](image.gif)

TODO: Add the command line reference.

### Expression

| Input                                   | Interpreted as                                               | Notes                                                        |
| --------------------------------------- | ------------------------------------------------------------ | ------------------------------------------------------------ |
| `-x`                                    | −*x*                                                         |                                                              |
| `x + y`                                 | *x* + *y*                                                    |                                                              |
| `x - y`                                 | *x* − *y*                                                    |                                                              |
| `x * y`                                 | *x* *y*                                                      |                                                              |
| `x / y`                                 | *x* / *y*                                                    | Undefined for *y* = 0.                                       |
| `sqrt(x)`                               | √*x*                                                         | Undefined for *x* < 0.                                       |
| `x^n`                                   | *x*<sup>*n*</sup>                                            | `n` must be an integer constant.<br />Repetition like `x^2^3` is not supported.<br />See also `exp`, `exp2` and `exp10`. |
| `exp(x)`<br />`exp2(x)`<br />`exp10(x)` | *e*<sup>*x*</sup><br />2<sup>*x*</sup><br />10<sup>*x*</sup> |                                                              |
| `log(x)`<br />`log2(x)`<br />`log10(x)` | log<sub>e</sub> *x*<br />log<sub>2</sub> *x*<br />log<sub>10</sub> *x* | Undefined for *x* ≤ 0.                                       |
| `sin(x)`                                | sin *x*                                                      |                                                              |
| `cos(x)`                                | cos *x*                                                      |                                                              |
| `tan(x)`                                | tan *x*                                                      | Undefined for *x* = (*n* + 1/2)π.                            |
| `asin(x)`                               | sin<sup>-1</sup> *x*                                         | Undefined for *x* < −1 and *x* > 1.                          |
| `acox(x)`                               | cos<sup>-1</sup> *x*                                         | Undefined for *x* < −1 and *x* > 1.                          |
| `atan(x)`                               | tan<sup>-1</sup> *x*                                         |                                                              |
| `atan2(y, x)`                           | tan<sup>-1</sup> (*y* / *x*)                                 | Undefined for (*x*, *y*) = (0, 0).                           |
| `sinh(x)`                               | sinh *x*                                                     |                                                              |
| `cosh(x)`                               | cosh *x*                                                     |                                                              |
| `tanh(x)`                               | tanh *x*                                                     |                                                              |
| `asinh(x)`                              | sinh<sup>-1</sup> *x*                                        |                                                              |
| `acosh(x)`                              | cosh<sup>-1</sup> *x*                                        | Undefined for *x* < 1.                                       |
| `atanh(x)`                              | tanh<sup>-1</sup> *x*                                        | Undefined for *x* ≤ −1 and *x* ≥ 1.                          |
| `abs(x)`                                | \|*x*\|                                                      |                                                              |
| `min(x, y)`                             | min {*x*, *y*}                                               |                                                              |
| `max(x, y)`                             | max {*x*, *y*}                                               |                                                              |
| `floor(x)`                              | ⌊*x*⌋                                                        | [The floor function.](https://en.wikipedia.org/wiki/Floor_and_ceiling_functions) |
| `ceil(x)`                               | ⌈*x*⌉                                                        | [The ceiling function.](https://en.wikipedia.org/wiki/Floor_and_ceiling_functions) |
| `sgn(x)`                                | sgn(*x*)                                                     | [The sign function.](https://en.wikipedia.org/wiki/Sign_function) |
| `mod(x, y)`                             | *x* mod *y*                                                  | [The modulo operation.](https://en.wikipedia.org/wiki/Modulo_operation)<br />The value is nonnegative, *i.e.*, 0 ≤ *x* mod *y* < *y*. |

### Relation

| Input    | Interpreted as | Notes                                                        |
| -------- | -------------- | ------------------------------------------------------------ |
| `x == y` | *x* = *y*      |                                                              |
| `x < y`  | *x* < *y*      |                                                              |
| `x <= y` | *x* ≤ *y*      |                                                              |
| `x > y`  | *x* > *y*      |                                                              |
| `x >= y` | *x* ≥ *y*      |                                                              |
| `X && Y` | *X* ∧ *Y*      | [Logical conjunction.](https://en.wikipedia.org/wiki/Logical_conjunction)<br />`X` and `Y` must be a relation. |
| `X || Y` | *X* ∨ *Y*      | [Logical disjunction.](https://en.wikipedia.org/wiki/Logical_disjunction)<br />`X` and `Y` must be a relation. |

You can group a part of an expression or a relation with `(` … `)`.
