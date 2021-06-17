# Introduction to interval arithmetic

This is an introduction to interval arithmetic (IA) provided by the crate. There are other variations on IA, which are not mentioned in this documentation. The variation here is called the _set-based_ flavor in the IEEE 1788 standard.

## Intervals

An interval is a set of the all real numbers within a range such as $-1 ≤ x ≤ 2, -1 ≤ x$ and $x ≤ 2$. The [empty set][emptyset] ($∅$) as well as the [set of all real numbers][reals] ($\R$) are also treated as intervals. Here is a summary of the notations and definitions of intervals:

| Interval Notation  | Definition                 | [Bounded][bounded] | Notes              |
| ------------------ | -------------------------- | :----------------: | ------------------ |
| $∅$                | $∅$ (the empty set)        |        Yes         |                    |
| $[a, b]$           | $\\{x ∈ \R ∣ a ≤ x ≤ b\\}$ |        Yes         | $a, b ∈ \R, a ≤ b$ |
| $[a, +∞]$          | $\\{x ∈ \R ∣ a ≤ x\\}$     |         No         | $a ∈ \R$           |
| $[-∞, b]$          | $\\{x ∈ \R ∣ x ≤ b\\}$     |         No         | $b ∈ \R$           |
| $\R$ or $[-∞, +∞]$ | $\R$ (the whole real line) |         No         |                    |

To put the definitions of intervals into a simpler form, let us introduce the [extended real numbers][xreals] $\XR$:

$$
\XR = \R ∪ \\{+∞, -∞\\},
$$

which is a [totally ordered set][toset] extending $\XR$ with the following ordering rules:

$$
∀x ∈ \R ∪ \\{-∞\\} : x < +∞,\qquad ∀x ∈ \R ∪ \\{+∞\\} : -∞ < x.
$$

Now we can define any nonempty interval $[a, b]$ as $\\{x ∈ \R ∣ a ≤ x ≤ b\\}$, where $a, b ∈ \XR ∧ a ≤ b ∧ a < +∞ ∧ b > -∞$. $a$ is called the _lower bound_ and $b$ is called the _upper bound_ of the interval. Fundamentally, the lower and upper bounds of an interval $𝒙$ are defined as $\inf 𝒙$ and $\sup 𝒙$ (the [infimum][inf] and the [supremum][sup] of $𝒙$ in $\XR$), respectively. Therefore, the lower (resp. upper) bound of $∅$ is $+∞$ (resp. $-∞$).

Note that while the bounds of an interval are members of $\XR$, the interval itself is a [subset][subset] of $\R$. Therefore, neither $-∞$ nor $+∞$ can be a member of an interval.

We denote by $\IR$ the set of all intervals:

$$
\IR = \\{∅\\} ∪ \\{[a, b] ∣ a, b ∈ \XR ∧ a ≤ b ∧ a < +∞ ∧ b > -∞\\}.
$$

An interval is denoted by a bold letter such as $𝒙$ or $𝒚$. Furthermore, an $n$-tuple of intervals $(𝒙\_1, …, 𝒙\_n) ∈ \IR^n$ is also denoted by $𝒙$.

Every interval is a [closed][closed] subset of $\R$.

## Interval Extensions of Functions

Let $n ∈ \N = \\{0, 1, 2, …\\}$ and $X ⊆ \R^n$. Let $f : X → \R$ be a real-valued function. A function $𝒇 : \IR^n → \IR$ is said to be an _interval extension_ of $f$ if and only if:

$$
∀𝒙 ∈ \IR^n : 𝒇(𝒙) ⊇ f\[𝒙\],
$$

where $f\[𝒙\] = \\{f(x) ∣ x ∈ 𝒙 ∩ X\\}$ is the [image][image] of $𝒙$ under $f$.

The _natural interval extension_ of $f$ is the interval extension that maps an interval $𝒙$ to the tightest interval that encloses $f\[𝒙\]$:

$$
\begin{align}
 𝒇(𝒙) &= \operatorname{min}_⊆\\{𝒚 ∈ \IR ∣ 𝒚 ⊇ f\[𝒙\]\\} \\\\
  &= \begin{cases}
    ∅ & \if f\[𝒙\] = ∅, \\\\
    [\inf f\[𝒙\], \sup f\[𝒙\]] & \otherwise.
   \end{cases}
