#[derive(Debug,PartialEq)]
pub enum Value {
    Bool(bool),
    String(String),
    UInt64(u64),
    Int64(i64),
    Float64(f64),
    Array(Vec<Value>),
}

pub trait CreateFromValue<T> {
    fn create_from_value(val: &Value, default: T)->T;
}

macro_rules! impl_CreateFromValue_for_numbers {
    ($t:ty) => {
        impl CreateFromValue<$t> for $t {
            fn create_from_value(val: &Value, default: $t)->$t {
                if let Value::UInt64(cval) = val {
                    if (*cval)<=(<$t>::MAX as u64) {
                        return (*cval) as $t;
                    } else {
                        return default;
                    }
                }
                if let Value::Int64(cval) = val {
                    if ((*cval)<=(<$t>::MAX as i64)) && ((*cval)>=(<$t>::MIN as i64)) {
                        return (*cval) as $t;
                    } else {
                        return default;
                    }
                }
                return default;       
            }
        }
    }
}

impl_CreateFromValue_for_numbers!(i8);
impl_CreateFromValue_for_numbers!(i16);
impl_CreateFromValue_for_numbers!(i32);
impl_CreateFromValue_for_numbers!(i64);
impl_CreateFromValue_for_numbers!(u8);
impl_CreateFromValue_for_numbers!(u16);
impl_CreateFromValue_for_numbers!(u32);
impl_CreateFromValue_for_numbers!(u64);

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}
impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self::String(String::from(value))
    }
}
impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}