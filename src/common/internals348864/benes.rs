use super::params::{COND_BYTES, GFBITS, SYS_N};
use crate::common::{benes::apply_benes_gf12, gf12, gf12::Gf};

pub(crate) fn support_gen(s: &mut [Gf; SYS_N], c: &[u8; COND_BYTES]) {
    let mut a: Gf;
    let mut l = [[0u8; (1 << GFBITS) / 8]; GFBITS];

    for i in 0..(1 << GFBITS) {
        a = gf12::bitrev(i as Gf);

        for j in 0..GFBITS {
            l[j][i / 8] |= (((a >> j) & 1) << (i % 8)) as u8;
        }
    }

    for j in 0..GFBITS {
        apply_benes_gf12(&mut l[j], c, 0);
    }

    for i in 0..SYS_N {
        s[i] = 0;
        for j in (0..=(GFBITS - 1)).rev() {
            s[i] <<= 1;
            s[i] |= ((l[j][i / 8] >> (i % 8)) & 1) as u16;
        }
    }
}