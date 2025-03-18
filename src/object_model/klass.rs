use std::{collections::HashMap, ptr::NonNull};

use crate::{method::Method, utils::{align, align_reference}};

pub struct Klass {
    pub klass_name: String,
    pub methods: HashMap<String, Method>,
    // pub static_fields: HashMap<String, Field>,
    pub fields: HashMap<String, Field>,
    pub reference_size: u16,
    pub u64_size: u16,
    pub u32_size: u16,
    pub u16_size: u16,
    pub u8_size: u16,
}

impl Klass {
    pub fn new(klass_name: String) -> Self {
        Self {
            klass_name,
            methods: HashMap::new(),
            // static_fields: HashMap::new(),
            fields: HashMap::new(),
            reference_size: 0,
            u64_size: 0,
            u32_size: 0,
            u16_size: 0,
            u8_size: 0,
        }
    }

    pub fn get_object_size(&self, is_pointer_compression: bool) -> usize {
        let reference_byte_size = if is_pointer_compression {
            self.reference_size as usize * 4
        } else {
            self.reference_size as usize * 8
        };
        let reference_byte_size = align_reference(reference_byte_size);
        let u64_byte_size = self.u64_size as usize * 8;
        let u32_byte_size = self.u64_size as usize * 4;
        let u16_byte_size = self.u64_size as usize * 2;
        let u8_byte_size = self.u64_size as usize;
        let total_byte_size =
            reference_byte_size + u64_byte_size + u32_byte_size + u16_byte_size + u8_byte_size;
        align(total_byte_size, 8)
    }
}

pub struct Field {
    pub value_type: ValueType,
    pub offset: u16,
}

pub enum ValueType {
    Ref64(Option<String>),
    Ref32(Option<String>),
    Array(Box<ValueType>),
    U64,
    I64,
    U32,
    I32,
    U16,
    I16,
    U8,
    I8,
}
