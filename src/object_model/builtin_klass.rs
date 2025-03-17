use crate::object_model::klass::{Field, Klass, ValueType};

pub fn build_frame_klass(mut klass: Klass) -> Klass {
    klass.fields.insert(
        "__prev_frame__".to_owned(),
        Field {
            value_type: ValueType::Ref64(None),
            offset: klass.reference_size,
        },
    );
    klass.reference_size += 1;
    klass
}

pub fn build_proto_object_klass() -> Klass {
    let mut klass = Klass::new("Proto".to_owned());
    klass.fields.insert(
        "values".to_owned(),
        Field {
            value_type: ValueType::Array(Box::new(ValueType::Ref64(None))),
            offset: klass.reference_size,
        },
    );
    klass.fields.insert(
        "keys".to_owned(),
        Field {
            value_type: ValueType::Array(Box::new(ValueType::Ref64(Some("String".to_owned())))),
            offset: klass.reference_size,
        },
    );
    klass
}
