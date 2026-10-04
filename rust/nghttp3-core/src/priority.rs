//! Safe parser for the RFC 9218 Priority HTTP field.
//!
//! The Priority field is an RFC 8941 Structured Fields dictionary. This module
//! interprets the standardized `u` (urgency) and `i` (incremental) members,
//! while safely consuming syntactically valid unrecognized members and
//! parameters. The parser uses slice indexing only through checked cursors.

/// Default RFC 9218 urgency.
pub const DEFAULT_URGENCY: u8 = 3;

/// Highest urgency value.
pub const URGENCY_HIGH: u8 = 0;

/// Lowest urgency value.
pub const URGENCY_LOW: u8 = 7;

/// HTTP Priority values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Priority {
    pub urgency: u8,
    pub incremental: bool,
}

impl Default for Priority {
    fn default() -> Self {
        Self {
            urgency: DEFAULT_URGENCY,
            incremental: false,
        }
    }
}

/// Priority-field parse failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParseError {
    Syntax,
    WrongType,
    UrgencyOutOfRange,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Item {
    Integer(i64),
    Boolean(bool),
    Other,
}

struct Parser<'a> {
    input: &'a [u8],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a [u8]) -> Self {
        Self { input, pos: 0 }
    }

    fn peek(&self) -> Option<u8> {
        self.input.get(self.pos).copied()
    }

    fn bump(&mut self) -> Option<u8> {
        let byte = self.peek()?;
        self.pos += 1;
        Some(byte)
    }

    fn consume(&mut self, byte: u8) -> bool {
        if self.peek() == Some(byte) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn skip_spaces(&mut self) {
        while self.peek() == Some(b' ') {
            self.pos += 1;
        }
    }

    fn eof(&self) -> bool {
        self.pos == self.input.len()
    }

    fn parse_key_range(&mut self) -> Result<(usize, usize), ParseError> {
        let start = self.pos;
        let first = self.bump().ok_or(ParseError::Syntax)?;
        if !is_key_first(first) {
            return Err(ParseError::Syntax);
        }

        while self.peek().is_some_and(is_key_rest) {
            self.pos += 1;
        }

        Ok((start, self.pos))
    }

    fn parse_member_value(&mut self) -> Result<Item, ParseError> {
        if self.consume(b'(') {
            self.parse_inner_list()?;
            Ok(Item::Other)
        } else {
            self.parse_bare_item()
        }
    }

    fn parse_bare_item(&mut self) -> Result<Item, ParseError> {
        match self.peek().ok_or(ParseError::Syntax)? {
            b'?' => self.parse_boolean(),
            b'"' => {
                self.parse_string()?;
                Ok(Item::Other)
            }
            b':' => {
                self.parse_byte_sequence()?;
                Ok(Item::Other)
            }
            b'-' | b'0'..=b'9' => self.parse_number(),
            byte if is_token_first(byte) => {
                self.parse_token()?;
                Ok(Item::Other)
            }
            _ => Err(ParseError::Syntax),
        }
    }

    fn parse_boolean(&mut self) -> Result<Item, ParseError> {
        self.bump();
        match self.bump() {
            Some(b'0') => Ok(Item::Boolean(false)),
            Some(b'1') => Ok(Item::Boolean(true)),
            _ => Err(ParseError::Syntax),
        }
    }

    fn parse_number(&mut self) -> Result<Item, ParseError> {
        let negative = self.consume(b'-');
        let mut digits = 0_u8;
        let mut value = 0_i64;

        while let Some(byte @ b'0'..=b'9') = self.peek() {
            if digits == 15 {
                return Err(ParseError::Syntax);
            }
            self.pos += 1;
            digits += 1;
            value = value
                .checked_mul(10)
                .and_then(|v| v.checked_add(i64::from(byte - b'0')))
                .ok_or(ParseError::Syntax)?;
        }

        if digits == 0 {
            return Err(ParseError::Syntax);
        }

        if self.consume(b'.') {
            let mut fraction = 0_u8;
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                if fraction == 3 {
                    return Err(ParseError::Syntax);
                }
                self.pos += 1;
                fraction += 1;
            }
            if fraction == 0 {
                return Err(ParseError::Syntax);
            }
            return Ok(Item::Other);
        }

        Ok(Item::Integer(if negative { -value } else { value }))
    }

    fn parse_string(&mut self) -> Result<(), ParseError> {
        if !self.consume(b'"') {
            return Err(ParseError::Syntax);
        }

        loop {
            match self.bump().ok_or(ParseError::Syntax)? {
                b'"' => return Ok(()),
                b'\\' => match self.bump() {
                    Some(b'"' | b'\\') => {}
                    _ => return Err(ParseError::Syntax),
                },
                0x20..=0x21 | 0x23..=0x5b | 0x5d..=0x7e => {}
                _ => return Err(ParseError::Syntax),
            }
        }
    }

    fn parse_byte_sequence(&mut self) -> Result<(), ParseError> {
        if !self.consume(b':') {
            return Err(ParseError::Syntax);
        }
        while let Some(byte) = self.bump() {
            if byte == b':' {
                return Ok(());
            }
            if !(byte.is_ascii_alphanumeric() || matches!(byte, b'+' | b'/' | b'=')) {
                return Err(ParseError::Syntax);
            }
        }
        Err(ParseError::Syntax)
    }

    fn parse_token(&mut self) -> Result<(), ParseError> {
        let first = self.bump().ok_or(ParseError::Syntax)?;
        if !is_token_first(first) {
            return Err(ParseError::Syntax);
        }
        while self.peek().is_some_and(is_token_rest) {
            self.pos += 1;
        }
        Ok(())
    }

    fn parse_inner_list(&mut self) -> Result<(), ParseError> {
        self.skip_spaces();
        if self.consume(b')') {
            return self.parse_parameters();
        }

        loop {
            self.parse_bare_item()?;
            self.parse_parameters()?;

            match self.peek() {
                Some(b')') => {
                    self.pos += 1;
                    return self.parse_parameters();
                }
                Some(b' ') => self.skip_spaces(),
                _ => return Err(ParseError::Syntax),
            }
        }
    }

    fn parse_parameters(&mut self) -> Result<(), ParseError> {
        while self.consume(b';') {
            self.skip_spaces();
            self.parse_key_range()?;

            if self.consume(b'=') {
                // Historical bug aed3107 advanced beyond the end here. The
                // checked bare-item parser rejects a dangling '=' immediately.
                self.parse_bare_item()?;
            }
        }
        Ok(())
    }
}

