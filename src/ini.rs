use std::{
    collections::HashMap,
    fmt::Write,
    ops::{Index, IndexMut},
};

enum NumericalValue {
    UInt64(u64),
    Int64(i64),
    Float64(f64),
}
enum Value {
    Bool(bool),
    String(String),
    Number(NumericalValue),
    Array(Vec<Value>),
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
struct KeyValue {
    name: String,
    value: Value,
}
pub struct Section {
    name: String,
    items: HashMap<u64, KeyValue>,
}
pub struct Ini {
    sections: HashMap<u64, Section>,
    has_default_section: bool,
}

#[derive(PartialEq, Debug)]
enum Status {
    ExpectSectionNameOrKey,
    ExpectAssign,
    ExpectValue,
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
enum CharType {
    Space,
    NewLine,
    String,
    StartSection,
    EndSection,
    Assign,
    Invalid,
    Comment,
    Word,
}

const CHAR_TYPE: [CharType; 256] = [
    CharType::Invalid,
    CharType::Invalid,
    CharType::Invalid,
    CharType::Invalid,
    CharType::Invalid,
    CharType::Invalid,
    CharType::Invalid,
    CharType::Invalid,
    CharType::Invalid,
    CharType::Space,
    CharType::NewLine,
    CharType::Invalid,
    CharType::Invalid,
    CharType::NewLine,
    CharType::Invalid,
    CharType::Invalid,
    CharType::Invalid,
    CharType::Invalid,
    CharType::Invalid,
    CharType::Invalid,
    CharType::Invalid,
    CharType::Invalid,
    CharType::Invalid,
    CharType::Invalid,
    CharType::Invalid,
    CharType::Invalid,
    CharType::Invalid,
    CharType::Invalid,
    CharType::Invalid,
    CharType::Invalid,
    CharType::Invalid,
    CharType::Invalid,
    CharType::Space,
    CharType::Word,
    CharType::String,
    CharType::Comment,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::String,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Assign,
    CharType::Comment,
    CharType::Word,
    CharType::Assign,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::StartSection,
    CharType::Word,
    CharType::EndSection,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
    CharType::Word,
];

const LOWER_CASE_TABLE: [u8; 256] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 97, 98, 99, 100, 101, 102, 103,
    104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122,
    91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111,
    112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130,
    131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149,
    150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168,
    169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187,
    188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206,
    207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225,
    226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244,
    245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255,
];
fn compute_string_hash(buf: &[u8]) -> u64 {
    // use FNV algorithm ==> https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function
    if buf.len() == 0 {
        return 0;
    }
    let mut hash = 0xcbf29ce484222325u64;
    let mut idx = 0usize;
    while idx < buf.len() {
        hash = hash ^ (LOWER_CASE_TABLE[buf[idx] as usize] as u64);
        //hash = hash * 0x00000100000001B3u64;
        hash = hash.wrapping_mul(0x00000100000001B3u64);
        idx += 1;
    }
    return hash;
}
struct ParserObject<'a> {
    status: Status,
    ini: &'a mut Ini,
    pos: usize,
    buf: &'a [u8],
    text: &'a str,
    current_section: Option<Section>,
    current_section_hash: u64,
    current_key: Option<&'a str>,
    current_key_hash: u64,
}