\end{align}
$$

Let $n > 0$. Let $𝒇$ be the natural interval extension of $f$. The following holds:

$$
∀𝒙 ∈ \IR^n : ((∃i ∈ \\{1, …, n\\} : 𝒙\_i = ∅) ⟹ 𝒇(𝒙) = ∅).
$$

### Examples

Here are some examples of the natural interval extension. The cases where any of the arguments is $∅$ are omitted. In the examples, we also define arithmetic operations on extended real numbers involving $±∞$.

1. $\surd : [0, ∞] → ℝ$ is extended as

   $$
   \sqrt{[a, b]} = \begin{cases}
     ∅ & \if b < 0, \\\\
     [0, \sqrt{b}] & \if a ≤ 0 ≤ b, \\\\
     [\sqrt{a}, \sqrt{b}] & \otherwise,
    \end{cases}
   $$

   where $\sqrt{+∞} = +∞$.

2. Addition and subtraction ($+, - : ℝ × ℝ → ℝ$) are extended as

   $$
   \begin{align}
    [a, b] + [c, d] &= [a + c, b + d], \\\\
    [a, b] - [c, d] &= [a - d, b - c],
   \end{align}
   $$

   where

   $$
   \begin{gathered}
    ∀x ∈ \R ∪ \\{+∞\\} : x + (+∞) = +∞ + x = +∞, \\\\
    ∀x ∈ \R ∪ \\{-∞\\} : x + (-∞) = -∞ + x = -∞, \\\\
    -(±∞) = ∓∞.
   \end{gathered}
   $$

3. Multiplication ($× : ℝ × ℝ → ℝ$) is extended as

   $[a, b] × [c, d] =$

   |             |  $d ≤ 0$   |              $c < 0 < d$               |  $0 ≤ c$   |
   | :---------: | :--------: | :------------------------------------: | :--------: |
   |   $b ≤ 0$   | $[bd, ac]$ |               $[ad, ac]$               | $[ad, bc]$ |
   | $a < 0 < b$ | $[bc, ac]$ | $[\min\\{ad, bc\\}, \max\\{ac, bd\\}]$ | $[ad, bd]$ |
   |   $0 ≤ a$   | $[bc, ad]$ |               $[bc, bd]$               | $[ac, bd]$ |

   where

   $$
   ∀x ∈ \XR{∖}\\{0\\} : x × (±∞) = ±∞ × x = \begin{cases}
     ±∞ & \if x > 0, \\\\
     ∓∞ & \if x < 0.
    \end{cases}
   $$

4. Division ($/ : ℝ × ℝ{∖}\\{0\\} → ℝ$) is extended as

   $[a, b]/[c, d] =$

   |             |   $d < 0$    | $c < 0 ∧ d = 0$ | $c = d = 0$ | $c < 0 < d$ | $c = 0 ∧ 0 < d$ |   $0 < c$    |
   | :---------: | :----------: | :-------------: | :---------: | :---------: | :-------------: | :----------: |
   |   $b ≤ 0$   | $[b/c, a/d]$ |   $[b/c, +∞]$   |     $∅$     |    $\R$     |   $[-∞, b/d]$   | $[a/c, b/d]$ |
   | $a = 0 = b$ |   $[0, 0]$   |    $[0, 0]$     |     $∅$     |  $[0, 0]$   |    $[0, 0]$     |   $[0, 0]$   |
   | $a < 0 < b$ | $[b/d, a/d]$ |      $\R$       |     $∅$     |    $\R$     |      $\R$       | $[a/c, b/c]$ |
   |   $0 ≤ a$   | $[b/d, a/c]$ |   $[-∞, a/c]$   |     $∅$     |    $\R$     |   $[a/d, +∞]$   | $[a/d, b/c]$ |

   where

   $$
   \begin{gathered}
    ∀x ∈ \R : x/(±∞) = 0, \\\\
    ∀x ∈ \R{∖}\\{0\\} : ±∞/x = \begin{cases}
      ±∞ & \if x > 0, \\\\
      ∓∞ & \if x < 0.
     \end{cases}
   \end{gathered}
   $$

