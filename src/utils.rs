#[inline]
pub fn align_reference(raw_size: usize) -> usize {
    align(raw_size, 8)
}

#[inline]
pub fn align(raw_size: usize, align_size: usize) -> usize {
    let r#mod = raw_size % align_size;
    if r#mod == 0 {
        raw_size
    } else {
        raw_size + (align_size - r#mod)
    }
}
