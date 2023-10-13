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
fn greater_zero_ret_one_3(num: usize) -> usize {
    // if num is 0, identity will have a value of 0 as all bits are 0, for other values, this will overflow.
    let identity = 0_usize - num;
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
fn greater_zero_ret_one_4(num: usize) -> usize {
    // if num is 0, identity will have a value of 0 as all bits are 0, for other values, this will overflow.
    let identity = 0_usize.overflowing_sub(num);
    unsafe { transmute::<bool, u8>(identity.1) as usize }
}