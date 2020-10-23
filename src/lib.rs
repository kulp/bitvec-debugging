#[cfg(test)]
use bitvec::prelude::*;

#[test]
fn workaround() {
    let mut bv = BitVec::new();

    bv.extend_from_slice(1u32.bits::<Lsb0>());

    let bb: BitBox = bv.into();

    assert_eq!(bb, BitBox::new(1u32.bits::<Lsb0>()));
}

