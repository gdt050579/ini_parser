use std::{fmt::Write, path::Path};

#[derive(Debug)]
pub struct Error {
    line: u32,
    message: String,
}

impl Error {
    fn is_eol_char(value: u8) -> bool {
        return (value == 10) || (value == 13);
    }
    pub(super) fn from_parser(msg: &str, start: usize, end: usize, buf: &[u8]) -> Self {
        let mut err = Error {
            line: 0,
            message: String::with_capacity(256),
        };
        err.message.push_str(msg);
        err.message.push_str("\n");
        // we need to compute the line number
        let mut index = 0usize;
        let mut line_number = 1u32;
        let mut line_start = 0usize;
        while (index < start) && (index < buf.len()) {
            if Error::is_eol_char(buf[index]) {
                line_number += 1;
                index += 1;
                if (index < start) && (index < buf.len()) {
                    if (Error::is_eol_char(buf[index])) && (buf[index - 1] != buf[index]) {
                        // either CRLF or LFCR
                        index += 1;
                    }
                }
                line_start = index;
            } else {
                index += 1;
            }
        }
        index = line_start;
        while (index < buf.len()) && (!Error::is_eol_char(buf[index])) {
            index+=1;
        }
        if let Ok(txt) = std::str::from_utf8(&buf[line_start..index])
        {
            // add text to string, but convert tabs (\t) into spaces
            for ch in txt.chars() {
                if ch=='\t' {
                    err.message.push(' ');
                } else {
                    err.message.push(ch);
                }
            }
            err.message.push('\n');
            // mark the error
            let min_ofs = start - line_start;
            let max_ofs = end - line_start;
            for (index,_) in txt.char_indices() {
                if (index>=min_ofs) && (index<max_ofs) {
                    err.message.push('^');
                } else {
                    err.message.push(' ');
                }
            }
            err.message.push('\n');
        } else {
            err.message.push_str("Error: <Fail to convert INI content to UTF-8>\n");
        }
        err.message.push_str("Line number: ");
        err.message
            .write_fmt(format_args!("{}\n", line_number))
            .unwrap();
        err.line = line_number;

        err
    }
    pub(super) fn from_io_error(msg: &str, file_path: &Path, io_error: std::io::Error) -> Self {
        let mut err = Error {
            line: 0,
            message: String::with_capacity(256),
        };
        err.message.push_str(msg);
        err.message.push_str("\n");

        if let Some(fname) = file_path.to_str() {
            err.message.push_str("File: ");
            err.message.push_str(fname);
            err.message.push_str("\n");
        } else {
            err.message
                .push_str("File: <failed to convert file name to a valid UTF-8 string>\n");
        }

        if let Some(io_error_code) = io_error.raw_os_error() {
            err.message
                .write_fmt(format_args!("IO Error Code: {}\n", io_error_code))
                .unwrap();
        }
        err.message
            .write_fmt(format_args!("IO Error: {}\n", io_error.to_string()))
            .unwrap();

        err
    }
    #[inline]
    pub fn get_error_message(&self) -> &str {
        return self.message.as_str();
    }
    #[inline]
    pub fn get_line_number(&self) -> u32 {
        return self.line;
    }
}
