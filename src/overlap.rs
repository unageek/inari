use crate::interval::*;

/// States returned by [`Interval::overlap`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OverlappingState {
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

    /// ```text
    ///        a      b
    /// self:  •——————•
    ///  rhs:             •——————•
    ///                   c      d
    /// ```
    ///
    /// Both `self` and `rhs` are nonempty and $b < c$.
    ///
    /// Equivalently, $\self ≠ ∅ ∧ \rhs ≠ ∅ ∧ ∀x ∈ \self, ∀y ∈ \rhs : x < y$.
    Before,

    /// ```text
    ///        a      b
    /// self:  •——————•
    ///  rhs:         •——————•
    ///               c      d
    /// ```
    ///
    /// Both `self` and `rhs` are nonempty and $a < b ∧ b = c ∧ c < d$.
    ///
    /// Equivalently, $\self ≠ ∅ ∧ \rhs ≠ ∅ ∧ (∀x ∈ \self, ∀y ∈ \rhs : x ≤ y)
    /// ∧ (∃x ∈ \self, ∀y ∈ \rhs : x < y) ∧ (∃x ∈ \self, ∃y ∈ \rhs : x = y)$.
    Meets,

    /// ```text
    ///        a      b
    /// self:  •——————•
    ///  rhs:      •——————•
    ///            c      d
    /// ```
    ///
    /// Both `self` and `rhs` are nonempty and $a < c ∧ c < b ∧ b < d$.
    ///
    /// Equivalently, $\self ≠ ∅ ∧ \rhs ≠ ∅ ∧ (∃x ∈ \self, ∀y ∈ \rhs : x < y)
    /// ∧ (∃y ∈ \rhs, ∀x ∈ \self : x < y) ∧ (∃x ∈ \self, ∃y ∈ \rhs : y < x)$.
    Overlaps,

    /// ```text
    ///        a    b                a,b
    /// self:  •————•          self:  • (point)
    ///  rhs:  •————————•       rhs:  •——————•
    ///        c        d             c      d
    /// ```
    ///
    /// Both `self` and `rhs` are nonempty and $a = c ∧ b < d$.
    ///
    /// Equivalently, $\self ≠ ∅ ∧ \rhs ≠ ∅ ∧ (∀y ∈ \rhs, ∃x ∈ \self : x ≤ y)
    /// ∧ (∀x ∈ \self, ∃y ∈ \rhs : y ≤ x) ∧ (∃y ∈ \rhs, ∀x ∈ \self : x < y)$.
    Starts,

    /// ```text
    ///          a    b
    /// self:    •————•
    ///  rhs:  •————————•
    ///        c        d
    /// ```
    ///
    /// Both `self` and `rhs` are nonempty and $c < a ∧ b < d$.
    ///
    /// Equivalently, $\self ≠ ∅ ∧ \rhs ≠ ∅ ∧ (∃y ∈ \rhs, ∀x ∈ \self : y < x)
    /// ∧ (∃y ∈ \rhs, ∀x ∈ \self : x < y)$.
    ContainedBy,

    /// ```text
    ///            a    b                   a,b
    /// self:      •————•      self:         • (point)
    ///  rhs:  •————————•       rhs:  •——————•
    ///        c        d             c      d
    /// ```
    ///
    /// Both `self` and `rhs` are nonempty and $c < a ∧ b = d$.
    ///
    /// Equivalently, $\self ≠ ∅ ∧ \rhs ≠ ∅ ∧ (∃y ∈ \rhs, ∀x ∈ \self : y < x)
    /// ∧ (∀y ∈ \rhs, ∃x ∈ \self : y ≤ x) ∧ (∀x ∈ \self, ∃y ∈ \rhs : x ≤ y)$.
    Finishes,

    /// ```text
    ///        a      b            a,b
    /// self:  •——————•      self:  • (point)
    ///  rhs:  •——————•       rhs:  • (point)
    ///        c      d            c,d
    /// ```
    ///
    /// Both `self` and `rhs` are nonempty and $a = c ∧ b = d$.
    ///
    /// Equivalently, $\self ≠ ∅ ∧ \rhs ≠ ∅ ∧ (∀x ∈ \self, ∃y ∈ \rhs : x = y)
    /// ∧ (∀y ∈ \rhs, ∃x ∈ \self : y = x)$.
    Equals,

    /// ```text
    ///        a        b             a      b
    /// self:  •————————•      self:  •——————•
    ///  rhs:      •————•       rhs:         • (point)
    ///            c    d                   c,d
    /// ```
    ///
    /// Both `self` and `rhs` are nonempty and $a < c ∧ b = d$.
    ///
    /// Equivalently, $\self ≠ ∅ ∧ \rhs ≠ ∅ ∧ (∃x ∈ \self, ∀y ∈ \rhs : x < y)
    /// ∧ (∀x ∈ \self, ∃y ∈ \rhs : x ≤ y) ∧ (∀y ∈ \rhs, ∃x ∈ \self : y ≤ x)$.
    FinishedBy,

    /// ```text
    ///        a        b
    /// self:  •————————•
    ///  rhs:    •————•
    ///          c    d
    /// ```
    ///
    /// Both `self` and `rhs` are nonempty and $a < c ∧ d < b$.
    ///
    /// Equivalently, $\self ≠ ∅ ∧ \rhs ≠ ∅ ∧ (∃x ∈ \self, ∀y ∈ \rhs : x < y)
    /// ∧ (∃x ∈ \self, ∀y ∈ \rhs : y < x)$.
    Contains,

    /// ```text
    ///        a        b             a      b
    /// self:  •————————•      self:  •——————•
    ///  rhs:  •————•           rhs:  • (point)
    ///        c    d                c,d
    /// ```
    ///
    /// Both `self` and `rhs` are nonempty and $a = c ∧ d < b$.
    ///
    /// Equivalently, $\self ≠ ∅ ∧ \rhs ≠ ∅ ∧ (∀x ∈ \self, ∃y ∈ \rhs : y ≤ x)
    /// ∧ (∀y ∈ \rhs, ∃x ∈ \self : x ≤ y) ∧ (∃x ∈ \self, ∀y ∈ \rhs : y < x)$.
    StartedBy,

    /// ```text
    ///            a      b
    /// self:      •——————•
    ///  rhs:  •——————•
    ///        c      d
    /// ```
    ///
    /// Both `self` and `rhs` are nonempty and $c < a ∧ a < d ∧ d < b$.
    ///
    /// Equivalently, $\self ≠ ∅ ∧ \rhs ≠ ∅ ∧ (∃y ∈ \rhs, ∀x ∈ \self : y < x)
    /// ∧ (∃x ∈ \self, ∀y ∈ \rhs : y < x) ∧ (∃y ∈ \rhs, ∃x ∈ \self : x < y)$.
    OverlappedBy,

    /// ```text
    ///               a      b
    /// self:         •——————•
    ///  rhs:  •——————•
    ///        c      d
    /// ```
    ///
    /// Both `self` and `rhs` are nonempty and $c < d ∧ a = d ∧ a < b$.
    ///
    /// Equivalently, $\self ≠ ∅ ∧ \rhs ≠ ∅ ∧ (∀y ∈ \rhs, ∀x ∈ \self : y ≤ x)
    /// ∧ (∃y ∈ \rhs, ∃x ∈ \self : y = x) ∧ (∃y ∈ \rhs, ∀x ∈ \self : y < x)$.
    MetBy,

    /// ```text
    ///                   a      b
    /// self:             •——————•
    ///  rhs:  •——————•
    ///        c      d
    /// ```
    ///
    /// Both `self` and `rhs` are nonempty and $d < a$.
    ///
    /// Equivalently, $\self ≠ ∅ ∧ \rhs ≠ ∅ ∧ ∀y ∈ \rhs, ∀x ∈ \self : y < x$.
    After,
}

impl Interval {
    /// Returns the overlapping state of `self` and `rhs`. See [`OverlappingState`]
    /// for the possible values returned.
    pub fn overlap(self, rhs: Self) -> OverlappingState {
        use OverlappingState::*;

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

        #[allow(clippy::collapsible_if)]
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

impl DecoratedInterval {
    pub fn overlap(self, rhs: Self) -> Option<OverlappingState> {
        if self.is_nai() || rhs.is_nai() {
            return None;
        }

        Some(self.x.overlap(rhs.x))
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    type DI = DecoratedInterval;

    #[test]
    fn nai() {
        assert_eq!(DI::NAI.overlap(DI::PI), None);
        assert_eq!(DI::PI.overlap(DI::NAI), None);
    }

    #[test]
    fn pattern() {
        use OverlappingState::*;
        // Pattern matching results in a more efficient code than using bitmasks as the compiler
        // can eliminate unnecessary comparisons.
        assert!(matches!(
            const_interval!(1.0, 2.0).overlap(const_interval!(3.0, 4.0)),
            BothEmpty | FirstEmpty | SecondEmpty | Before | After
        ))
    }
}