5. Let $c ∈ \R$. Let $f : \R^0 → \R$ be a function that maps $∅$ to $c$ (note that $S^0 = \\{∅\\}$ for any set $S$). The natural interval extension of $f$ is a function $𝒇 : \IR^0 → \IR$ that maps $∅$ to $[c, c]$.

   For this reason, we define the natural interval extension of an real constant $c$ as $[c, c]$.

## $\IF$-Interval Extensions of Functions

Floating-point arithmetic (FA) is an approximation of the extended real numbers designed to be efficiently implemented at hardware level. The crate provides an efficient implementation of IA by using `f64` numbers to represent and compute with intervals. See the IEEE 754 standards for the details of FA.

We denote by $\F ⊆ \XR$ the set of all normal and subnormal `f64` numbers, zero, $+∞$ and $-∞$.

Let $\RD$ and $\RU : \XR → \F$ be the functions that maps an extended real number $x$ to the greatest $\F$ number $≤ x$ and the least $\F$ number $≥ x$ respectively:

$$
\begin{align}
 \RD x &= \max\\{y ∈ \F ∣ y ≤ x\\}, \\\\
 \RU x &= \min\\{y ∈ \F ∣ x ≤ y\\},
\end{align}
$$

and $\RDU : \IR → \IF$ be the function that maps an interval $𝒙$ to the tightest $\IF$ interval that encloses $𝒙$:

$$
\begin{align}
 \RDU 𝒙 &= \operatorname{min}_⊆\\{𝒚 ∈ \IF ∣ 𝒚 ⊇ 𝒙\\} \\\\
  &= \begin{cases}
    ∅ & \if 𝒙 = ∅, \\\\
    [\RD a, \RU b] & \otherwise, 𝒙 = [a, b].
   \end{cases}
\end{align}
$$

Let $\nextDown$ and $\nextUp : \F → \F$ be the functions defined as follows:

$$
\begin{align}
 \nextDown(x) &= \begin{cases}
   -∞ & \if x = -∞, \\\\
   \max\\{y ∈ \F ∣ y < x\\} & \otherwise,
  \end{cases} \\\\
 \nextUp(x) &= \begin{cases}
   +∞ & \if x = +∞, \\\\
   \min\\{y ∈ \F ∣ x < y\\} & \otherwise,
  \end{cases}
\end{align}
$$

and $\nextOut : \IF → \IF$ be the function defined as follows:

$$
\nextOut(𝒙) = \begin{cases}
  ∅ & \if 𝒙 = ∅, \\\\
  [\nextDown(a), \nextUp(b)] & \otherwise, 𝒙 = [a, b].
 \end{cases}
$$

We denote by $\IF ⊆ \IR$ the set of intervals whose bounds are $\F$ numbers:

$$
\IF = \\{∅\\} ∪ \\{[a, b] ∣ a, b ∈ \F ∧ a ≤ b ∧ a < +∞ ∧ b > -∞\\}.
$$

Let $n ∈ \N$ and $X ⊆ \R^n$. Let $f : X → ℝ$ be a real-valued function. A function $𝒇\_\IF : \IF^n → \IF$ is said to be an _$\IF$-interval extension_ of $f$ if and only if:

$$
∀𝒙 ∈ \IF^n : 𝒇\_\IF(𝒙) ⊇ f\[𝒙\].
$$

Let $𝒇\_\IF$ be an $\IF$-interval extension of $f$. Then $𝒇\_\IF$ is also an interval extension of $f$. The _tightness_ of $𝒇\_\IF$ is said to be _tightest_ if and only if its values are the tightest $\IF$ intervals:

$$
𝒇\_\IF(𝒙) = \RDU 𝒇(𝒙),
$$

where $𝒇 : \IR → \IR$ is the natural interval extension of $f$. The tightness of $𝒇\_\IF$ is said to be _accurate_ if and only if its values are slightly wider than in the tightest case:

$$
𝒇\_\IF(𝒙) ⊇ \nextOut(\RDU 𝒇(\nextOut(𝒙))).
$$

The tightness of $𝒇\_\IF$ is said to be _valid_ if and only if:

$$
𝒇\_\IF(𝒙) ⊇ 𝒇(𝒙),
$$

which is always true.

For example, the tightest $\IF$-interval extension of $π = 3.14159265358979323…$ is

