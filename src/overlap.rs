use crate::interval::*;

/// The overlapping state between intervals, returned by [`Interval::overlap`].
///
/// # Quick Reference
///
/// ```text
///                                  self
///      ┌──────────────────────────────────────────────────────────┐
///      │                                                          │
///      │             •——————•                       •             │
///      │             :      :                       :             │
///      │             :      :  •————• B             :  •————• B   │
///      │             :      •————• M                :             │
///      │             :   •————• O                   :             │
///      │             •————————• S                   •————• S      │
///      │           •——————————• Cb               •—————• Cb       │
///      │           •————————• F                •————• F           │
///      │             •——————• E                     • E           │
///  rhs │             : •————• Fb                    :             │
///      │             :      • Fb                    :             │
///      │           C : •——• :                       :             │
///      │          Sb •————• :                       :             │
///      │          Sb •      :                       :             │
///      │        Ob •————•   :                       :             │
///      │     Mb •————•      :                       :             │
///      │   A •————•  :      :             A •————•  :             │
///      │             :      :                       :             │
///      │             •——————•                       •             │
///      │                                                          │
///      └──────────────────────────────────────────────────────────┘
/// ```
///
/// ```text
///                                   rhs
///      ┌──────────────────────────────────────────────────────────┐
///      │                                                          │
///      │             •——————•                       •             │
///      │             :      :                       :             │
///      │   B •————•  :      :             B •————•  :             │
///      │      M •————•      :                       :             │
///      │         O •————•   :                       :             │
///      │           S •————• :                       :             │
///      │           S •      :                       :             │
///      │          Cb : •——• :                       :             │
///      │             : •————• F                     :             │
/// self │             :      • F                     :             │
///      │             •——————• E                     • E           │
///      │           •————————• Fb               •————• Fb          │
///      │           •——————————• C                •—————• C        │
///      │             •————————• Sb                  •————• Sb     │
///      │             :   •————• Ob                  :             │
///      │             :      •————• Mb               :             │
///      │             :      :  •————• A             :  •————• A   │
///      │             :      :                       :             │
///      │             •——————•                       •             │
///      │                                                          │
///      └──────────────────────────────────────────────────────────┘
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Overlap {
    /// Both `self` and `rhs` are empty.
    ///
    /// Equivalently, $\self = \rhs = ∅$.
    BothEmpty,

    /// `self` is empty while `rhs` is not.
    ///
    /// Equivalently, $\self = ∅ ∧ \rhs ≠ ∅$.
    FirstEmpty,

    /// `rhs` is empty while `self` is not.
    ///
    /// Equivalently, $\self ≠ ∅ ∧ \rhs = ∅$.
    SecondEmpty,

    /// Both $\self = \[a, b\]$ and $rhs = \[c, d\]$ are nonempty and $b < c$.
    ///
    /// Equivalently,
    ///
    /// $$
    /// \self ≠ ∅ ∧ \rhs ≠ ∅ ∧ ∀x ∈ \self, ∀y ∈ \rhs : x < y.
    /// $$
    ///
    /// ```text
    ///        a      b
    /// self:  •——————•
    ///  rhs:             •——————•
    ///                   c      d
    /// ```
    ///
    /// Inverse: [`Overlap::After`].
    Before,

    /// Both $\self = \[a, b\]$ and $rhs = \[c, d\]$ are nonempty and $a < b ∧ b = c ∧ c < d$.
    ///
    /// Equivalently,
    ///
    ///
    /// $$
    /// \begin{align*}
    ///  \self ≠ ∅ ∧ \rhs ≠ ∅
    ///   &∧ ∀x ∈ \self, ∀y ∈ \rhs : x ≤ y \\\\
    ///   &∧ ∃x ∈ \self, ∀y ∈ \rhs : x < y \\\\
    ///   &∧ ∃x ∈ \self, ∃y ∈ \rhs : x = y.
    /// \end{align*}
    /// $$
    ///
    /// ```text
    ///        a      b
    /// self:  •——————•
    ///  rhs:         •——————•
    ///               c      d
    /// ```
    ///
    /// Inverse: [`Overlap::MetBy`].
    Meets,

    /// Both $\self = \[a, b\]$ and $rhs = \[c, d\]$ are nonempty and $a < c ∧ c < b ∧ b < d$.
    ///
    /// Equivalently,
    ///
    /// $$
    /// \begin{align*}
    ///  \self ≠ ∅ ∧ \rhs ≠ ∅
    ///   &∧ ∃x ∈ \self, ∀y ∈ \rhs : x < y \\\\
    ///   &∧ ∃y ∈ \rhs, ∀x ∈ \self : x < y \\\\
    ///   &∧ ∃x ∈ \self, ∃y ∈ \rhs : y < x.
    /// \end{align*}
    /// $$
    ///
    /// ```text
    ///        a      b
    /// self:  •——————•
    ///  rhs:      •——————•
    ///            c      d
    /// ```
    ///
    /// Inverse: [`Overlap::OverlappedBy`].
    Overlaps,

    /// Both $\self = \[a, b\]$ and $rhs = \[c, d\]$ are nonempty and $a = c ∧ b < d$.
    ///
    /// Equivalently,
    ///
    /// $$
    /// \begin{align*}
    ///  \self ≠ ∅ ∧ \rhs ≠ ∅
    ///   &∧ ∀y ∈ \rhs, ∃x ∈ \self : x ≤ y \\\\
    ///   &∧ ∀x ∈ \self, ∃y ∈ \rhs : y ≤ x \\\\
    ///   &∧ ∃y ∈ \rhs, ∀x ∈ \self : x < y.
    /// \end{align*}
    /// $$
    ///
    /// ```text
    ///        a    b        :          a=b
    /// self:  •————•        :    self:  •
    ///  rhs:  •————————•    :     rhs:  •——————•
    ///        c        d    :           c      d
    /// ```
    ///
    /// Inverse: [`Overlap::StartedBy`].
    Starts,

    /// Both $\self = \[a, b\]$ and $rhs = \[c, d\]$ are nonempty and $c < a ∧ b < d$.
    ///
    /// Equivalently,
    ///
    /// $$
    /// \begin{align*}
    ///  \self ≠ ∅ ∧ \rhs ≠ ∅
    ///   &∧ ∃y ∈ \rhs, ∀x ∈ \self : y < x \\\\
    ///   &∧ ∃y ∈ \rhs, ∀x ∈ \self : x < y.
    /// \end{align*}
    /// $$
    ///
    /// ```text
    ///          a    b
    /// self:    •————•
    ///  rhs:  •————————•
    ///        c        d
    /// ```
    ///
    /// Inverse: [`Overlap::Contains`].
    ContainedBy,

    /// Both $\self = \[a, b\]$ and $rhs = \[c, d\]$ are nonempty and $c < a ∧ b = d$.
    ///
    /// Equivalently,
    ///
    /// $$
    /// \begin{align*}
    ///  \self ≠ ∅ ∧ \rhs ≠ ∅
    ///   &∧ ∃y ∈ \rhs, ∀x ∈ \self : y < x \\\\
    ///   &∧ ∀y ∈ \rhs, ∃x ∈ \self : y ≤ x \\\\
    ///   &∧ ∀x ∈ \self, ∃y ∈ \rhs : x ≤ y.
    /// \end{align*}
    /// $$
    ///
    /// ```text
    ///            a    b    :                 a=b
    /// self:      •————•    :    self:         •
    ///  rhs:  •————————•    :     rhs:  •——————•
    ///        c        d    :           c      d
    /// ```
    ///
    /// Inverse: [`Overlap::FinishedBy`].
    Finishes,

    /// Both $\self = \[a, b\]$ and $rhs = \[c, d\]$ are nonempty and $a = c ∧ b = d$.
    ///
    /// Equivalently,
    ///
    /// $$
    /// \begin{align*}
    ///  \self ≠ ∅ ∧ \rhs ≠ ∅
    ///   &∧ ∀x ∈ \self, ∃y ∈ \rhs : x = y \\\\
    ///   &∧ ∀y ∈ \rhs, ∃x ∈ \self : y = x.
    /// \end{align*}
    /// $$
    ///
    /// ```text
    ///        a      b    :          a=b
    /// self:  •——————•    :    self:  •
    ///  rhs:  •——————•    :     rhs:  •
    ///        c      d    :          c=d
    /// ```
    Equals,

    /// Both $\self = \[a, b\]$ and $rhs = \[c, d\]$ are nonempty and $a < c ∧ b = d$.
    ///
    /// Equivalently,
    ///
    /// $$
    /// \begin{align*}
    ///  \self ≠ ∅ ∧ \rhs ≠ ∅
    ///   &∧ ∃x ∈ \self, ∀y ∈ \rhs : x < y \\\\
    ///   &∧ ∀x ∈ \self, ∃y ∈ \rhs : x ≤ y \\\\
    ///   &∧ ∀y ∈ \rhs, ∃x ∈ \self : y ≤ x.
    /// \end{align*}
    /// $$
    ///
    /// ```text
    ///        a        b    :           a      b
    /// self:  •————————•    :    self:  •——————•
    ///  rhs:      •————•    :     rhs:         •
    ///            c    d    :                 c=d
    /// ```
    ///
    /// Inverse: [`Overlap::Finishes`].
    FinishedBy,

    /// Both $\self = \[a, b\]$ and $rhs = \[c, d\]$ are nonempty and $a < c ∧ d < b$.
    ///
    /// Equivalently,
    ///
    /// $$
    /// \begin{align*}
    ///  \self ≠ ∅ ∧ \rhs ≠ ∅
    ///   &∧ ∃x ∈ \self, ∀y ∈ \rhs : x < y \\\\
    ///   &∧ ∃x ∈ \self, ∀y ∈ \rhs : y < x.
    /// \end{align*}
    /// $$
    ///
    /// ```text
    ///        a        b
    /// self:  •————————•
    ///  rhs:    •————•
    ///          c    d
    /// ```
    ///
    /// Inverse: [`Overlap::ContainedBy`].
    Contains,

    /// Both $\self = \[a, b\]$ and $rhs = \[c, d\]$ are nonempty and $a = c ∧ d < b$.
    ///
    /// Equivalently,
    ///
    /// $$
    /// \begin{align*}
    ///  \self ≠ ∅ ∧ \rhs ≠ ∅
    ///   &∧ ∀x ∈ \self, ∃y ∈ \rhs : y ≤ x \\\\
    ///   &∧ ∀y ∈ \rhs, ∃x ∈ \self : x ≤ y \\\\
    ///   &∧ ∃x ∈ \self, ∀y ∈ \rhs : y < x.
    /// \end{align*}
    /// $$
    ///
    /// ```text
    ///        a        b    :           a      b
    /// self:  •————————•    :    self:  •——————•
    ///  rhs:  •————•        :     rhs:  •
    ///        c    d        :          c=d
    /// ```
    ///
    /// Inverse: [`Overlap::Starts`].
    StartedBy,

    /// Both $\self = \[a, b\]$ and $rhs = \[c, d\]$ are nonempty and $c < a ∧ a < d ∧ d < b$.
    ///
    /// Equivalently,
    ///
    /// $$
    /// \begin{align*}
    ///  \self ≠ ∅ ∧ \rhs ≠ ∅
    ///   &∧ ∃y ∈ \rhs, ∀x ∈ \self : y < x \\\\
    ///   &∧ ∃x ∈ \self, ∀y ∈ \rhs : y < x \\\\
    ///   &∧ ∃y ∈ \rhs, ∃x ∈ \self : x < y.
    /// \end{align*}
    /// $$
    ///
    /// ```text
    ///            a      b
    /// self:      •——————•
    ///  rhs:  •——————•
    ///        c      d
    /// ```
    ///
    /// Inverse: [`Overlap::Overlaps`].
    OverlappedBy,

    /// Both $\self = \[a, b\]$ and $rhs = \[c, d\]$ are nonempty and $c < d ∧ a = d ∧ a < b$.
    ///
    /// Equivalently,
    ///
    /// $$
    /// \begin{align*}
    ///  \self ≠ ∅ ∧ \rhs ≠ ∅
    ///   &∧ ∀y ∈ \rhs, ∀x ∈ \self : y ≤ x \\\\
    ///   &∧ ∃y ∈ \rhs, ∃x ∈ \self : y = x \\\\
    ///   &∧ ∃y ∈ \rhs, ∀x ∈ \self : y < x.
    /// \end{align*}
    /// $$
    ///
    /// ```text
    ///               a      b
    /// self:         •——————•
    ///  rhs:  •——————•
    ///        c      d
    /// ```
    ///
    /// Inverse: [`Overlap::Meets`].
    MetBy,

    /// Both $\self = \[a, b\]$ and $rhs = \[c, d\]$ are nonempty and $d < a$.
    ///
    /// Equivalently,
    ///
    /// $$
    /// \self ≠ ∅ ∧ \rhs ≠ ∅ ∧ ∀y ∈ \rhs, ∀x ∈ \self : y < x.
    /// $$
    ///
    /// ```text
    ///                   a      b
    /// self:             •——————•
    ///  rhs:  •——————•
    ///        c      d
    /// ```
    ///
    /// Inverse: [`Overlap::Before`].
    After,
}

