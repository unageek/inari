use std::{arch::aarch64::*, mem::transmute};

pub(crate) const HAS_MAXIMUM: bool = true;

pub(crate) type F64X2 = float64x2_t;

pub(crate) fn abs(x: F64X2) -> F64X2 {
    vabsq_f64(x)
}

pub(crate) fn all(x: F64X2) -> bool {
    unsafe { transmute::<_, u64>(vmovn_u64(transmute(x))) == !0u64 }
}

pub(crate) fn and(x: F64X2, y: F64X2) -> F64X2 {
    unsafe { transmute(vandq_u64(transmute(x), transmute(y))) }
}

pub(crate) fn any(x: F64X2) -> bool {
    unsafe { transmute::<_, u64>(vmovn_u64(transmute(x))) != 0u64 }
}

pub(crate) fn bitmask(x: F64X2) -> u32 {
    let [a, b] = unsafe { transmute::<_, [u64; 2]>(x) };
    (b as u32) & 0x2 | (a as u32) & 0x1
}

pub(crate) fn ceil(x: F64X2) -> F64X2 {
    vrndpq_f64(x)
}

pub(crate) fn constant(x: f64, y: f64) -> F64X2 {
    unsafe { transmute([x, y]) }
}

pub(crate) fn eq(x: F64X2, y: F64X2) -> F64X2 {
    unsafe { transmute(vceqq_f64(x, y)) }
}

pub(crate) fn extract0(x: F64X2) -> f64 {
    unsafe { transmute::<_, [f64; 2]>(x)[0] }
}

pub(crate) fn extract1(x: F64X2) -> f64 {
    unsafe { transmute::<_, [f64; 2]>(x)[1] }
}

pub(crate) fn floor(x: F64X2) -> F64X2 {
    vrndmq_f64(x)
}

pub(crate) fn ge(x: F64X2, y: F64X2) -> F64X2 {
    unsafe { transmute(vcgeq_f64(x, y)) }
}

pub(crate) fn gt(x: F64X2, y: F64X2) -> F64X2 {
    unsafe { transmute(vcgtq_f64(x, y)) }
}

pub(crate) fn le(x: F64X2, y: F64X2) -> F64X2 {
    unsafe { transmute(vcleq_f64(x, y)) }
}

pub(crate) fn lt(x: F64X2, y: F64X2) -> F64X2 {
    unsafe { transmute(vcltq_f64(x, y)) }
}

pub(crate) fn max(x: F64X2, y: F64X2) -> F64X2 {
    vmaxnmq_f64(x, y)
}

pub(crate) fn maximum(x: F64X2, y: F64X2) -> F64X2 {
    unsafe { vmaxq_f64(x, y) }
}

pub(crate) fn min(x: F64X2, y: F64X2) -> F64X2 {
    vminnmq_f64(x, y)
}

pub(crate) fn minimum(x: F64X2, y: F64X2) -> F64X2 {
    unsafe { vminq_f64(x, y) }
}

pub(crate) fn neg(x: F64X2) -> F64X2 {
    vnegq_f64(x)
}

pub(crate) fn neg0(x: F64X2) -> F64X2 {
    constant(-extract0(x), extract1(x))
}

pub(crate) fn or(x: F64X2, y: F64X2) -> F64X2 {
    unsafe { transmute(vorrq_u64(transmute(x), transmute(y))) }
}

pub(crate) fn round(x: F64X2) -> F64X2 {
    vrndaq_f64(x)
}

pub(crate) fn round_ties_to_even(x: F64X2) -> F64X2 {
    vrndnq_f64(x)
}

pub(crate) fn shuffle02(x: F64X2, y: F64X2) -> F64X2 {
    constant(extract0(x), extract0(y))
}

pub(crate) fn shuffle03(x: F64X2, y: F64X2) -> F64X2 {
    constant(extract0(x), extract1(y))
}

pub(crate) fn shuffle13(x: F64X2, y: F64X2) -> F64X2 {
    constant(extract1(x), extract1(y))
}

pub(crate) fn splat(x: f64) -> F64X2 {
    unsafe { transmute([x, x]) }
}

pub(crate) fn swap(x: F64X2) -> F64X2 {
    shuffle12(x, x)
}

