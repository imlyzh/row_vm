
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

    // comparison
    i32_eq,
    i32_ne,
    i32_lt,
    i32_le,
    i32_gt,
    i32_ge,

    i64_eq,
    i64_ne,
    i64_lt,
    i64_le,
    i64_gt,
    i64_ge,

    f32_eq,
    f32_ne,
    f32_lt,
    f32_le,
    f32_gt,
    f32_ge,

    f64_eq,
    f64_ne,
    f64_lt,
    f64_le,
    f64_gt,
    f64_ge,

    // conversion
    i32_to_i64,
    i32_to_f32,
    i32_to_f64,

    i64_to_i32,
    i64_to_f32,
    i64_to_f64,

    f32_to_i32,
    f32_to_i64,
    f32_to_f64,

    f64_to_i32,
    f64_to_i64,
    f64_to_f32,

    // control flow
    jump_if,
    always_jump,

    // function call
    call,
    return_,
    return_void,

    // object
    new,
    ref_null,
    get_field,
    set_field,

    invoke,
    // native_invoke,

    // array
    new_array,
    array_load,
    array_store,
    array_length,

    // exception
    // throw,
    // catch,
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

