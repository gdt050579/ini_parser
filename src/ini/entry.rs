use super::key_value::KeyValue;
use super::value::CreateFromValue;

#[derive(Debug)]
pub struct Entry<'a> {
    pub(super) data: Option<&'a KeyValue>,
}
impl Entry<'_> {
    pub fn name(&self) -> &str {
        if self.data.is_some() {
            return self.data.clone().unwrap().name.as_str();
        } else {
            return "";
        }
    }
    pub fn exists(&self) -> bool {
        self.data.is_some()
    }
    pub fn or_else<T: CreateFromValue<T>>(&self, default: T) -> T {
        if self.data.is_none() {
            return default;
        }
        return T::create_from_value(self.data.clone().unwrap().get_value(), default);        
    }
}
