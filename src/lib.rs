#[cfg(test)]
use bitvec::prelude::*;

#[test]
fn workaround() {
    let mut bv = BitVec::new();

    bv.extend_from_bitslice(1usize.bits::<Lsb0>());

    let bb: BitBox = bv.into();

    assert_eq!(bb, BitBox::new(1usize.bits::<Lsb0>()));
}

