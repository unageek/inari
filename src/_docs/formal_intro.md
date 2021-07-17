# Formal introduction to interval arithmetic

This article describes interval arithmetic (IA) implemented in the crate. The variation of IA is the one defined as the _set-based flavor_ in the IEEE 1788 standards.

## Intervals

An interval is a [closed][closed], [convex][convex] [subset][subset] of $\R$, the set of all real numbers. By definition, $∅$, the [empty set][emptyset] as well as $\R$ are also intervals. The notations of intervals are summarized below:

| Interval notation     | Definition                                            | [Bounded][bounded] in $\R$ |
| --------------------- | ----------------------------------------------------- | :------------------------: |
| $∅$                   | $∅$, the empty set                                    |            Yes             |
| $\set{a}$ or $[a, a]$ | $\set{a}$, where $a ∈ \R$                             |            Yes             |
| $[a, b]$              | $\set{x ∈ \R ∣ a ≤ x ≤ b}$, where $a, b ∈ \R ∧ a ≤ b$ |            Yes             |
| $[a, +∞]$             | $\set{x ∈ \R ∣ a ≤ x}$, where $a ∈ \R$                |             No             |
| $[-∞, b]$             | $\set{x ∈ \R ∣ x ≤ b}$, where $b ∈ \R$                |             No             |
| $\R$ or $[-∞, +∞]$    | $\R$, the set of all real numbers                     |             No             |

The notation above can be rationalized by introducing the [extended real numbers][xreals] $\XR$, which is a superset of $\R$ with two extra elements, $+∞$ and $-∞$:

$$
\XR = \R ∪ \set{+∞, -∞}.
$$

$\XR$ is a [totally ordered set][toset] extending the standard ordering of $\R$ with the following rule:

$$
∀x ∈ \R : -∞ < x < +∞.
$$

Every subset of $\XR$ has both an [infimum][inf] and a [supremum][sup] in $\XR$.

Now we can write $\IR ⊆ \powerset(\R)$, the set of all intervals as:

$$
\IR = \set{∅} ∪ \set{[a, b] ∣ a ∈ \XR ∖ \set{+∞} ∧ b ∈ \XR ∖ \set{-∞} ∧ a ≤ b},
$$

where $[a, b] = \set{x ∈ \R ∣ a ≤ x ≤ b}$.

An interval is denoted by a bold letter such as $𝒙$ or $𝒚$. An $n$-tuple of intervals $(𝒙\_1, …, 𝒙\_n) ∈ \IR^n$ is also denoted by $𝒙$.

Let $𝒙 ∈ \IR$. The infimum and the supremum of $𝒙$ in $\XR$ are called the _lower bound_ and the _upper bound_ of $𝒙$, respectively. In particular,

$$
\begin{align*}
 \inf 𝒙 &= \begin{cases}
   +∞ & \if 𝒙 = ∅, \\\\
   a  & \if 𝒙 = [a, b],
  \end{cases} \\\\
 \sup 𝒙 &= \begin{cases}
   -∞ & \if 𝒙 = ∅, \\\\
   b  & \if 𝒙 = [a, b].
  \end{cases}
\end{align*}
$$

Note that while the bounds of an interval are members of $\XR$, the interval itself is a subset of $\R$. Therefore, neither $+∞$ nor $-∞$ can belong to an interval.

## Interval extensions of functions

Let $n ≥ 0$, $X ⊆ \R^n$ and $f : X → \R$. A function $𝒇 : \IR^n → \IR$ is said to be an _interval extension_ of $f$ if and only if:

$$
∀𝒙 ∈ \IR^n : 𝒇(𝒙) ⊇ \Rge(f, 𝒙),
$$

where

$$
\Rge(f, 𝒙) = \set{f(x_1, …, x_n) ∣ \textstyle{⋀_{i=1}^n} x_i ∈ 𝒙_i ∧ (x_1, …, x_n) ∈ X}.
$$

Let $\hull : \powerset(\R) → \IR$ be the function that maps every subset of $\R$ to its tightest enclosure in $\IR$:

$$
\begin{align*}
 \hull(X) &= \operatorname{min_⊆} \set{𝒙 ∈ \IR^n ∣ 𝒙 ⊇ X} \\\\
  &= \begin{cases}
    ∅                & \if X = ∅, \\\\
    [\inf X, \sup X] & \otherwise.
   \end{cases}
\end{align*}
$$

