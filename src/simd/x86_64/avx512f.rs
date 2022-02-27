use std::arch::asm;

macro_rules! impl_op_round {
    ($t:ty, $reg:ident, $f:ident ($x:ident $(,$y:ident)*), $inst:literal, rd) => {
        impl_op_round!($t, $reg, $f ($x $(,$y)*), $inst, "{{rd-sae}}");
    };

    ($t:ty, $reg:ident, $f:ident ($x:ident $(,$y:ident)*), $inst:literal, ru) => {
        impl_op_round!($t, $reg, $f ($x $(,$y)*), $inst, "{{ru-sae}}");
    };

    ($t:ty, $reg:ident, $f:ident ($x:ident $(,$y:ident)*), $inst:literal, $er:literal) => {
        pub(crate) fn $f(mut $x: $t, $($y: $t,)*) -> $t {
            unsafe {
                asm!(
                    concat!($inst, ", ", $er),
                    $x = inout($reg) $x,
                    $($y = in($reg) $y,)*
                    options(pure, nomem, nostack, preserves_flags)
                );
                $x
            }
        }
    };
}

impl_op_round!(f64, xmm_reg, sqrt1_rd(x), "vsqrtsd {x}, {x}, {x}", rd);
impl_op_round!(f64, xmm_reg, sqrt1_ru(x), "vsqrtsd {x}, {x}, {x}", ru);
impl_op_round!(f64, xmm_reg, sub1_ru(x, y), "vsubsd {x}, {x}, {y}", ru);
impl_op_round!(
    super::F64X2,
    zmm_reg,
    add_ru(x, y),
    "vaddpd {x:z}, {x:z}, {y:z}",
    ru
);
impl_op_round!(
    super::F64X2,
    zmm_reg,
    sub_ru(x, y),
    "vsubpd {x:z}, {x:z}, {y:z}",
    ru
);
impl_op_round!(
    super::F64X2,
    zmm_reg,
    mul_ru(x, y),
    "vmulpd {x:z}, {x:z}, {y:z}",
    ru
);
impl_op_round!(
    super::F64X2,
    zmm_reg,
    div_ru(x, y),
    "vdivpd {x:z}, {x:z}, {y:z}",
    ru
);
impl_op_round!(
    super::F64X2,
    zmm_reg,
    mul_add_ru(x, y, z),
    "vfmadd213pd {x:z}, {y:z}, {z:z}",
    ru
);
