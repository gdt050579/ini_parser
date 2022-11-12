use super::key_value::KeyValue;
#[derive(Debug)]
pub struct Entry<'a>
{
    pub(super) data: Option<&'a KeyValue>
}

impl Entry<'_> {    
    pub fn name(&self)-> &str {
        if self.data.is_some() {
            return self.data.clone().unwrap().name.as_str();
        } else {
            return "";
        }
    }
    pub fn exists(&self)->bool {
        self.data.is_some()
    }
}