The _natural_ interval extension of $f$ is the interval extension of $f$ that maps every $𝒙 ∈ \IR^n$ to the tightest enclosure of $\Rge(f, 𝒙)$ in $\IR$:

$$
𝒇(𝒙) = \hull(\Rge(f, 𝒙)).
$$

Let $𝒇$ be the natural interval extension of $f$. The following holds:

$$
∀𝒙 ∈ \IR^n : [(∃i ∈ \set{1, …, n} : 𝒙\_i = ∅) ⟹ 𝒇(𝒙) = ∅].
$$

### Examples

Here are some examples of the natural interval extensions of functions. The trivial cases where any of the arguments is $∅$ are omitted.

1. Square root $\sqrt{⋅} : [0, ∞] → \R$:

   $$
   \sqrt{[a, b]} = \begin{cases}
     ∅ & \if b < 0, \\\\
     [0, \sqrt{b}] & \if a ≤ 0 ≤ b, \\\\
     [\sqrt{a}, \sqrt{b}] & \otherwise,
    \end{cases}
   $$

   where $\sqrt{+∞} = +∞$.

1. Addition $+ : \R × \R → \R$ and subtraction $- : \R × \R → \R$:

   $$
   \begin{align*}
    [a, b] + [c, d] &= [a + c, b + d], \\\\
    [a, b] - [c, d] &= [a - d, b - c] = [a + (-d), b + (-c)],
   \end{align*}
   $$

   where

   $$
   \begin{gather*}
    ∀x ∈ \R ∪ \set{+∞} : x + (+∞) = +∞ + x = +∞, \\\\
    ∀x ∈ \R ∪ \set{-∞} : x + (-∞) = -∞ + x = -∞, \\\\
    -(±∞) = ∓∞.
   \end{gather*}
   $$

1. Multiplication $× : \R × \R → \R$:

   $[a, b] × [c, d] =$

   |             |  $d ≤ 0$   |               $c < 0 < d$                |  $0 ≤ c$   |
   | :---------: | :--------: | :--------------------------------------: | :--------: |
   |   $b ≤ 0$   | $[bd, ac]$ |                $[ad, ac]$                | $[ad, bc]$ |
   | $a < 0 < b$ | $[bc, ac]$ | $[\min \set{ad, bc}, \max \set{ac, bd}]$ | $[ad, bd]$ |
   |   $0 ≤ a$   | $[bc, ad]$ |                $[bc, bd]$                | $[ac, bd]$ |

   where

   $$
   ∀x ∈ \XR ∖ \set{0} : x × (±∞) = ±∞ × x = \begin{cases}
     ±∞ & \if x > 0, \\\\
     ∓∞ & \if x < 0.
    \end{cases}
   $$

1. Division $/ : \R × \R ∖ \set{0} → \R$:

   $[a, b]/[c, d] =$

   |             |   $d < 0$    | $c < 0 ∧ d = 0$ | $c = d = 0$ | $c < 0 < d$ | $c = 0 ∧ 0 < d$ |   $0 < c$    |
   | :---------: | :----------: | :-------------: | :---------: | :---------: | :-------------: | :----------: |
   |   $b ≤ 0$   | $[b/c, a/d]$ |   $[b/c, +∞]$   |     $∅$     |    $\R$     |   $[-∞, b/d]$   | $[a/c, b/d]$ |
   | $a = 0 = b$ |   $[0, 0]$   |    $[0, 0]$     |     $∅$     |  $[0, 0]$   |    $[0, 0]$     |   $[0, 0]$   |
   | $a < 0 < b$ | $[b/d, a/d]$ |      $\R$       |     $∅$     |    $\R$     |      $\R$       | $[a/c, b/c]$ |
   |   $0 ≤ a$   | $[b/d, a/c]$ |   $[-∞, a/c]$   |     $∅$     |    $\R$     |   $[a/d, +∞]$   | $[a/d, b/c]$ |

   where

   $$
   \begin{gather*}
    ∀x ∈ \R : \frac{x}{±∞} = 0, \\\\
    ∀x ∈ \R ∖ \set{0} : \frac{±∞}{x} = \begin{cases}
      ±∞ & \if x > 0, \\\\
      ∓∞ & \if x < 0.
     \end{cases}
   \end{gather*}
   $$

1. Let $c ∈ \R$ and $f : \R^0 → \R$ be the function that maps $∅$ to $c$ (note that $S^0 = \set{∅}$ for any set $S$). The natural interval extension of $f$ is the function $𝒇 : \IR^0 → \IR$ that maps $∅$ to $[c, c]$.

   For this reason, we define the natural interval extension of a real constant $c$ to be $[c, c]$.

