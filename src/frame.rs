use crate::object_model::klass::{Field, Klass, ValueType};

pub fn build_frame_klass(mut klass: Klass) -> Klass {
    klass.fields.insert(
        "__prev_frame__".to_owned(),
        Field {
            value_type: ValueType::Reference(None),
            offset: klass.reference_size,
        },
    );
    klass.reference_size += 1;
    klass
}
