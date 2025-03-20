
#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum ByteCode {
    /// ## Numeric instructions

    /// inn.const unn
    i32_const,
    i64_const,
    /// fnn.const fnn
    f32_const,
    f64_const,

    /// inn.iunop
    i32_clz,
    i32_ctz,
    i32_popcnt,
    i64_clz,
    i64_ctz,
    i64_popcnt,

    /// fnn.funop
    f32_abs,
    f32_neg,
    f32_sqrt,
    f32_ceil,
    f32_floor,
    f32_trunc,
    f32_nearest,
    f64_abs,
    f64_neg,
    f64_sqrt,
    f64_ceil,
    f64_floor,
    f64_trunc,
    f64_nearest,

    /// inn.ibinop
    i32_add,
    i32_sub,
    i32_mul,
    i32_div_s,
    i32_div_u,
    i32_rem_s,
    i32_rem_u,
    i32_and,
    i32_or,
    i32_xor,
    i32_shl,
    i32_shr_s,
    i32_shr_u,
    i32_rotl,
    i32_rotr,
    i64_add,
    i64_sub,
    i64_mul,
    i64_div_s,
    i64_div_u,
    i64_rem_s,
    i64_rem_u,
    i64_and,
    i64_or,
    i64_xor,
    i64_shl,
    i64_shr_s,
    i64_shr_u,
    i64_rotl,
    i64_rotr,

    /// fnn.fbinop
    f32_add,
    f32_sub,
    f32_mul,
    f32_div,
    f32_min,
    f32_max,
    f32_copysign,

    /// inn.itestop
    i32_eqz,
    i64_eqz,

    /// inn.irelop
    i32_eq,
    i32_ne,
    i32_lt_s,
    i32_lt_u,
    i32_le_s,
    i32_le_u,
    i32_gt_s,
    i32_gt_u,
    i32_ge_s,
    i32_ge_u,
    i64_eq,
    i64_ne,
    i64_lt_s,
    i64_lt_u,
    i64_le_s,
    i64_le_u,
    i64_gt_s,
    i64_gt_u,
    i64_ge_s,
    i64_ge_u,

    /// fnn.frelop
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

    i32_extend8_s,
    i64_extend8_s,

    i32_extend16_s,
    i64_extend16_s,

    i64_extend32_s,

    i32_wrap_i64,

    i64_extend_i32_s,
    i64_extend_i32_u,

    i32_trunc_f32_s,
    i32_trunc_f32_u,
    i32_trunc_f64_s,
    i32_trunc_f64_u,
    i64_trunc_f32_s,
    i64_trunc_f32_u,
    i64_trunc_f64_s,
    i64_trunc_f64_u,

    i32_trunc_sat_f32_s,
    i32_trunc_sat_f32_u,
    i32_trunc_sat_f64_s,
    i32_trunc_sat_f64_u,
    i64_trunc_sat_f32_s,
    i64_trunc_sat_f32_u,
    i64_trunc_sat_f64_s,
    i64_trunc_sat_f64_u,

    f32_demote_f64,
    f64_promote_f32,

    f32_convert_i32_s,
    f32_convert_i32_u,
    f32_convert_i64_s,
    f32_convert_i64_u,
    f64_convert_i32_s,
    f64_convert_i32_u,
    f64_convert_i64_s,
    f64_convert_i64_u,

    i32_reinterpret_f32,
    i64_reinterpret_f64,
    f32_reinterpret_i32,
    f64_reinterpret_i64,

    /// ref.func is not exist in the spec

    /// # Parametric Instructions
    drop,
    select,

    /// local.get localidx
    local_get,
    /// local.set localidx
    local_set,
    /// local.tee localidx
    // local_tee,

    /// ## control instructions
    nop,
    unreachable,
    if_,

    /// ## Reference instructions
    ref_null,
    ref_is_null,

    /// ### Heap Object Instructions
    new,
    new_array,

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

