
#[allow(non_camel_case_types)]
#[repr(u16)]
pub enum ByteCode {
    nop = 0,

    // const:
    i32_const,
    i64_const,
    f32_const,
    f64_const,

    // arithmetic
    i32_add,
    i32_sub,
    i32_mul,
    i32_div,
    i32_rem,
    i32_and,
    i32_or,
    i32_xor,
    i32_shl,
    i32_shr,
    i32_ushr,

    i64_add,
    i64_sub,
    i64_mul,
    i64_div,
    i64_rem,
    i64_and,
    i64_or,
    i64_xor,
    i64_shl,
    i64_shr,
    i64_ushr,

    f32_add,
    f32_sub,
    f32_mul,
    f32_div,
    f32_min,
    f32_max,

    f64_add,
    f64_sub,
    f64_mul,
    f64_div,
    f64_min,
    f64_max,

    ref_null,
}

#[repr(C)]
pub struct Const32 {
    pub value: i32,
    pub dst: u16,
}

#[repr(C)]
pub struct Const64 {
    pub value: i64,
    pub dst: u16,
}

#[repr(C)]
pub struct ThreeAddressCode {
    pub dst: u16,
    pub src1: u16,
    pub src2: u16,
}

