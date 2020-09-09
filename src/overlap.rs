use crate::interval::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OverlappingState {
    Undefined,
    BothEmpty,
    FirstEmpty,
    SecondEmpty,
    Before,
    Meets,
    Overlaps,
    Starts,
    ContainedBy,
    Finishes,
    Equal,
    FinishedBy,
    Contains,
    StartedBy,
    OverlappedBy,
    MetBy,
    After,
}

impl Interval {
    /// Returns the overlapping state of `self` and `rhs`.
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
                Equal
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
    pub fn overlap(self, rhs: Self) -> OverlappingState {
        if self.is_nai() || rhs.is_nai() {
            return OverlappingState::Undefined;
        }

        self.x.overlap(rhs.x)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    type DI = DecoratedInterval;

    #[test]
    fn nai() {
        assert_eq!(DI::NAI.overlap(DI::PI), OverlappingState::Undefined);
        assert_eq!(DI::PI.overlap(DI::NAI), OverlappingState::Undefined);
    }
}
