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

macro_rules! impl_from_type_for_value {
    ($t:ty, $result_type: ty, $n:ident) => {
        impl From<$t> for Value {
            fn from(value: $t) -> Self {
                Self::$n(value as $result_type)
            }
        }
    };
}
impl_from_type_for_value!(String,String,String);
impl_from_type_for_value!(bool,bool, Bool);
impl_from_type_for_value!(i8,i64,Int64);
impl_from_type_for_value!(i16,i64,Int64);
impl_from_type_for_value!(i32,i64,Int64);
impl_from_type_for_value!(i64,i64,Int64);
impl_from_type_for_value!(u8,u64,UInt64);
impl_from_type_for_value!(u16,u64,UInt64);
impl_from_type_for_value!(u32,u64,UInt64);
impl_from_type_for_value!(u64,u64,UInt64);
impl_from_type_for_value!(f32,f64,Float64);
impl_from_type_for_value!(f64,f64,Float64);


impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self::String(String::from(value))
    }
}
