use crate::{classify::*, interval::*, simd::*};

impl Interval {
    /// Returns the absolute value of `self`.
    ///
    /// | Domain | Range     |
    /// | ------ | --------- |
    /// | $\R$   | $\[0, ∞)$ |
    ///
    /// Tightness: tightest
    pub fn abs(self) -> Self {
        use IntervalClass::*;
        match self.classify() {
            E | P0 | P1 | Z => self,
            M => {
                // [0, max(-a, b)] = [0; max(-a, b)]
                let x = self.rep; // [-a; b]
                let r = max(x, swap(x)); // [max(-a, b); _]
                Self {
                    rep: shuffle02(splat(0.0), r),
                }
            }
            N0 | N1 => {
                // [-b, -a] = [b; -a]
                Self {
                    rep: swap(self.rep),
                }
            }
        }
    }

    /// Returns the maximum of `self` and `rhs`.
    ///
    /// | Domain | Range |
    /// | ------ | ----- |
    /// | $\R^2$ | $\R$  |
    ///
    /// Tightness: tightest
    pub fn max(self, rhs: Self) -> Self {
        if HAS_MAXIMUM {
            // [max(a, c), max(b, d)] = [-max(a, c); max(b, d)] = [min(-a, -c); max(b, d)]
            let min = minimum(self.rep, rhs.rep); // [min(-a, -c); min(b, d)]
            let max = maximum(self.rep, rhs.rep); // [max(-a, -c); max(b, d)]
            Self {
                rep: shuffle03(min, max),
            }
        } else {
            if self.either_empty(rhs) {
                return Self::EMPTY;
            }

            let min = min(self.rep, rhs.rep);
            let max = max(self.rep, rhs.rep);
            Self {
                rep: shuffle03(min, max),
            }
        }
    }

    /// Returns the minimum of `self` and `rhs`.
    ///
    /// | Domain | Range |
    /// | ------ | ----- |
    /// | $\R^2$ | $\R$  |
    ///
    /// Tightness: tightest
    pub fn min(self, rhs: Self) -> Self {
        if HAS_MAXIMUM {
            // [min(a, c), min(b, d)] = [-min(a, c); min(b, d)] = [max(-a, -c); min(b, d)]
            let max = maximum(self.rep, rhs.rep); // [max(-a, -c); max(b, d)]
            let min = minimum(self.rep, rhs.rep); // [min(-a, -c); min(b, d)]
            Self {
                rep: shuffle03(max, min),
            }
        } else {
            if self.either_empty(rhs) {
                return Self::EMPTY;
            }

            let max = max(self.rep, rhs.rep);
            let min = min(self.rep, rhs.rep);
            Self {
                rep: shuffle03(max, min),
            }
        }
    }
}

impl DecInterval {
    /// The decorated version of [`Interval::abs`].
    ///
    /// A NaI is returned if `self` is NaI.
    pub fn abs(self) -> Self {
        if self.is_nai() {
            return self;
        }

        Self::set_dec(self.x.abs(), self.d)
    }

    /// The decorated version of [`Interval::max`].
    ///
    /// A NaI is returned if `self` or `rhs` is NaI.
    pub fn max(self, rhs: Self) -> Self {
        if self.is_nai() {
            return self;
        }

        Self::set_dec(self.x.max(rhs.x), self.d.min(rhs.d))
    }

    /// The decorated version of [`Interval::min`].
    ///
    /// A NaI is returned if `self` or `rhs` is NaI.
    pub fn min(self, rhs: Self) -> Self {
        if self.is_nai() {
            return self;
        }

        Self::set_dec(self.x.min(rhs.x), self.d.min(rhs.d))
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use DecInterval as DI;
    use Interval as I;

    #[test]
    fn empty() {
        assert!(I::EMPTY.abs().is_empty());

        assert!(I::EMPTY.max(I::PI).is_empty());
        assert!(I::PI.max(I::EMPTY).is_empty());

        assert!(I::EMPTY.min(I::PI).is_empty());
        assert!(I::PI.min(I::EMPTY).is_empty());

        assert!(DI::EMPTY.abs().is_empty());

        assert!(DI::EMPTY.max(DI::PI).is_empty());
        assert!(DI::PI.max(DI::EMPTY).is_empty());

        assert!(DI::EMPTY.min(DI::PI).is_empty());
        assert!(DI::PI.min(DI::EMPTY).is_empty());
    }

    #[test]
    fn nai() {
        assert!(DI::NAI.abs().is_nai());

        assert!(DI::NAI.max(DI::PI).is_nai());
        assert!(DI::PI.max(DI::NAI).is_nai());

        assert!(DI::NAI.min(DI::PI).is_nai());
        assert!(DI::PI.min(DI::NAI).is_nai());
    }
}
