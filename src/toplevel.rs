use std::{collections::HashMap, ptr::NonNull};

use crate::object_model::klass::Klass;

pub type TopLevel = HashMap<String, NonNull<Klass>>;