pub(crate) fn trunc(x: F64X2) -> F64X2 {
    vrndq_f64(x)
}

fn shuffle12(x: F64X2, y: F64X2) -> F64X2 {
    constant(extract1(x), extract0(y))
}

macro_rules! impl_op {
    ($t:ty, $f:ident ($x:ident $(,$y:ident)*), $inst:literal) => {
        pub(crate) fn $f(mut $x: $t, $($y: $t,)*) -> $t {
            unsafe {
                asm!(
                    $inst,
                    $x = inout(vreg) $x,
                    $($y = in(vreg) $y,)*
                    options(pure, nomem, nostack, preserves_flags)
                );
            }
            $x
        }
    };
}

impl_op!(F64X2, vabsq_f64(x), "fabs.2d {x:v}, {x:v}");
impl_op!(F64X2, vmaxnmq_f64(x, y), "fmaxnm.2d {x:v}, {x:v}, {y:v}");
impl_op!(F64X2, vminnmq_f64(x, y), "fminnm.2d {x:v}, {x:v}, {y:v}");
impl_op!(F64X2, vnegq_f64(x), "fneg.2d {x:v}, {x:v}");
impl_op!(F64X2, vrndaq_f64(x), "frinta.2d {x:v}, {x:v}");
impl_op!(F64X2, vrndmq_f64(x), "frintm.2d {x:v}, {x:v}");
impl_op!(F64X2, vrndnq_f64(x), "frintn.2d {x:v}, {x:v}");
impl_op!(F64X2, vrndpq_f64(x), "frintp.2d {x:v}, {x:v}");
impl_op!(F64X2, vrndq_f64(x), "frintz.2d {x:v}, {x:v}");

macro_rules! impl_op_round {
    ($t:ty, $f:ident ($x:ident $(,$y:ident)*), $inst:literal, rd) => {
        impl_op_round!($t, $f ($x $(,$y)*), $inst, 0x800000);
    };

    ($t:ty, $f:ident ($x:ident $(,$y:ident)*), $inst:literal, ru) => {
        impl_op_round!($t, $f ($x $(,$y)*), $inst, 0x400000);
    };

    ($t:ty, $f:ident ($x:ident $(,$y:ident)*), $inst:literal, $fpcr:literal) => {
        pub(crate) fn $f(mut $x: $t, $($y: $t,)*) -> $t {
            unsafe {
                asm!(
                    "mrs {fpcr_bak}, fpcr",
                    "msr fpcr, {fpcr:x}",
                    $inst,
                    "msr fpcr, {fpcr_bak}",
                    $x = inout(vreg) $x,
                    $($y = in(vreg) $y,)*
                    fpcr = in(reg) $fpcr,
                    fpcr_bak = out(reg) _,
                    options(pure, nomem, nostack, preserves_flags)
                );
            }
            $x
        }
    };
}

impl_op_round!(f64, sqrt1_rd(x), "fsqrt {x:d}, {x:d}", rd);
impl_op_round!(f64, sqrt1_ru(x), "fsqrt {x:d}, {x:d}", ru);
impl_op_round!(f64, sub1_ru(x, y), "fsub {x:d}, {x:d}, {y:d}", ru);
impl_op_round!(F64X2, add_ru(x, y), "fadd.2d {x:v}, {x:v}, {y:v}", ru);
impl_op_round!(F64X2, mul_ru(x, y), "fmul.2d {x:v}, {x:v}, {y:v}", ru);
impl_op_round!(F64X2, div_ru(x, y), "fdiv.2d {x:v}, {x:v}, {y:v}", ru);

pub(crate) fn mul_add_ru(x: F64X2, y: F64X2, mut z: F64X2) -> F64X2 {
    unsafe {
        asm!(
            "mrs {fpcr_bak}, fpcr",
            "msr fpcr, {fpcr:x}",
            "fmla.2d {z:v}, {x:v}, {y:v}",
            "msr fpcr, {fpcr_bak}",
            x = in(vreg) x,
            y = in(vreg) y,
            z = inout(vreg) z,
            fpcr = in(reg) 0x400000,
            fpcr_bak = out(reg) _,
            options(pure, nomem, nostack, preserves_flags)
        );
    }
    z
}