impl Interval {
    /// Returns the overlapping state of `self` and `rhs`.
    /// See [`Overlap`] for the possible states to be returned.
    pub fn overlap(self, rhs: Self) -> Overlap {
        use Overlap::*;

        if self.is_empty() {
            if rhs.is_empty() {
                return BothEmpty;
            } else {
                return FirstEmpty;
            }
        }
        if rhs.is_empty() {
            return SecondEmpty;
        }

        let a = self.inf_raw();
        let b = self.sup_raw();
        let c = rhs.inf_raw();
        let d = rhs.sup_raw();

        //     |  aRc  |  aRd  |  bRc  |  bRd
        //     | < = > | < = > | < = > | < = >
        // ----+-------+-------+-------+-------
        //   B | x     | x     | x     | x
        //   M | x     | x     |   x   | x
        //   O | x     | x     |     x | x
        //   S |   x   | x     |   ? ? | x
        //  Cb |     x | x     |     x | x
        //   F |     x | ? ?   |     x |   x
        //   E |   x   | ? ?   |   ? ? |   x
        //  Fb | x     | x     |   ? ? |   x
        //   C | x     | x     |     x |     x
        //  Sb |   x   | ? ?   |     x |     x
        //  Ob |     x | x     |     x |     x
        //  Mb |     x |   x   |     x |     x
        //   A |     x |     x |     x |     x

        #[allow(clippy::collapsible_else_if, clippy::collapsible_if)]
        if b < d {
            if a < c {
                if b < c {
                    Before
                } else if b == c {
                    Meets
                } else {
                    Overlaps
                }
            } else {
                if a == c {
                    Starts
                } else {
                    ContainedBy
                }
            }
        } else if b == d {
            if a > c {
                Finishes
            } else if a == c {
                Equals
            } else {
                FinishedBy
            }
        } else {
            if a <= c {
                if a < c {
                    Contains
                } else {
                    StartedBy
                }
            } else {
                if a < d {
                    OverlappedBy
                } else if a == d {
                    MetBy
                } else {
                    After
                }
            }
        }
    }
}

impl DecInterval {
    /// Applies [`Interval::overlap`] to the interval parts of `self` and `rhs` and returns the result.
    ///
    /// [`None`] is returned if `self` or `rhs` is NaI.
    pub fn overlap(self, rhs: Self) -> Option<Overlap> {
        if self.is_nai() || rhs.is_nai() {
            return None;
        }

        Some(self.x.overlap(rhs.x))
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use DecInterval as DI;

    #[test]
    fn nai() {
        assert_eq!(DI::NAI.overlap(DI::PI), None);
        assert_eq!(DI::PI.overlap(DI::NAI), None);
    }

    #[test]
    fn pattern() {
        use Overlap::*;
        // Pattern matching results in a more efficient code than using bitmasks as the compiler
        // can eliminate unnecessary comparisons.
        assert!(matches!(
            const_interval!(1.0, 2.0).overlap(const_interval!(3.0, 4.0)),
            BothEmpty | FirstEmpty | SecondEmpty | Before | After
        ))
    }
}