## $\IF$-interval extensions of functions

Floating-point arithmetic (FA) is an approximation of the extended real number arithmetic with a nice trade-off between magnitude and accuracy of numbers. The crate provides an efficient implementation of IA by using the binary64 floating-point numbers (the [`f64`] type) for representing and computing with intervals. Consult the IEEE 754 standards for the details of FA.

We denote by $\F ⊆ \XR$ the set of all finite (both normal and subnormal) `f64` numbers, zero, $+∞$ and $-∞$. We refer to a member of $\F$ as a _$\F$-number_.

We denote by $\IF ⊆ \IR$ the set of intervals whose bounds are $\F$-numbers:

$$
\IF = \set{∅} ∪ \set{[a, b] ∣ a ∈ \F ∖ \set{+∞} ∧ b ∈ \F ∖ \set{-∞} ∧ a ≤ b}.
$$

Let $n ≥ 0$, $X ⊆ \R^n$ and $f : X → \R$. A function $𝚏 : \IF^n → \IF$ is said to be an _$\IF$-interval extension_ of $f$ if and only if:

$$
∀𝚡 ∈ \IF^n : 𝚏(𝚡) ⊇ \Rge(f, 𝚡).
$$

Let $\fldown$ and $\flup : \XR → \F$ be the functions that maps every $x ∈ \XR$ to the closest $\F$ number toward $-∞$ and $+∞$ respectively:

$$
\begin{align*}
 \fldown(x) &= \max \set{y ∈ \F ∣ y ≤ x}, \\\\
 \flup(x) &= \min \set{y ∈ \F ∣ x ≤ y}.
\end{align*}
$$

Let $\thull : \powerset(\R) → \IF$ be the function that maps every subset of $\R$ to its tightest enclosure in $\IF$:

$$
\begin{align*}
 \thull(X) &= \operatorname{min_⊆} \set{𝚡 ∈ \IF ∣ 𝚡 ⊇ X} \\\\
  &= \begin{cases}
    ∅                                & \if X = ∅, \\\\
    [\fldown(\inf X), \flup(\sup X)] & \otherwise.
   \end{cases}
\end{align*}
$$

The _tightest_ $\IF$-interval extension of $f$ is the $\IF$-interval extension of $f$ that maps every $𝚡 ∈ \IF^n$ to the tightest enclosure of $\Rge(f, 𝚡)$ in $\IF$:

$$
𝚏(𝚡) = \thull(\Rge(f, 𝚡)).
$$

### Examples

Here are some examples of the _tightest_ $\IF$-interval extensions of functions.

1. Addition $+ : \R × \R → \R$:

   $$
   \operatorname{\mathtt{add}}([𝚊, 𝚋], [𝚌, 𝚍]) = [\fldown(𝚊 + 𝚌), \flup(𝚋 + 𝚍)].
   $$

1. $π = 3.14159265358979323…$:

   $$
   \mathtt{pi} = [\mathtt{3.14159265358979311…}, \mathtt{3.14159265358979356…}].
   $$

## The decoration system

The decoration system gives us some additional information on the underlying function of an interval extension being evaluated, such as whether its value is defined or whether it is [continuous][continuous] on the input intervals.

We denote by $\D$ the set of decorations:

$$
\D = \set{\com, \dac, \def, \trv, \ill}.
$$

Their names are abbreviations of **com**mon, **d**efined **a**nd **c**ontinuous, **def**ined, **tr**i**v**ial and **ill**-formed, respectively. $\D$ is a totally ordered set with the following ordering rules:

$$
\com > \dac > \def > \trv > \ill.
$$

