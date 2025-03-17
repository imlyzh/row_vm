use super::klass::ValueType;

pub struct ArrayHead {
    pub value_type: ValueType,
    pub length: u32,
}

pub struct Array {
    pub head: ArrayHead,
    pub instance: u8,
}
