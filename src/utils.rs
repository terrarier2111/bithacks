use std::mem::transmute;

/// a short, branchless algorithm that is eqivalent to
/// if num > 0:
///    ret 1
/// else:
///    ret 0
#[inline]
fn greater_zero_ret_one(num: usize) -> usize {
    let lz = num.leading_zeros() as usize;
    let word_bits = usize::BITS as usize;
    // if num is 0, identity will have a value of 0 as all bits are 0, for other values, this will overflow.
    let identity = lz - word_bits;
    let msb_off = (u32::BITS - 1) as usize;
    let msb = 1 << msb_off;
    (identity & msb) >> msb_off
}

/// a short, branchless algorithm that is eqivalent to
/// if num > 0:
///    ret 1
/// else:
///    ret 0
#[inline]
fn greater_zero_ret_one_2(num: usize) -> usize {
    let lz = num.leading_zeros() as usize;
    let word_bits = usize::BITS as usize;
    // if num is 0, identity will have a value of 0 as all bits are 0, for other values, this will overflow.
    let identity = lz.overflowing_sub(word_bits);
    unsafe { transmute::<bool, u8>(identity.1) as usize }
}

/// a short, branchless algorithm that is eqivalent to
/// if num > 0:
///    ret 1
/// else:
///    ret 0
#[inline]
const fn greater_zero_ret_one_3(num: usize) -> usize {
    const MSB_OFF: usize = (usize::BITS - 1) as usize;

    // if num is 0, identity will have a value of 0 as all bits are 0, for other values, this will overflow.
    let identity = 0_usize - num;
    identity >> MSB_OFF
}

/// a short, branchless algorithm that is eqivalent to
/// if num > 0:
///    ret 1
/// else:
///    ret 0
#[inline]
const fn greater_zero_ret_one_4(num: usize) -> usize {
    // if num is 0, identity will have a value of 0 as all bits are 0, for other values, this will overflow.
    let identity = 0_usize.overflowing_sub(num);
    unsafe { transmute::<bool, u8>(identity.1) as usize }
}

#[no_mangle]
#[inline(never)]
pub(crate) const fn build_bit_mask(offset: usize, ones_cnt: usize) -> usize {
    let mut mask = 0;
    let mut bit = 0;
    while bit < ones_cnt {
        mask |= 1 << bit;
        bit += 1;
    }
    mask << offset
}

#[allow(arithmetic_overflow)]
#[no_mangle]
#[inline(never)]
pub(crate) fn build_bit_mask2(offset: usize, ones_cnt: usize) -> usize {
    /*let mut mask = 0;
    let mut bit = 0;
    while bit < ones_cnt {
        mask |= 1 << bit;
        bit += 1;
    }
    mask << offset*/
    ((1 << ones_cnt) - 1 | ((1 - greater_zero_ret_zero(ones_cnt)) << ones_cnt)) << offset
}

/// a short, branchless algorithm that is eqivalent to
/// if num > 0:
///    ret 0
/// else:
///    ret 1
#[inline]
pub(crate) fn greater_zero_ret_zero(num: usize) -> usize {
    1 - greater_zero_ret_one_3(num)
}

#[no_mangle]
#[inline(never)]
pub fn four_to_pow(pow: usize) -> usize {
    4_usize.pow(pow as u32)
}

#[no_mangle]
#[inline(never)]
pub fn four_to_pow_v2(pow: usize) -> usize {
    if pow >= (usize::BITS as usize / 2) {
        return 0;
    }
    1 << (2 * pow)
}

#[no_mangle]
#[inline(never)]
pub fn four_to_pow_v3(pow: usize) -> usize {
    let exceeds = pow & !(usize::BITS as usize / 2 - 1);
    1 << (2 * (pow & !(greater_zero_ret_one_3(exceeds) * usize::MAX)))
}

fn simple_build_bit_mask(offset: usize, ones_cnt: usize) -> usize {
    let mut mask = 0;
    let mut bit = 0;
    while bit < ones_cnt {
        mask |= 1 << bit;
        bit += 1;
    }
    mask << offset
}

fn test_build_bit_mask_valid() {
    for off in 0..(usize::BITS as usize) {
        for cnt in 0..(usize::BITS as usize) {
            assert_eq!(simple_build_bit_mask(off, cnt), build_bit_mask(off, cnt));
        }
    }
}