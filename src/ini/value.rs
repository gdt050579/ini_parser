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

impl CreateFromValue<i32> for i32 {
    fn create_from_value(val: &Value, default: i32)->i32 {
        if let Value::UInt64(cval) = val {
            if (*cval)<=(i32::MAX as u64) {
                return (*cval) as i32;
            } else {
                return default;
            }
        }
        if let Value::Int64(cval) = val {
            if ((*cval)<=(i32::MAX as i64)) && ((*cval)>=(i32::MIN as i64)) {
                return (*cval) as i32;
            } else {
                return default;
            }
        }
        return default;       
    }
}

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