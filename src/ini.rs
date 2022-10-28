use std::{collections::HashMap, fmt::Write};
struct KeyValue {
    key: String,
}
struct Section {
    name: String,
    items: HashMap<u64, KeyValue>,
}
pub struct Ini {}

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

struct ParserObject<'a> {
    status: Status,
    ini: &'a mut Ini,
    pos: usize,
    buf: &'a [u8],
    text: &'a str,
}

impl ParserObject<'_> {
    pub fn new<'a>(ini: &'a mut Ini, text: &'a str) -> ParserObject<'a> {
        ParserObject {
            status: Status::ExpectSectionNameOrKey,
            ini: ini,
            pos: 0,
            buf: text.as_bytes(),
            text: text,
        }
    }

    fn build_error_message(&mut self, message: &str, start: usize, end: usize) -> String {
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
        println!("Section found: {}", &self.text[start..end]);
        Ok(index)
    }

    #[inline]
    fn add_value(&mut self, start: usize, end: usize) {
        println!("Value = {}",&self.text[start..end]);
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
            self.add_value(index+3,next-3);
            index = next;
        } else {
            let next = self.parse_single_quote_string(index)?;
            self.add_value(index+1,next-1);
            index = next;
        }
        Ok(index)
    }

    #[inline]
    fn parse_word(&mut self, mut index: usize) -> Result<usize, String> {
        // assume that we start with a valid character
        // we should move until the end of the line or until a comment is found
        let start = index;
        while index<self.buf.len() {
            let ch_type = self.get_char_type(index);
            if (ch_type == CharType::NewLine) || (ch_type == CharType::Comment) {
                break;
            }
            index+=1;
        }
        // trim any extra spaces from the end
        while (index>start) && (self.get_char_type(index)==CharType::Space) {
            index-=1;
        }
        index+=1;
        // now we have a value between start and index
        self.add_value(start, index);
        Ok(index)
    }

    #[inline]
    fn parse_key_name(&mut self, index: usize)-> Result<usize, String> {
        let next = self.parse_same_type(index);
        println!("key={}",&self.text[index..next]);
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
        Ok(())
    }
}

impl Ini {
    pub fn new() -> Ini {
        Ini {}
    }
    pub fn from<'a>(text: &'a str) -> Result<Ini, String> {
        let mut i = Ini::new();
        let mut p = ParserObject::new(&mut i, text);
        p.parse()?;
        Ok(i)
    }
}
