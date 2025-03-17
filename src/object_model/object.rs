use std::ptr::NonNull;

use super::klass::Klass;

pub struct GcInfo {
    pub is_moved: bool,
    // pub age: u8,
}

pub struct ObjectHead {
    pub klass: NonNull<Klass>,
    pub gc_info: GcInfo,
}

pub struct Object {
    pub head: ObjectHead,
    pub instance: u8,
}