impl ParserObject<'_> {
    pub fn new<'a>(ini: &'a mut Ini, text: &'a str) -> ParserObject<'a> {
        ParserObject {
            status: Status::ExpectSectionNameOrKey,
            ini: ini,
            pos: 0,
            buf: text.as_bytes(),
            text: text,
            current_section: None,
            current_section_hash: 0,
            current_key: None,
            current_key_hash: 0,
        }
    }

    fn build_error_message(&mut self, message: &str, start: usize, _end: usize) -> String {
        let mut result = String::with_capacity(256);
        result.push_str(message);
        result.push_str("\n");
        // we need to compute the line number
        let mut index = 0usize;
        let mut line_number = 1u32;
        while (index < start) && (index < self.buf.len()) {
            if self.get_char_type(index) == CharType::NewLine {
                line_number += 1;
                index += 1;
                if (index < start) && (index < self.buf.len()) {
                    if (self.get_char_type(index) == CharType::NewLine)
                        && (self.get_char_type(index - 1) != self.get_char_type(index))
                    {
                        // either CRLF or LFCR
                        index += 1;
                    }
                }
            } else {
                index += 1;
            }
        }
        result.push_str("Line number: ");
        result.write_fmt(format_args!("{}", line_number)).unwrap();

        result
    }

    #[inline]
    fn get_char_type(&mut self, index: usize) -> CharType {
        CHAR_TYPE[self.buf[index] as usize]
    }
    #[inline]
    fn value_to_bool(&self, start: usize, end: usize) -> Option<bool> {
        let sz = end - start;
        if (sz < 2) || (sz > 5) {
            return None;
        }
        // possible values: TRUE  = true, on, yes
        //                : FALSE = false, off, no
        match self.buf[start] | 0x20u8 {
            b't' => {
                if (sz == 4)
                    && (self.buf[start + 1] | 0x20 == b'r')
                    && (self.buf[start + 2] | 0x20 == b'u')
                    && (self.buf[start + 3] | 0x20 == b'e')
                {
                    return Some(true);
                } // true
            }
            b'y' => {
                if (sz == 3)
                    && (self.buf[start + 1] | 0x20 == b'e')
                    && (self.buf[start + 2] | 0x20 == b's')
                {
                    return Some(true);
                } // yes
            }
            b'o' => {
                match sz {
                    2 => {
                        if self.buf[start + 1] | 0x20 == b'n' {
                            return Some(true);
                        } // on
                    }
                    3 => {
                        if (self.buf[start + 1] | 0x20 == b'f')
                            && (self.buf[start + 2] | 0x20 == b'f')
                        {
                            return Some(false);
                        } // off
                    }
                    _ => {
                        return None;
                    }
                }
            }
            b'n' => {
                if (sz == 2) && ((self.buf[start + 1] | 0x20) == b'o') {
                    return Some(false);
                } // no
            }
            b'f' => {
                if (sz == 5)
                    && (self.buf[start + 1] | 0x20 == b'a')
                    && (self.buf[start + 2] | 0x20 == b'l')
                    && (self.buf[start + 3] | 0x20 == b's')
                    && (self.buf[start + 4] | 0x20 == b'e')
                {
                    return Some(false);
                } // false
            }

            _ => {
                return None;
            }
        }
        return None;
    }
    #[inline]
    fn parse_same_type(&mut self, mut index: usize) -> usize {
        let ctype = self.get_char_type(index);
        while (index < self.buf.len()) && (self.get_char_type(index) == ctype) {
            index += 1
        }
        index
    }
    #[inline]
    fn parse_until_eol(&mut self, mut index: usize) -> usize {
        while (index < self.buf.len()) && (self.get_char_type(index) != CharType::NewLine) {
            index += 1
        }
        index
    }
    #[inline]
    fn parse_section_name(&mut self, mut index: usize) -> Result<usize, String> {
        // assume that we start with an '[' character
        let mut start = index + 1;
        let section_start = index;
        while (start < self.buf.len()) && (self.get_char_type(start) == CharType::Space) {
            start += 1;
        }
        // search first ']' character
        let mut end = start;
        loop {
            if end >= self.buf.len() {
                return Err(self.build_error_message(
                    "Unexpected end of section definition",
                    start,
                    self.buf.len(),
                ));
            }
            let c_type = self.get_char_type(end);
            if (c_type == CharType::Comment) || (c_type == CharType::NewLine) {
                return Err(self.build_error_message(
                    "Expecting a ']' character to finish the section !",
                    start,
                    end,
                ));
            }
            if c_type == CharType::EndSection {
                break;
            }
            end += 1;
        }
        // if we reach this point, end points to an ']'
        index = end + 1; // next char after ']'
        end -= 1;
        while (end > start) && (self.get_char_type(end) == CharType::Space) {
            end -= 1;
        }
        if end == start {
            return Err(self.build_error_message(
                "Empty section (without any name)",
                section_start,
                index,
            ));
        }
        end += 1;
        // now the section name is between start and end
        let section_hash = compute_string_hash(&self.buf[start..end]);
        if self.ini.sections.contains_key(&section_hash) {
            return Err(self.build_error_message(
                "A section with the same name already exists !",
                start,
                end,
            ));
        }
        // move current section to the hash_map (if any)
        self.insert_current_section();
        // create the new section
        self.current_section = Some(Section::new(&self.text[start..end]));
        self.current_section_hash = section_hash;
        Ok(index)
    }

    #[inline]
    fn add_value(&mut self, start: usize, end: usize, stringValue: bool) {
        println!("Value = {}", &self.text[start..end]);
        
        if stringValue {
            let c_sect = self.current_section.as_mut().unwrap();
            c_sect.items.insert(
                self.current_key_hash,
                KeyValue::new_string(self.current_key.unwrap(), &self.text[start..end]),
            );
            return;
        }
        // check if it is a bool value
        if let Some(result) = self.value_to_bool(start, end) {
            let c_sect = self.current_section.as_mut().unwrap();
            c_sect.items.insert(
                self.current_key_hash,
                KeyValue::new_bool(self.current_key.unwrap(), result),
            );
            return;
        }
        // check if it a numerical value

        // if none of the above --> consider a string
    }

    #[inline]
    fn insert_current_section(&mut self) {
        if self.current_section.is_some() {
            let sect = self.current_section.take().unwrap();
            self.ini.sections.insert(self.current_section_hash, sect);
            if self.current_section_hash == 0 {
                self.ini.has_default_section = true;
            }
            self.current_section_hash = 0;
        }
    }

    #[inline]
    fn parse_three_quotes_string(&mut self, mut index: usize) -> Result<usize, String> {
        // we assume that we start with three quotes
        let quote_char = self.buf[index];
        let start = index;
        index += 3;
        while index < self.buf.len() {
            if self.buf[index] == quote_char {
                if (index + 3 <= self.buf.len())
                    && (self.buf[index + 1] == quote_char)
                    && (self.buf[index + 2] == quote_char)
                {
                    return Ok(index + 3);
                }
            }
            index += 1;
        }
        return Err(self.build_error_message(
            "Unexpected end of multi-line string",
            start,
            self.buf.len(),
        ));
    }
    #[inline]
    fn parse_single_quote_string(&mut self, mut index: usize) -> Result<usize, String> {
        // we assume that we start with one quote
        // single quote string is a single line string
        let quote_char = self.buf[index];
        let start = index;
        index += 1;
        while index < self.buf.len() {
            if self.buf[index] == quote_char {
                return Ok(index + 1);
            }
            if self.get_char_type(index) == CharType::NewLine {
                return Err(self.build_error_message(
                    "Unexpected end of single-line string",
                    start,
                    index,
                ));
            }
            index += 1;
        }
        return Err(self.build_error_message(
            "Unexpected end of single-line string",
            start,
            self.buf.len(),
        ));
    }
    #[inline]
    fn parse_string(&mut self, mut index: usize) -> Result<usize, String> {
        // assume we start with a single or double quote
        let quote_char = self.buf[index];
        if (index + 3 <= self.buf.len())
            && (self.buf[index + 1] == quote_char)
            && (self.buf[index + 2] == quote_char)
        {
            let next = self.parse_three_quotes_string(index)?;
            self.add_value(index + 3, next - 3, true);
            index = next;
        } else {
            let next = self.parse_single_quote_string(index)?;
            self.add_value(index + 1, next - 1, true);
            index = next;
        }
        Ok(index)
    }

    #[inline]
    fn parse_word(&mut self, mut index: usize) -> Result<usize, String> {
        // assume that we start with a valid character
        // we should move until the end of the line or until a comment is found
        let start = index;
        while index < self.buf.len() {
            let ch_type = self.get_char_type(index);
            if (ch_type == CharType::NewLine) || (ch_type == CharType::Comment) {
                break;
            }
            index += 1;
        }
        // trim any extra spaces from the end
        while (index > start) && (self.get_char_type(index) == CharType::Space) {
            index -= 1;
        }
        index += 1;
        // now we have a value between start and index
        self.add_value(start, index, false);
        Ok(index)
    }

    #[inline]
    fn parse_key_name(&mut self, index: usize) -> Result<usize, String> {
        let next = self.parse_same_type(index);
        if self.current_section.is_none() {
            self.current_section = Some(Section::new_default());
        }
        let hash = compute_string_hash(&self.buf[index..next]);
        let sect = self.current_section.as_mut().unwrap();
        if sect.items.contains_key(&hash) {
            return Err(self.build_error_message(
                "Key already exists in current section",
                index,
                next,
            ));
        }
        self.current_key = Some(&self.text[index..next]);
        self.current_key_hash = hash;
        self.status = Status::ExpectAssign;
        Ok(next)
    }

    fn parse_for_section_or_key(&mut self) -> Result<(), String> {
        let ch_type = self.get_char_type(self.pos);
        match ch_type {
            CharType::Space => self.pos = self.parse_same_type(self.pos),
            CharType::NewLine => self.pos = self.parse_same_type(self.pos),
            CharType::Comment => self.pos = self.parse_until_eol(self.pos),
            CharType::Word => self.pos = self.parse_key_name(self.pos)?,
            CharType::StartSection => self.pos = self.parse_section_name(self.pos)?,
            _ => {
                return Err(self.build_error_message(
                    "Expecting a a section '[...]' or a key !",
                    self.pos,
                    self.pos + 1,
                ))
            }
        }
        Ok(())
    }
    fn parse_for_assign(&mut self) -> Result<(), String> {
        // skip any possible space
        if self.get_char_type(self.pos) == CharType::Space {
            self.pos = self.parse_same_type(self.pos);
        }
        if self.get_char_type(self.pos) != CharType::Assign {
            return Err(self.build_error_message(
                "Expecting a '=' or ':' character !",
                self.pos,
                self.pos + 1,
            ));
        }
        // all good
        self.pos += 1;
        self.status = Status::ExpectValue;
        Ok(())
    }
    fn parse_for_value(&mut self) -> Result<(), String> {
        // skip any possible space
        if self.get_char_type(self.pos) == CharType::Space {
            self.pos = self.parse_same_type(self.pos);
        }
        match self.get_char_type(self.pos) {
            CharType::String => self.pos = self.parse_string(self.pos)?,
            CharType::Word => self.pos = self.parse_word(self.pos)?,
            _ => {
                return Err(self.build_error_message("Expecting a value", self.pos, self.pos + 1));
            }
        }
        self.status = Status::ExpectSectionNameOrKey;
        Ok(())
    }
    pub fn parse(&mut self) -> Result<(), String> {
        while self.pos < self.buf.len() {
            //println!("Index: {}, status={:?}",self.pos,self.status);
            match self.status {
                Status::ExpectSectionNameOrKey => self.parse_for_section_or_key()?,
                Status::ExpectAssign => self.parse_for_assign()?,
                Status::ExpectValue => self.parse_for_value()?,
            }
        }
        // all good --> insert current section into hash table
        self.insert_current_section();
        Ok(())
    }
}

impl Ini {
    pub fn new() -> Ini {
        Ini {
            sections: HashMap::with_capacity(4),
            has_default_section: true,
        }
    }
    pub fn from<'a>(text: &'a str) -> Result<Ini, String> {
        let mut i = Ini::new();
        let mut p = ParserObject::new(&mut i, text);
        p.parse()?;
        Ok(i)
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

impl KeyValue {
    fn new_string(name: &str, value: &str) -> KeyValue {
        KeyValue {
            name: String::from(name),
            value: Value::String(String::from(value)),
        }
    }
    fn new_bool(name: &str, value: bool) -> KeyValue {
        KeyValue {
            name: String::from(name),
            value: Value::Bool(value),
        }
    }
}

impl Section {
    fn new(name: &str) -> Section {
        Section {
            name: String::from(name),
            items: HashMap::with_capacity(4),
        }
    }
    fn new_default() -> Section {
        Section {
            name: String::new(),
            items: HashMap::with_capacity(4),
        }
    }
    pub fn get_name(&self) -> &str {
        return &self.name;
    }
    pub fn get_key_count(&self) -> usize {
        return self.items.len();
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
}
