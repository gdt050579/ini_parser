use super::value::Value;

pub struct KeyValue {
    pub (super) name: String,
    pub (super) value: Value,
}

impl KeyValue {
    pub (super) fn new_u64(name: &str, value: u64) -> KeyValue {
        KeyValue {
            name: String::from(name),
            value: Value::UInt64(value),
        }
    }
    pub (super) fn new_i64(name: &str, value: i64) -> KeyValue {
        KeyValue {
            name: String::from(name),
            value: Value::Int64(value),
        }
    }
    pub (super) fn new_f64(name: &str, value: f64) -> KeyValue {
        KeyValue {
            name: String::from(name),
            value: Value::Float64(value),
        }
    }
    pub (super) fn new_string(name: &str, value: &str) -> KeyValue {
        KeyValue {
            name: String::from(name),
            value: Value::String(String::from(value)),
        }
    }
    pub (super) fn new_bool(name: &str, value: bool) -> KeyValue {
        KeyValue {
            name: String::from(name),
            value: Value::Bool(value),
        }
    }
    pub fn get_name(&self) -> &str {
        return &self.name;
    }
    pub fn get_value(&self) -> &Value {
        return &self.value;
    }
}