Let $n ≥ 0$, $X ⊆ \R^n$, $f : X → \R$, $𝒙 ∈ \IR^n$ and $𝒚 ∈ \IR$. We define the following [predicates](https://proofwiki.org/wiki/Definition:Propositional_Function):

$$
\begin{align*}
p\_\com(f, 𝒙, 𝒚) &:⟺ ∅ ≠ 𝒙 ⊆ X ∧ (f \text{ is continuous on } 𝒙) ∧ (\text{$𝒙$ and $𝒚$ are bounded}), \\\\
p\_\dac(f, 𝒙, 𝒚) &:⟺ ∅ ≠ 𝒙 ⊆ X ∧ (f{↾\_𝒙} \text{ is continuous}), \\\\
p\_\def(f, 𝒙, 𝒚) &:⟺ ∅ ≠ 𝒙 ⊆ X, \\\\
p\_\trv(f, 𝒙, 𝒚) &:⟺ (\text{always true}), \\\\
p\_\ill(f, 𝒙, 𝒚) &:⟺ X = ∅,
\end{align*}
$$

where $f{↾_𝒙}$ is the [restriction] of $f$ to $𝒙$. The following implications hold:

$$
\begin{gather*}
p\_\com(f, 𝒙, 𝒚) ⟹ p\_\dac(f, 𝒙, 𝒚) ⟹ p\_\def(f, 𝒙, 𝒚) ⟹ p\_\trv(f, 𝒙, 𝒚), \\\\
p\_\ill(f, 𝒙, 𝒚) ⟹ p\_\trv(f, 𝒙, 𝒚).
\end{gather*}
$$

$d$ is said to be the _strongest_ decoration of $(f, 𝒙, 𝒚)$ if and only if:

$$
d = \begin{cases}
  \com & \if p\_\com(f, 𝒙, 𝒚), \\\\
  \dac & \if p\_\dac(f, 𝒙, 𝒚) ∧ ¬p\_\com(f, 𝒙, 𝒚), \\\\
  \def & \if p\_\def(f, 𝒙, 𝒚) ∧ ¬p\_\dac(f, 𝒙, 𝒚), \\\\
  \ill & \if p\_\ill(f, 𝒙, 𝒚), \\\\
  \trv & \otherwise.
 \end{cases}
$$

Let $𝒙 ∈ \IR, d ∈ \D$. A _decorated interval_ is a pair $(𝒙, d)$ of the following combinations:

| Interval $𝒙$         | Decoration $d$                     |
| -------------------- | ---------------------------------- |
| Nonempty and bounded | $\com, \dac, \def, \trv$ or $\ill$ |
| Unbounded            | $\dac, \def, \trv$ or $\ill$       |
| Empty                | $\trv$ or $\ill$                   |

We denote by $\DIR$ the set of all decorated intervals.

- (Advanced) Fundamentally, $\DIR$ is defined as the set of pairs $(𝒚, dy)$ that satisfies:

  $$
  ∃n ≥ 0, X ⊆ \R^n, f ∈ \R^X, 𝒙 ∈ \IR^n : [𝒚 ⊇ \Rge(f, 𝒙) ∧ p_{dy}(f, 𝒙, 𝒚)].
  $$

  By substituting $n = 0$, $X = ∅$, $f : ∅ → \R$ (the [empty function][emptymap]) and $𝒙 = ∅$ into the above statement, we see that for any $𝒚 ∈ \IR$, $(𝒚, \ill)$ is a decorated interval.

A decorated interval $(𝒙, d) ∈ \DIR$ is alternatively written as $𝒙\_d$, thus $[1, 2]\_\com = ([1, 2], \com)$. We also write an $n$-tuple of decorated intervals $({𝒙\_1}\_{d\_1}, …, {𝒙\_n}\_{d\_n}) ∈ \DIR^n$ as $𝒙_d$.

Let $π_I^{(n)} : \DIR^n ∋ 𝒙\_d ↦ 𝒙 ∈ \IR^n$ and $π_D^{(n)} : \DIR^n ∋ 𝒙\_d ↦ d ∈ \D^n$ be the functions that maps a decorated interval (or a tuple of them) to its interval part and decoration part, respectively. Let $π_I = π_I^{(1)}$ and $π_D = π_D^{(1)}$.

Let $n ≥ 0$, $X ⊆ \R^n$ and $f : X → \R$. A function $𝒇 : \DIR^n → \DIR$ is said to be a _decorated interval extension_ of $f$ if and only if there exists $𝒇_I : \IR^n → \IR$ such that $𝒇_I$ is an interval extension of $f$ and $𝒇_I ∘ π_I^{(n)} = π_I ∘ 𝒇$ holds, and the following also holds:

$$
∀𝒙_{dx} ∈ \DIR^n, ∃d ∈ \D : [p_d(f, 𝒙, 𝒚) ∧ dy = \min \set{d, dx_1, …, dx_n}],
$$

where $𝒚$ and $dy$ represents $π_I(𝒇(𝒙_{dx}))$ and $π_D(𝒇(𝒙_{dx}))$, respectively.

Let $𝒇$ be a decorated interval extension of $f$. The following holds:

$$
∀𝒙_d ∈ \DIR^n : [(∃i ∈ \set{1, …, n} : d_i = \ill) ⟹ π_D(𝒇(𝒙_d)) = \ill].
$$

Any interval decorated with $\ill$ is said to be _NaI_ (_Not an Interval_). A NaI is produced by an invalid construction of a (decorated) interval, and it is propagated through evaluation.

$\DIF$, the decorated version of $\IF$ and relevant properties are derived in the same manner as we did in the previous section.

### Examples

1. Let $\tfloor : \DIF → \DIF$ be the tightest, strongest-decorated interval extension of the [floor function][floor] $⌊{⋅}⌋ : \R → \R$. Then,

   $$
   \tag{a} \tfloor([\mathtt{1.25}, \mathtt{1.75}]\_\com) = [\mathtt{1}, \mathtt{1}]\_\com.
   $$

   $$
   \tag{b} \tfloor([\mathtt{0.5}, \mathtt{1.5}]\_\com) = [\mathtt{0}, \mathtt{1}]\_\def.
   $$

   $$
   \tag{c} \tfloor([\mathtt{1}, \mathtt{1.5}]\_\com) = [\mathtt{1}, \mathtt{1}]\_\dac.
   $$

   In (b), the result is decorated with $\def$ because $⌊{⋅}⌋$ is discontinuous at $0$.

   In (c), the result is decorated with $\dac$ bacause the restriction of $⌊{⋅}⌋$ to $[1, 1.5]$ is continuous, by the definition of the [subspace topology][subspace].

1. Let $\tsqrt : \DIF → \DIF$ be the tightest, strongest-decorated interval extension of $\sqrt{⋅} : [0, +∞] → \R$. Then,

   $$
   \tag{a} \tsqrt([\mathtt{0}, \mathtt{1}]\_\com) = [\mathtt{0}, \mathtt{1}]\_\com.
   $$

   $$
   \tag{b} \tsqrt([\mathtt{-1}, \mathtt{1}]\_\com) = [\mathtt{0}, \mathtt{1}]\_\trv.
   $$

   $$
   \tag{c} \tsqrt([\mathtt{-2}, \mathtt{-1}]\_\com) = ∅\_\trv.
   $$

## Notation

Some of the symbols used in this article is different from the IEEE 1788 standards. Here are the differences between them:

| This article                             | The IEEE 1788 standards                     |
| ---------------------------------------- | ------------------------------------------- |
| $\IR$                                    | $\overline{𝕀ℝ}$                             |
| $\DIR$                                   | $\overline{𝔻𝕀ℝ}$                            |
| ---                                      | $𝔽$ (as a generic number format)            |
| $\F$                                     | $\operatorname{Val}(𝔽)$                     |
| $\IF$                                    | $𝕋$ (as a generic interval type)            |
| $\DIF$                                   | $𝔻𝕋$ (as a generic decorated interval type) |
| $\Rge(f, 𝒙)$                             | $\Rge(f ∣ 𝒙)$                               |
| $p_d(f,𝒙,𝒚)$                             | $p_d(f ∣ 𝒙)$                                |
| The strongest decoration for $(f, 𝒙, 𝒚)$ | $\operatorname{Dec}(f ∣ 𝒙)$                 |

[bounded]: https://proofwiki.org/wiki/Definition:Bounded_Ordered_Set
[closed]: https://proofwiki.org/wiki/Definition:Closed_Set
[continuous]: https://proofwiki.org/wiki/Definition:Continuous_Mapping
[convex]: https://proofwiki.org/wiki/Definition:Convex_Set_(Order_Theory)
[emptymap]: https://proofwiki.org/wiki/Definition:Empty_Mapping
[emptyset]: https://proofwiki.org/wiki/Definition:Empty_Set
[floor]: https://proofwiki.org/wiki/Definition:Floor_Function
[inf]: https://proofwiki.org/wiki/Definition:Infimum_of_Set
[restriction]: https://proofwiki.org/wiki/Definition:Restriction/Mapping
[reals]: https://proofwiki.org/wiki/Definition:Real_Number/Real_Number_Line
[subset]: https://proofwiki.org/wiki/Definition:Subset
[subspace]: https://proofwiki.org/wiki/Definition:Topological_Subspace
[sup]: https://proofwiki.org/wiki/Definition:Supremum_of_Set
[toset]: https://proofwiki.org/wiki/Definition:Totally_Ordered_Set
[xreals]: https://proofwiki.org/wiki/Definition:Extended_Real_Number_Line
