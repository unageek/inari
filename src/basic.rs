use crate::{classify::*, interval::*, simd::*};

// NOTE: `neg`, `add`, `sub`, `mul` and `div` are implemented in arith.rs

impl Interval {
    /// Returns $(\self × \rhs) + \addend$.
    ///
    /// | Domain | Range |
    /// | ------ | ----- |
    /// | $\R^3$ | $\R$  |
    // Almost a copy-paste of mul. Additions/modifications are indicated with "// *".
    #[allow(clippy::many_single_char_names)]
    pub fn mul_add(self, rhs: Self, addend: Self) -> Self {
        if addend.is_empty() {
            return Self::EMPTY; // *
        }

        use IntervalClass2::*;
        match self.classify2(rhs) {
            E_E | E_M | E_N0 | E_N1 | E_P0 | E_P1 | E_Z | M_E | N0_E | N1_E | P0_E | P1_E | Z_E => {
                Self::EMPTY
            }
            M_Z | N0_Z | N1_Z | P0_Z | P1_Z | Z_M | Z_N0 | Z_N1 | Z_P0 | Z_P1 | Z_Z => addend, // *
            M_M => {
                let x = shuffle02(self.rep, self.rep);
                let y = swap(rhs.rep);
                let xy = mul_add_ru(x, y, addend.rep); // *
                let z = shuffle13(self.rep, self.rep);
                let w = rhs.rep;
                let zw = mul_add_ru(z, w, addend.rep); // *
                let r = max(xy, zw);
                Self { rep: r }
            }
            M_N0 | M_N1 => {
                let x = swap(self.rep);
                let y = shuffle02(rhs.rep, rhs.rep);
                Self {
                    rep: mul_add_ru(x, y, addend.rep), // *
                }
            }
            M_P0 | M_P1 => {
                let x = self.rep;
                let y = shuffle13(rhs.rep, rhs.rep);
                Self {
                    rep: mul_add_ru(x, y, addend.rep), // *
                }
            }
            N0_M | N1_M => {
                let x = shuffle02(self.rep, self.rep);
                let y = swap(rhs.rep);
                Self {
                    rep: mul_add_ru(x, y, addend.rep), // *
                }
            }
            N0_N0 | N0_N1 | N1_N0 | N1_N1 => {
                let x = swap(self.rep);
                let x = neg0(x);
                let y = swap(rhs.rep);
                Self {
                    rep: mul_add_ru(x, y, addend.rep), // *
                }
            }
            N0_P0 | N0_P1 | N1_P0 | N1_P1 => {
                let x = self.rep;
                let y = neg0(rhs.rep);
                let y = swap(y);
                Self {
                    rep: mul_add_ru(x, y, addend.rep), // *
                }
            }
            P0_M | P1_M => {
                let x = shuffle13(self.rep, self.rep);
                let y = rhs.rep;
                Self {
                    rep: mul_add_ru(x, y, addend.rep), // *
                }
            }
            P0_N0 | P0_N1 | P1_N0 | P1_N1 => {
                let x = neg0(self.rep);
                let x = swap(x);
                let y = rhs.rep;
                Self {
                    rep: mul_add_ru(x, y, addend.rep), // *
                }
            }
            P0_P0 | P0_P1 | P1_P0 | P1_P1 => {
                let x = self.rep;
                let y = neg0(rhs.rep);
                Self {
                    rep: mul_add_ru(x, y, addend.rep), // *
                }
            }
        }
    }

    /// Returns the multiplicative inverse of `self`.
    ///
    /// | Domain        | Range         |
    /// | ------------- | ------------- |
    /// | $\R ∖ \set 0$ | $\R ∖ \set 0$ |
    pub fn recip(self) -> Self {
        use IntervalClass::*;
        match self.classify() {
            E | Z => Self::EMPTY,
            M => Self::ENTIRE,
            N0 => {
                // 1 / N0 => [-∞, 1/a] = [+∞; -1/-a]
                let x = splat(-1.0);
                let y = self.rep; // [-a; b]
                Self {
                    rep: shuffle02(splat(f64::INFINITY), div_ru(x, y)),
                }
            }
            N1 => {
                // 1 / N1 => [1/b, 1/a] = [-1/b; 1/a] = [-1/b; -1/-a] = [-1; -1] ./ [b; -a]
                let x = splat(-1.0);
                let y = swap(self.rep); // [b; -a]
                Self { rep: div_ru(x, y) }
            }
            P0 => {
                // 1 / P0 => [1/b, +∞] = [-1/b; +∞]
                let x = splat(-1.0);
                let y = self.rep; // [-a; b]
                Self {
                    rep: shuffle13(div_ru(x, y), splat(f64::INFINITY)),
                }
            }
            P1 => {
                // 1 / P1 => [1/b, 1/a] = [-1/b; 1/a] = [-1/b; -1/-a] = [-1; -1] ./ [b; -a]
                let x = splat(-1.0);
                let y = swap(self.rep); // [b; -a]
                Self { rep: div_ru(x, y) }
            }
        }
    }