$$
𝛑\_\IF ⊆ [3.14159265358979311, 3.14159265358979357].
$$

Note that $⊆$ is used instead of $=$ because $\F$ numbers are often too long to be written in decimal.

## The Decoration System

The decoration system gives us some additional information on the underlying function of an interval extension being evaluated, such as whether it is defined or [continuous][cont] on the input interval.

We denote by $\D$ the set of decorations:

$$
\D = \\{\com, \dac, \def, \trv, \ill\\}.
$$

They are abbreviations of **com**mon, **d**efined **a**nd **c**ontinuous, **def**ined, **tri**vial and **ill**-formed, respectively. $\D$ is a totally ordered set with the following ordering rules:

$$
\com > \dac > \def > \trv > \ill.
$$

Let $n ∈ \N$ and $X ⊆ \R^n$. Let $f : X → \R$ be a real-valued function. Let $𝒙 ∈ \XR^n, 𝒚 ∈ \XR$. We define the following [predicates](https://proofwiki.org/wiki/Definition:Propositional_Function):

$$
\begin{align}
p\_\com(f, 𝒙, 𝒚) &:⟺ ∅ ≠ 𝒙 ⊆ X ∧ (f \text{ is continuous on } 𝒙) ∧ (\text{$𝒙$ and $𝒚$ are bounded}), \\\\
p\_\dac(f, 𝒙, 𝒚) &:⟺ ∅ ≠ 𝒙 ⊆ X ∧ (f{↾\_𝒙} \text{ is continuous}), \\\\
p\_\def(f, 𝒙, 𝒚) &:⟺ ∅ ≠ 𝒙 ⊆ X, \\\\
p\_\trv(f, 𝒙, 𝒚) &:⟺ (\text{always true}), \\\\
p\_\ill(f, 𝒙, 𝒚) &:⟺ X = ∅,
\end{align}
$$

where $f{↾\_𝒙}$ is the [restriction] of $f$ to $𝒙$. The following implications hold:

$$
\begin{align}
p\_\com(f, 𝒙, 𝒚) ⟹ p\_\dac(f, 𝒙, 𝒚) &⟹ p\_\def(f, 𝒙, 𝒚) ⟹ p\_\trv(f, 𝒙, 𝒚), \\\\
p\_\ill(f, 𝒙, 𝒚) &⟹ p\_\trv(f, 𝒙, 𝒚).
\end{align}
$$

Let $𝒙 ∈ \IR, d ∈ \D$. A _decorated interval_ is a pair $(𝒙, d)$ of the following combinations:

| Interval $𝒙$         | Decoration $d$               |
| -------------------- | ---------------------------- |
| Nonempty and bounded | $\com, \dac, \def$ or $\trv$ |
| Unbounded            | $\dac, \def$ or $\trv$       |
| Empty                | $\trv$                       |
| Any                  | $\ill$                       |

We denote by $\DIR$ the set of all decorated intervals.

- (Advanced) Fundamentally, a pair $(𝒚, dy)$ is said to be a decorated interval (member of $\DIR$) if and only if:

  $$
  ∃n ∈ \N, X ⊆ \R^n, f ∈ \R^X, 𝒙 ∈ \IR^n : (𝒚 ⊇ f\[𝒙\] ∧ p_{dy}(f, 𝒙, 𝒚)).
  $$

  Let $f : ∅ → \R$ be the [empty function][emptymap]. By substituting $n = 0, X = ∅, f$ and $𝒙 = ∅$ into the above statement, one can show that for any $𝒚 ∈ \IR$, $(𝒚, \ill)$ is a decorated interval.

A decorated interval $(𝒙, d) ∈ \DIR$ is also written as $𝒙\_d$, thus $[1, 2]\_\com = ([1, 2], \com)$, for example. We also write an $n$-tuple of decorated intervals $({𝒙\_1}\_{d\_1}, …, {𝒙\_n}\_{d\_n}) ∈ \DIR^n$ as $𝒙_d$.

A function $𝒇 : \DIR^n → \DIR$ is said to be a _decorated interval extension_ of $f$ if and only if:

$$
∀𝒙\_{dx} ∈ \DIR^n : (𝒚 ⊇ f\[𝒙\] ∧ ∃d ∈ \D : (p\_d(f, 𝒙, 𝒚) ∧ dy = \min\\{d, dx\_1, …, dx\_n\\})),
$$

where $𝒙 = (𝒙\_1, …, 𝒙\_n)$ and $𝒚_{dy} = 𝒇(𝒙\_{dx})$.

Let $n > 0$. Let $𝒇$ be a decorated interval extension of $f$. The following holds:

$$
∀𝒙_{dx} ∈ \DIR^n : ((∃i ∈ \\{1, …, n\\} : dx_i = \ill) ⟹ dy = \ill),
$$

where $𝒚_{dy} = 𝒇(𝒙\_{dx})$.

Any interval decorated with $\ill$ is called _NaI_ (_Not an Interval_). NaI is produced by an invalid construction of a (decorated) interval, and it is propagated through calculations.

In all functions in the crate, unless otherwise mentioned, $d$ in the above statement is chosen to be the _strongest decoration_ for $(f, 𝒙, 𝒚)$:

$$
d = \begin{cases}
  \com & \if p\_\com(f, 𝒙, 𝒚), \\\\
  \dac & \if p\_\dac(f, 𝒙, 𝒚) ∧ ¬p\_\com(f, 𝒙, 𝒚), \\\\
  \def & \if p\_\def(f, 𝒙, 𝒚) ∧ ¬p\_\dac(f, 𝒙, 𝒚), \\\\
  \ill & \if p\_\ill(f, 𝒙, 𝒚), \\\\
  \trv & \otherwise.
 \end{cases}
$$

### Examples

1. Let $⌊{⋅}⌋ : \DIR → \DIR$ be the decorated (natural) interval extension of the floor function $⌊{⋅}⌋ : \R → \R$.

   $$
   ⌊[-1/2, 1/2]\_\com⌋ = [-1, 0]_\def.
   $$

   In this case, the result is decorated with $\def$ because the floor function is discontinuous at $0$.

   $$
   ⌊[0, 1/2]\_\com⌋ = [0, 0]_\dac.
   $$

   In this case, the result is decorated with $\dac$ bacause the restriction of the floor function to $[0, 1/2]$ is continuous, by the definition of the [subspace topology][subtopo].

## Notation

Some of the symbols used in this document is different from the standard. Here are the differences between them:

| This Document                            | IEEE 1788 Standard         |
| ---------------------------------------- | -------------------------- |
| $\IR$                                    | $\overline{𝕀ℝ}$            |
| $\DIR$                                   | $\overline{𝔻𝕀ℝ}$           |
| $\F$                                     | $\operatorname{Val}(𝔽)$    |
| $\IF$                                    | $𝕋$                        |
| $\DIF$                                   | $𝔻𝕋$                       |
| $f\[𝒙\]$                                 | $\operatorname{Rge}(f\|𝒙)$ |
| $p_d(f,𝒙,𝒚)$                             | $p_d(f\|𝒙)$                |
| The strongest decoration for $(f, 𝒙, 𝒚)$ | $\operatorname{Dec}(f\|𝒙)$ |

[bounded]: https://proofwiki.org/wiki/Definition:Bounded_Ordered_Set
[closed]: https://proofwiki.org/wiki/Definition:Closed_Set
[cont]: https://proofwiki.org/wiki/Definition:Continuous_Mapping
[emptymap]: https://proofwiki.org/wiki/Definition:Empty_Mapping
[emptyset]: https://proofwiki.org/wiki/Definition:Empty_Set
[image]: https://proofwiki.org/wiki/Definition:Image_(Set_Theory)/Mapping/Subset
[inf]: https://proofwiki.org/wiki/Definition:Infimum_of_Set
[restriction]: https://proofwiki.org/wiki/Definition:Restriction/Mapping
[reals]: https://proofwiki.org/wiki/Definition:Real_Number/Real_Number_Line
[subset]: https://proofwiki.org/wiki/Definition:Subset
[subtopo]: https://proofwiki.org/wiki/Definition:Topological_Subspace
[sup]: https://proofwiki.org/wiki/Definition:Supremum_of_Set
[toset]: https://proofwiki.org/wiki/Definition:Totally_Ordered_Set
[xreals]: https://proofwiki.org/wiki/Definition:Extended_Real_Number_Line