fn is_key_first(byte: u8) -> bool {
    byte.is_ascii_lowercase() || byte == b'*'
}

fn is_key_rest(byte: u8) -> bool {
    is_key_first(byte)
        || byte.is_ascii_digit()
        || matches!(byte, b'_' | b'-' | b'.')
}

fn is_token_first(byte: u8) -> bool {
    byte.is_ascii_alphabetic() || byte == b'*'
}

fn is_token_rest(byte: u8) -> bool {
    is_token_first(byte)
        || byte.is_ascii_digit()
        || matches!(
            byte,
            b'!' | b'#'
                | b'$'
                | b'%'
                | b'&'
                | b'\''
                | b'+'
                | b'-'
                | b'.'
                | b'^'
                | b'_'
                | b'`'
                | b'|'
                | b'~'
                | b':'
                | b'/'
        )
}

/// Parses a Priority field while preserving values for members that are absent.
pub fn parse(input: &[u8], initial: Priority) -> Result<Priority, ParseError> {
    let mut parser = Parser::new(input);
    let mut priority = initial;

    parser.skip_spaces();
    if parser.eof() {
        return Ok(priority);
    }

    loop {
        let (key_start, key_end) = parser.parse_key_range()?;
        let item = if parser.consume(b'=') {
            parser.parse_member_value()?
        } else {
            Item::Boolean(true)
        };
        parser.parse_parameters()?;

        let key = &parser.input[key_start..key_end];
        match key {
            b"u" => match item {
                Item::Integer(value) if (i64::from(URGENCY_HIGH)..=i64::from(URGENCY_LOW)).contains(&value) => {
                    priority.urgency = value as u8;
                }
                Item::Integer(_) => return Err(ParseError::UrgencyOutOfRange),
                _ => return Err(ParseError::WrongType),
            },
            b"i" => match item {
                Item::Boolean(value) => priority.incremental = value,
                _ => return Err(ParseError::WrongType),
            },
            _ => {}
        }

        parser.skip_spaces();
        if parser.eof() {
            return Ok(priority);
        }
        if !parser.consume(b',') {
            return Err(ParseError::Syntax);
        }
        parser.skip_spaces();
        if parser.eof() {
            return Err(ParseError::Syntax);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_matches_c(input: &[u8], initial: Priority) {
        let rust = parse(input, initial);
        let c = nghttp3::parse_priority_oracle(
            input,
            nghttp3::Priority {
                urgency: initial.urgency,
                incremental: initial.incremental,
            },
        );

        match (rust, c) {
            (Ok(rust), Ok(c)) => {
                assert_eq!(rust.urgency, c.urgency, "input {input:?}");
                assert_eq!(rust.incremental, c.incremental, "input {input:?}");
            }
            (Err(_), Err(_)) => {}
            pair => panic!("Rust/C priority result mismatch for {input:?}: {pair:?}"),
        }
    }

    #[test]
    fn canonical_priority_values_match_c() {
        for input in [
            &b""[..],
            &b"u=0"[..],
            &b"u=7"[..],
            &b"i"[..],
            &b"i=?0"[..],
            &b"u=2, i"[..],
            &b"foo=bar, u=4"[..],
            &b"u=5;foo=?1"[..],
        ] {
            assert_matches_c(input, Priority::default());
        }
    }

    #[test]
    fn preserves_initial_values_for_absent_members() {
        let initial = Priority {
            urgency: 6,
            incremental: true,
        };
        assert_matches_c(b"foo=bar", initial);
        assert_eq!(parse(b"foo=bar", initial), Ok(initial));
    }

    #[test]
    fn historical_trailing_equals_regression_aed3107() {
        assert!(parse(b"u=", Priority::default()).is_err());
        assert!(parse(b"x=?1;foo=", Priority::default()).is_err());
        assert_matches_c(b"u=", Priority::default());
        assert_matches_c(b"x=?1;foo=", Priority::default());
    }

    #[test]
    fn rejects_wrong_priority_types_and_ranges() {
        assert_eq!(
            parse(b"u=?1", Priority::default()),
            Err(ParseError::WrongType)
        );
        assert_eq!(
            parse(b"u=8", Priority::default()),
            Err(ParseError::UrgencyOutOfRange)
        );
        assert_eq!(
            parse(b"i=1", Priority::default()),
            Err(ParseError::WrongType)
        );
    }
}

#[cfg(kani)]
mod verification {
    use super::*;

    #[kani::proof]
    fn historical_two_byte_trailing_equals_is_rejected() {
        assert!(parse(b"u=", Priority::default()).is_err());
    }

    #[kani::proof]
    fn symbolic_single_key_trailing_equals_never_reads_past_end() {
        let key: u8 = kani::any();
        kani::assume(is_key_first(key));
        let input = [key, b'='];
        assert!(parse(&input, Priority::default()).is_err());
    }

    #[kani::proof]
    fn every_standard_urgency_roundtrips() {
        let urgency: u8 = kani::any();
        kani::assume(urgency <= URGENCY_LOW);

        let input = [b'u', b'=', b'0' + urgency];
        let parsed = parse(&input, Priority::default()).unwrap();
        assert_eq!(parsed.urgency, urgency);
    }

    #[kani::proof]
    fn boolean_incremental_forms_are_total() {
        let bit: bool = kani::any();
        let input = [b'i', b'=', b'?', if bit { b'1' } else { b'0' }];
        let parsed = parse(&input, Priority::default()).unwrap();
        assert_eq!(parsed.incremental, bit);
    }
}