    /// Returns the square of `self`.
    ///
    /// | Domain | Range     |
    /// | ------ | --------- |
    /// | $\R$   | $\[0, ∞)$ |
    pub fn sqr(self) -> Self {
        use IntervalClass::*;
        match self.classify() {
            E => Self::EMPTY,
            Z => Self::zero(),
            M => {
                // [0, max(a^2, b^2)] = [0; max(a^2, b^2)]
                let x = self.rep; // [-a; b]
                let r = mul_ru(x, x); // [a^2; b^2]
                let r = max(r, swap(r)); // [max(a^2, b^2); _]
                Self {
                    rep: shuffle02(splat(0.0), r),
                }
            }
            N0 | N1 => {
                // [b^2, a^2] = [-b^2; a^2] = [-b; -a] .* [b; -a]
                let x = swap(self.rep); // [b; -a]
                let y = neg0(x); // [-b; -a]
                Self { rep: mul_ru(x, y) }
            }
            P0 | P1 => {
                // [a^2, b^2] = [-a^2; b^2] = [-a; b] .* [a; b]
                let x = self.rep; // [-a; b]
                let y = neg0(x); // [a; b]
                Self { rep: mul_ru(x, y) }
            }
        }
    }

    /// Returns the principal square root of `self`.
    ///
    /// | Domain    | Range     |
    /// | --------- | --------- |
    /// | $\[0, ∞)$ | $\[0, ∞)$ |
    pub fn sqrt(self) -> Self {
        if self.is_empty() {
            return Self::EMPTY;
        }

        let a = self.inf_raw();
        let b = self.sup_raw();

        if b < 0.0 {
            Self::EMPTY
        } else if a <= 0.0 {
            Self::with_infsup_raw(0.0, sqrt1_ru(b))
        } else {
            Self::with_infsup_raw(sqrt1_rd(a), sqrt1_ru(b))
        }
    }
}

impl DecInterval {
    /// The decorated version of [`Interval::mul_add`].
    ///
    /// A NaI is returned if `self`, `rhs` or `addend` is NaI.
    pub fn mul_add(self, rhs: Self, addend: Self) -> Self {
        if self.is_nai() || rhs.is_nai() || addend.is_nai() {
            return Self::NAI;
        }

        Self::set_dec(
            self.x.mul_add(rhs.x, addend.x),
            self.d.min(rhs.d.min(addend.d)),
        )
    }

    /// The decorated version of [`Interval::recip`].
    ///
    /// A NaI is returned if `self` is NaI.
    pub fn recip(self) -> Self {
        if self.is_nai() {
            return self;
        }

        let d = if self.x.contains(0.0) {
            Decoration::Trv
        } else {
            self.d
        };
        Self::set_dec(self.x.recip(), d)
    }

    /// The decorated version of [`Interval::sqr`].
    ///
    /// A NaI is returned if `self` is NaI.
    pub fn sqr(self) -> Self {
        if self.is_nai() {
            return self;
        }

        Self::set_dec(self.x.sqr(), self.d)
    }

    /// The decorated version of [`Interval::sqrt`].
    ///
    /// A NaI is returned if `self` is NaI.
    pub fn sqrt(self) -> Self {
        if self.is_nai() {
            return self;
        }

        let d = if self.x.inf_raw() < 0.0 {
            Decoration::Trv
        } else {
            self.d
        };
        Self::set_dec(self.x.sqrt(), d)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use DecInterval as DI;
    use Interval as I;

    #[test]
    fn empty() {
        assert!((I::EMPTY.mul_add(I::PI, I::PI)).is_empty());
        assert!((I::PI.mul_add(I::EMPTY, I::PI)).is_empty());
        assert!((I::PI.mul_add(I::PI, I::EMPTY)).is_empty());

        assert!(I::EMPTY.recip().is_empty());
        assert!(I::EMPTY.sqrt().is_empty());
        assert!(I::EMPTY.sqr().is_empty());

        assert!((DI::EMPTY.mul_add(DI::PI, DI::PI)).is_empty());
        assert!((DI::PI.mul_add(DI::EMPTY, DI::PI)).is_empty());
        assert!((DI::PI.mul_add(DI::PI, DI::EMPTY)).is_empty());

        assert!(DI::EMPTY.recip().is_empty());
        assert!(DI::EMPTY.sqrt().is_empty());
        assert!(DI::EMPTY.sqr().is_empty());
    }

    #[test]
    fn nai() {
        assert!((DI::NAI.mul_add(DI::PI, DI::PI)).is_nai());
        assert!((DI::PI.mul_add(DI::NAI, DI::PI)).is_nai());
        assert!((DI::PI.mul_add(DI::PI, DI::NAI)).is_nai());

        assert!(DI::NAI.recip().is_nai());
        assert!(DI::NAI.sqrt().is_nai());
        assert!(DI::NAI.sqr().is_nai());
    }
}
