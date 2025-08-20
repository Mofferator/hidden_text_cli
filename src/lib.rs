use std::{num::ParseIntError};


const ZWNJ: char = '\u{200c}';
const INVSEP: char = '\u{2063}';

pub fn encode_hidden(text: String, low_char: Option<char>, high_char: Option<char>) -> Option<String> {
    if text.is_empty() {
        return None
    }

    let mut output = String::new();
    let binary: String = text.as_bytes()
        .iter()
        .map(|b| format!("{b:08b}"))
        .collect::<Vec<String>>()
        .join("");
    for c in binary.chars() {
        match c {
            '0' => output.push(low_char.unwrap_or(ZWNJ)),
            '1' => output.push(high_char.unwrap_or(INVSEP)),
            _ => {}
        }
    }
    Some(output)
}

pub fn decode_hidden(text: String, low_char: Option<char>, high_char: Option<char>) -> Result<Option<String>, DecodeError> {
    let mut hidden_chars = String::new();
    for c in text.chars() {
        match c {
            _ if c == low_char.unwrap_or(ZWNJ) => hidden_chars.push('0'),
            _ if c == high_char.unwrap_or(INVSEP) => hidden_chars.push('1'),
            _ => {}
        }
    };
    if hidden_chars.len() % 8 != 0 {
        return Err(DecodeError { kind: DecodeErrorKind::IncorrectLength(hidden_chars.len()) })
    } else if hidden_chars.is_empty() {
        return Ok(None)
    } else {
        let u8s: Vec<Result<u8, ParseIntError>> = (0..hidden_chars.len())
            .step_by(8)
            .map(|i| u8::from_str_radix(&hidden_chars[i..i + 8], 2))
            .collect();
        let mut output = String::new();
        for num in u8s {
            match num {
                Ok(u8) => output.push(u8 as char),
                Err(_) => { return Err(DecodeError { kind: DecodeErrorKind::InvalidInt }) }
            }
        };
        Ok(Some(output))
    }


}

#[derive(Debug)]
pub enum DecodeErrorKind {
    IncorrectLength(usize),
    InvalidChar(u8),
    InvalidInt
}

#[derive(Debug)]
pub struct DecodeError {
    kind: DecodeErrorKind
}

impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            DecodeErrorKind::IncorrectLength(size) 
                => write!(f, "Number of invisible characters must be divisible by 8, length was {size}"),
            DecodeErrorKind::InvalidChar(u8)
                => write!(f, "Could not interpret {u8} as char"),
            DecodeErrorKind::InvalidInt
                => write!(f, "Failed to parse int")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode() {
        // input strings are generated using https://www.promptfoo.dev/blog/invisible-unicode-threats/
        let input = String::from("This is visible​‌⁣⁣⁣‌⁣‌‌‌⁣⁣‌⁣‌‌‌‌⁣⁣‌⁣‌‌⁣‌⁣⁣⁣‌‌⁣⁣‌‌⁣‌‌‌‌‌‌⁣⁣‌⁣‌‌⁣‌⁣⁣⁣‌‌⁣⁣‌‌⁣‌‌‌‌‌‌⁣⁣‌⁣‌‌‌‌⁣⁣‌⁣‌‌⁣‌⁣⁣‌‌⁣‌‌‌⁣⁣‌‌⁣‌‌‌⁣⁣‌‌⁣‌⁣‌⁣⁣‌⁣⁣⁣‌‍"); //  this contains the text "this is hidden"
        let result = decode_hidden(input, None, None);
        assert!(matches!(result, Ok(Some(text)) if text == "this is hidden"));
    }

    #[test]
    fn decode_empty() {
        let input = String::from("Totally normal message with no hidden text");
        let result = decode_hidden(input, None, None);
        assert!(matches!(result, Ok(None)));
    }

    #[test]
    fn faulty_decode() {
        // this is the same text as before but with three characters removed
        // a 14 char input string ("this is hidden") would need 112 hidden chars, but this string has 109
        let malformed_input = String::from("This is visible​‌⁣⁣⁣‌⁣‌‌‌⁣⁣‌⁣‌‌‌‌⁣⁣‌⁣‌‌⁣‌⁣⁣⁣‌‌⁣⁣‌‌⁣‌‌‌‌‌‌⁣⁣‌⁣‌‌⁣‌⁣⁣⁣‌‌⁣⁣‌‌⁣‌‌‌‌‌‌⁣⁣‌⁣‌‌‌‌⁣⁣‌⁣‌‌⁣‌⁣⁣‌‌⁣‌‌‌⁣⁣‌‌⁣‌‌‌⁣⁣‌‌⁣‌⁣‌⁣⁣⁣‌‍"); 
        let result = decode_hidden(malformed_input, None, None);
        assert!(matches!(result, Err(DecodeError { kind: DecodeErrorKind::IncorrectLength(109) })));
    }

    #[test]
    fn decode_custom_chars() {
        let input = String::from("01100001");
        let result = decode_hidden(input, Some('0'), Some('1'));
        assert!(matches!(result, Ok(Some(text)) if text == "a"));
    }

    #[test]
    fn encode() {
        let input = String::from("this is hidden");
        let result = encode_hidden(input, None, None);
        assert!(matches!(result, Some(text) if text == "‌⁣⁣⁣‌⁣‌‌‌⁣⁣‌⁣‌‌‌‌⁣⁣‌⁣‌‌⁣‌⁣⁣⁣‌‌⁣⁣‌‌⁣‌‌‌‌‌‌⁣⁣‌⁣‌‌⁣‌⁣⁣⁣‌‌⁣⁣‌‌⁣‌‌‌‌‌‌⁣⁣‌⁣‌‌‌‌⁣⁣‌⁣‌‌⁣‌⁣⁣‌‌⁣‌‌‌⁣⁣‌‌⁣‌‌‌⁣⁣‌‌⁣‌⁣‌⁣⁣‌⁣⁣⁣‌")); // result generated from https://www.promptfoo.dev/blog/invisible-unicode-threats/
    }

    #[test]
    fn encode_empty() {
        let input = String::from("");
        let result = encode_hidden(input, None, None);
        assert!(matches!(result, None));
    }

    #[test]
    fn encode_custom_chars() {
        let input = String::from("a");
        let result = encode_hidden(input, Some('0'), Some('1'));
        assert!(matches!(result, Some(text) if text == "01100001"));
    }
}

