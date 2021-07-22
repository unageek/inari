// A SIMD vector [x0; x1] stored in a register is usually depicted as:
//
//   +----------------+----------------+
//   |       x1       |       x0       |
//   +----------------+----------------+
//   |127           64|63             0|
//
//   r[63:0] = x0, r[127:64] = x1.

cfg_if::cfg_if! {
    if #[cfg(docsrs)] {
        mod unimplemented;
        pub(crate) use unimplemented::*;
    } else if #[cfg(target_arch = "aarch64")] {
        mod aarch64;
        pub(crate) use aarch64::*;
    } else if #[cfg(target_arch = "x86_64")] {
        mod x86_64;
        pub(crate) use x86_64::*;
    } else {
        unreachable!();
    }
}
