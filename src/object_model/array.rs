use super::klass::ValueType;

pub struct Array {
    pub value_type: ValueType,
    pub length: u32,
    pub instance: u8,
}
