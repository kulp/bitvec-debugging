#[cfg(test)]
use bitvec::prelude::*;

#[test]
fn debug_bitbox() {
    let len = 8065; // 8065 fails, 8064 passes
    let mut bv = BitVec::with_capacity(len);

    bv.push(false);

    let _: BitBox = bv.into();
}
