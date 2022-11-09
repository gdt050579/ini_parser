use std::fmt::Write;
use super::section::Section;
use super::key_value::KeyValue;
use super::hash_utils::*;
use super::Ini;

enum Number {
    Invalid,
    UInt64(u64),
    Int64(i64),
    Float64(f64),
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



pub(super) struct ParserObject<'a> {
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
    pub(super) fn new<'a>(ini: &'a mut Ini, text: &'a str) -> ParserObject<'a> {
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
    fn hex_to_number(&self, start: usize, end: usize, signed: bool) -> Number {
        // this is a hex number
        let mut value = 0u64;
        let mut idx = start;
        while idx < end {
            let ch = self.buf[idx];
            idx += 1;
            match ch {
                b'0'..=b'9' => value = value << 4 + ((ch - b'0') as u64),
                b'A'..=b'F' => value = value << 4 + ((ch - 55) as u64),
                b'a'..=b'f' => value = value << 4 + ((ch - 87) as u64),
                _ => return Number::Invalid,
            }
        }
        if signed {
            return Number::Int64(-(value as i64));
        } else {
            return Number::UInt64(value);
        }
    }
    #[inline]
    fn dec_to_number(&self, start: usize, end: usize, signed: bool) -> Number {
        // this is a decimal number
        let mut supraunitar = 0u64;
        let mut subunitar = 0f64;
        let mut float_idx = 10f64;
        let mut has_subunitar = false;
        let mut idx = start;
        while idx < end {
            let ch = self.buf[idx];
            idx += 1;
            match ch {
                b'0'..=b'9' => {
                    if has_subunitar {
                        subunitar += ((ch - b'0') as f64) / float_idx;
                        float_idx *= 10f64;
                    } else {
                        supraunitar = (supraunitar * 10) + ((ch - b'0') as u64);
                    }
                }
                b'.' => {
                    if has_subunitar {
                        return Number::Invalid;
                    } else {
                        has_subunitar = true;
                    }
                }
                _ => return Number::Invalid,
            }
        }
        if has_subunitar {
            if signed {
                return Number::Float64(-((supraunitar as f64) + subunitar));
            } else {
                return Number::Float64((supraunitar as f64) + subunitar);
            }
        } else {
            if signed {
                return Number::Int64(-(supraunitar as i64));
            } else {
                return Number::UInt64(supraunitar);
            }
        }
    }
    #[inline]
    fn value_to_number(&self, start: usize, end: usize) -> Number {
        // check to see if this is a possible number
        let mut is_negative = false;
        let mut pos = start;
        match self.buf[pos] {
            b'+' => pos += 1,
            b'-' => {
                is_negative = true;
                pos += 1;
            }
            _ => {}
        }
        if pos >= end {
            return Number::Invalid;
        }
        if (pos + 2 < end) && (self.buf[pos] == b'0') && ((self.buf[pos + 1] | 0x20) == b'x') {
            // a hexazecimal number
            return self.hex_to_number(pos + 2, end, is_negative);
        }
        // consider a decimal number
        return self.dec_to_number(pos, end, is_negative);
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
    fn add_value(&mut self, start: usize, end: usize, force_string: bool) {
        if force_string {
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
        let number = self.value_to_number(start, end);
        let c_sect = self.current_section.as_mut().unwrap();
        match number {
            Number::UInt64(value) => c_sect.items.insert(
                self.current_key_hash,
                KeyValue::new_u64(self.current_key.unwrap(), value),
            ),
            Number::Int64(value) => c_sect.items.insert(
                self.current_key_hash,
                KeyValue::new_i64(self.current_key.unwrap(), value),
            ),
            Number::Float64(value) => c_sect.items.insert(
                self.current_key_hash,
                KeyValue::new_f64(self.current_key.unwrap(), value),
            ),
            _ => c_sect.items.insert(
                self.current_key_hash,
                KeyValue::new_string(self.current_key.unwrap(), &self.text[start..end]),
            ),
        };
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
                index -= 1;
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
