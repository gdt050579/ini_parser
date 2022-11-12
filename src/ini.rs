mod error;
mod hash_utils;
mod key_value;
mod parser;
mod section;
mod tests;
mod value;
mod entry;

use self::error::Error;
use self::hash_utils::*;
use self::parser::*;
use self::section::Section;
use self::value::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::ops::{Index, IndexMut};
use std::path::Path;

pub struct Ini {
    sections: HashMap<u64, Section>,
    has_default_section: bool,
}

impl Ini {
    pub fn new() -> Ini {
        Ini {
            sections: HashMap::with_capacity(4),
            has_default_section: true,
        }
    }
    pub fn from<'a>(text: &'a str) -> Result<Ini, Error> {
        let mut i = Ini::new();
        let mut p = ParserObject::new(&mut i, text);
        p.parse()?;
        Ok(i)
    }
    pub fn from_file(path: &Path) -> Result<Ini, Error> {
        let result = File::open(path);
        if result.is_err() {
            return Err(Error::from_io_error(
                "Fail to open file",
                path,
                result.err().unwrap(),
            ));
        } else {
            let mut f = result.unwrap();
            let mut text = String::with_capacity(4096);
            let result = f.read_to_string(&mut text);
            if result.is_ok() {
                return Ini::from(text.as_str());
            } else {
                return Err(Error::from_io_error(
                    "Fail to read file content",
                    path,
                    result.err().unwrap(),
                ));
            }
        }
    }

    #[inline]
    pub fn has_section(&self, name: &str) -> bool {
        let hash = compute_string_hash(name.as_bytes());
        return self.sections.contains_key(&hash);
    }

    #[inline]
    pub fn has_default_section(&self) -> bool {
        return self.has_default_section;
    }

    #[inline]
    pub fn get_mut_section(&mut self, name: &str) -> Option<&mut Section> {
        let hash = compute_string_hash(name.as_bytes());
        return self.sections.get_mut(&hash);
    }

    #[inline]
    pub fn get_section(&self, name: &str) -> Option<&Section> {
        let hash = compute_string_hash(name.as_bytes());
        return self.sections.get(&hash);
    }

    #[inline]
    pub fn get_default_section(&self) -> Option<&Section> {
        return self.sections.get(&0);
    }

    #[inline]
    pub fn get_mut_default_section(&mut self) -> Option<&mut Section> {
        return self.sections.get_mut(&0);
    }

    #[inline]
    pub fn get_sections_count(&self, ignore_default_section: bool) -> usize {
        if ignore_default_section && self.has_default_section {
            return self.sections.len() - 1;
        } else {
            return self.sections.len();
        }
    }

    #[inline]
    pub fn get_value(&self, section_name: &str, key_name: &str) -> Option<&Value> {
        if let Some(s) = self.get_section(section_name) {
            return s.get_value(key_name);
        } else {
            return None;
        }
    }
}

impl<'a> IntoIterator for &'a Ini {
    type Item = &'a Section;
    type IntoIter = std::collections::hash_map::Values<'a, u64, Section>;

    fn into_iter(self) -> Self::IntoIter {
        self.sections.values()
    }
}

impl Index<&str> for Ini {
    type Output = Section;

    fn index(&self, index: &str) -> &Self::Output {
        let hash = compute_string_hash(index.as_bytes());
        let res = self.sections.get(&hash);
        if res.is_none() {
            panic!("Section {} is not found in the ini list of sections", index);
        }
        res.unwrap()
    }
}

impl IndexMut<&str> for Ini {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        let hash = compute_string_hash(index.as_bytes());
        return self.sections.entry(hash).or_insert(Section::new(index));
    }
}
