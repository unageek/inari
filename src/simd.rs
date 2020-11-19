use std::arch::x86_64::*;

pub(crate) fn abs(x: __m128d) -> __m128d {
    // Drop sign bits.
    unsafe { _mm_andnot_pd(_mm_set1_pd(-0.0), x) }
}

pub(crate) fn dup_hi(x: __m128d) -> __m128d {
    unsafe { _mm_unpackhi_pd(x, x) }
}

pub(crate) fn dup_lo(x: __m128d) -> __m128d {
    unsafe { _mm_unpacklo_pd(x, x) }
}

pub(crate) fn negate_lo(x: __m128d) -> __m128d {
    unsafe { _mm_xor_pd(x, _mm_set_sd(-0.0)) }
}

pub(crate) fn set_hi_inf(x: __m128d) -> __m128d {
    unsafe { _mm_move_sd(_mm_set1_pd(f64::INFINITY), x) }
}

pub(crate) fn set_lo_inf(x: __m128d) -> __m128d {
    unsafe { _mm_move_sd(x, _mm_set1_pd(f64::INFINITY)) }
}

pub(crate) fn swap(x: __m128d) -> __m128d {
    unsafe { _mm_shuffle_pd(x, x, 1) }
}

#[cfg(not(feature = "avx512"))]
macro_rules! impl_op_round {
    ($t:ty, $f:ident ($x:ident $(,$y:ident)*), $inst:literal, rd) => {
        impl_op_round!($t, $f ($x $(,$y)*), $inst, "16256"); // _MM_ROUND_DOWN | _MM_MASK_MASK
    };

    ($t:ty, $f:ident ($x:ident $(,$y:ident)*), $inst:literal, ru) => {
        impl_op_round!($t, $f ($x $(,$y)*), $inst, "24448"); // _MM_ROUND_UP | _MM_MASK_MASK
    };

    ($t:ty, $f:ident ($x:ident $(,$y:ident)*), $inst:literal, $mxcsr:literal) => {
        pub(crate) fn $f(mut $x: $t, $($y: $t,)*) -> $t {
            unsafe {
                asm!(
                    "sub rsp, 8",
                    "stmxcsr [rsp]",
                    concat!("mov dword ptr [rsp + 4], ", $mxcsr),
                    "ldmxcsr [rsp + 4]",
                    $inst,
                    "ldmxcsr [rsp]",
                    "add rsp, 8",
                    $x = inout(xmm_reg) $x,
                    $($y = in(xmm_reg) $y,)*
                    options(pure, nomem)
                );
            }
            $x
        }
    };
}

#[cfg(feature = "avx512")]
macro_rules! impl_op_round {
    ($t:ty, $reg:ident, $f:ident ($x:ident $(,$y:ident)*), $inst:literal, rd) => {
        impl_op_round!($t, $reg, $f ($x $(,$y)*), $inst, "{{rd-sae}}");
    };

    ($t:ty, $reg:ident, $f:ident ($x:ident $(,$y:ident)*), $inst:literal, ru) => {
        impl_op_round!($t, $reg, $f ($x $(,$y)*), $inst, "{{ru-sae}}");
    };

    ($t:ty, $reg:ident, $f:ident ($x:ident $(,$y:ident)*), $inst:literal, $er:literal) => {
        pub(crate) fn $f($x: $t, $($y: $t,)*) -> $t {
            #[target_feature(enable = "avx512f")]
            unsafe fn inner(mut $x: $t, $($y: $t,)*) -> $t {
                asm!(
                    concat!($inst, ", ", $er),
                    $x = inout($reg) $x,
                    $($y = in($reg) $y,)*
                    options(pure, nomem, nostack, preserves_flags)
                );
                $x
            }

            unsafe { inner($x, $($y,)*) }
        }
    };
}

#[cfg(not(feature = "avx512"))]
impl_op_round!(f64, sqrt1_rd(x), "sqrtpd {x}, {x}", rd);
#[cfg(feature = "avx512")]
impl_op_round!(f64, xmm_reg, sqrt1_rd(x), "vsqrtsd {x}, {x}, {x}", rd);

#[cfg(not(feature = "avx512"))]
impl_op_round!(f64, sqrt1_ru(x), "sqrtpd {x}, {x}", ru);
#[cfg(feature = "avx512")]
impl_op_round!(f64, xmm_reg, sqrt1_ru(x), "vsqrtsd {x}, {x}, {x}", ru);

#[cfg(not(feature = "avx512"))]
impl_op_round!(f64, sub1_ru(x, y), "subpd {x}, {y}", ru);
#[cfg(feature = "avx512")]
impl_op_round!(f64, xmm_reg, sub1_ru(x, y), "vsubsd {x}, {x}, {y}", ru);

#[cfg(not(feature = "avx512"))]
impl_op_round!(__m128d, add_ru(x, y), "addpd {x}, {y}", ru);
#[cfg(feature = "avx512")]
impl_op_round!(
    __m128d,
    zmm_reg,
    add_ru(x, y),
    "vaddpd {x:z}, {x:z}, {y:z}",
    ru
);

#[cfg(not(feature = "avx512"))]
impl_op_round!(__m128d, mul_ru(x, y), "mulpd {x}, {y}", ru);
#[cfg(feature = "avx512")]
impl_op_round!(
    __m128d,
    zmm_reg,
    mul_ru(x, y),
    "vmulpd {x:z}, {x:z}, {y:z}",
    ru
);

#[cfg(not(feature = "avx512"))]
impl_op_round!(__m128d, div_ru(x, y), "divpd {x}, {y}", ru);
#[cfg(feature = "avx512")]
impl_op_round!(
    __m128d,
    zmm_reg,
    div_ru(x, y),
    "vdivpd {x:z}, {x:z}, {y:z}",
    ru
);

#[cfg(not(feature = "avx512"))]
impl_op_round!(
    __m128d,
    mul_add_ru(x, y, z),
    "vfmadd213pd {x}, {y}, {z}",
    ru
);
#[cfg(feature = "avx512")]
impl_op_round!(
    __m128d,
    zmm_reg,
    mul_add_ru(x, y, z),
    "vfmadd213pd {x:z}, {y:z}, {z:z}",
    ru
);
