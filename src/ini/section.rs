use super::entry::Entry;
use super::hash_utils::*;
use super::key_value::KeyValue;
use super::value::Value;
use std::collections::HashMap;

pub struct Section {
    pub(super) name: String,
    pub(super) items: HashMap<u64, KeyValue>,
}

impl Section {
    pub(super) fn new(name: &str) -> Section {
        Section {
            name: String::from(name),
            items: HashMap::with_capacity(4),
        }
    }
    pub(super) fn new_default() -> Section {
        Section {
            name: String::new(),
            items: HashMap::with_capacity(4),
        }
    }

    #[inline]
    pub fn get_name(&self) -> &str {
        return &self.name;
    }

    #[inline]
    pub fn get_keys_count(&self) -> usize {
        return self.items.len();
    }

    #[inline]
    pub fn has_key(&self, name: &str) -> bool {
        let hash = compute_string_hash(name.as_bytes());
        return self.items.contains_key(&hash);
    }

    pub fn set<T: Into<Value>>(&mut self, key_name: &str, value: T) {
        let hash = compute_string_hash(key_name.as_bytes());
        self.items.insert(
            hash,
            KeyValue {
                name: String::from(key_name),
                value: value.into(),
            },
        );
    }

    pub fn get_value(&self, key_name: &str) -> Option<&Value> {
        let hash = compute_string_hash(key_name.as_bytes());
        let kv = self.items.get(&hash);
        if let Some(value) = kv {
            return Some(&value.value);
        }
        return None;
    }
    pub fn get(&self, key_name: &str) -> Entry {
        let hash = compute_string_hash(key_name.as_bytes());
        Entry {
            data: self.items.get(&hash),
        }
    }
}

impl<'a> IntoIterator for &'a Section {
    type Item = &'a KeyValue;
    type IntoIter = std::collections::hash_map::Values<'a, u64, KeyValue>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.values()
    }
}
