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
            } else {
                index += 1;
            }
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
