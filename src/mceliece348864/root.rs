//! This file is for evaluating a polynomial at one or more field elements

use super::{SYS_N, SYS_T};

use crate::common::gf12::{gf_add, gf_mul, Gf};

/// Evaluate polynomial `f` with argument `a`.
/// Thus it returns `f(a)` in symbolic notation.
pub(crate) fn eval(f: &[Gf; SYS_T + 1], a: Gf) -> Gf {
    let mut r: Gf = f[SYS_T];

    for i in (0..=SYS_T - 1).rev() {
        r = gf_mul(r, a);
        r = gf_add(r, f[i]);
    }
    r
}

/// Given polynomial `f` and a list of field elements `l`,
/// return the roots `out` satisfying `[ f(a) for a in L ]`
pub(crate) fn root(out: &mut [Gf; SYS_N], f: &[Gf; SYS_T + 1], l: &[Gf; SYS_N]) {
    for i in 0..SYS_N {
        out[i] = eval(f, l[i]);
    }
}

#[cfg(test)]
mod tests {
    use super::super::CRYPTO_PRIMITIVE;
    use super::*;

    #[test]
    fn test_root() {
        let mut out = [0u16; SYS_N];
        let mut f = [0u16; SYS_T + 1];
        let mut l = [0u16; SYS_N];

        for i in 0..SYS_N {
            out[i] = (i as Gf).wrapping_add(3);
            l[i] = (i as Gf).wrapping_add(7);
        }
        for i in 0..(SYS_T + 1) {
            f[i] = (i as Gf).wrapping_mul(3);
        }

        root(&mut out, &f, &l);

        let mut name = format!("{}_root_out_expected", CRYPTO_PRIMITIVE);
        // NOTE the f-variants equals the non-f variants. We only stored the non-f variants
        name = name.replace("f_root_out", "_root_out");
        let expected = crate::TestData::new().u16vec(&name);
        assert_eq!(expected, out);
    }
}
