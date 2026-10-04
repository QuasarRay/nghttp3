//! Safe Structured Fields slice used by HTTP Priority.
//!
//! This incrementally replaces the historical pointer-based parser with a
//! bounds-checked slice cursor. Unsupported advanced Structured Fields
//! constructs are rejected rather than guessed.

pub const DEFAULT_URGENCY: u8 = 3;
pub const URGENCY_HIGH: u8 = 0;
pub const URGENCY_LOW: u8 = 7;

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParseError {
    UnexpectedEnd,
    InvalidSyntax,
    TrailingEquals,
    InvalidUrgency,
    InvalidIncremental,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Key {
    Urgency,
    Incremental,
    Other,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BareItem {
    Integer(i64),
    Boolean(bool),
    Other,
}

struct Cursor<'a> {
    input: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    const fn new(input: &'a [u8]) -> Self {
        Self { input, pos: 0 }
    }

    fn is_empty(&self) -> bool {
        self.pos == self.input.len()
    }

    fn peek(&self) -> Option<u8> {
        self.input.get(self.pos).copied()
    }

    fn take(&mut self) -> Option<u8> {
        let value = self.peek()?;
        self.pos += 1;
        Some(value)
    }

    fn take_if(&mut self, expected: u8) -> bool {
        if self.peek() == Some(expected) {
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

    fn parse_key(&mut self) -> Result<Key, ParseError> {
        let start = self.pos;
        let first = self.take().ok_or(ParseError::UnexpectedEnd)?;
        if !is_key_first(first) {
            return Err(ParseError::InvalidSyntax);
        }

        while self.peek().is_some_and(is_key_rest) {
            self.pos += 1;
        }

        Ok(match &self.input[start..self.pos] {
            b"u" => Key::Urgency,
            b"i" => Key::Incremental,
            _ => Key::Other,
        })
    }

    fn parse_integer(&mut self) -> Result<i64, ParseError> {
        let negative = self.take_if(b'-');
        let mut seen = false;
        let mut value = 0_i64;

        while let Some(byte @ b'0'..=b'9') = self.peek() {
            seen = true;
            self.pos += 1;
            value = value
                .checked_mul(10)
                .and_then(|v| v.checked_add(i64::from(byte - b'0')))
                .ok_or(ParseError::InvalidSyntax)?;
        }

        if !seen {
            return Err(ParseError::InvalidSyntax);
        }

        if negative {
            value.checked_neg().ok_or(ParseError::InvalidSyntax)
        } else {
            Ok(value)
        }
    }

    fn parse_quoted(&mut self) -> Result<BareItem, ParseError> {
        debug_assert_eq!(self.take(), Some(b'"'));
        loop {
            match self.take().ok_or(ParseError::UnexpectedEnd)? {
                b'"' => return Ok(BareItem::Other),
                b'\\' => match self.take().ok_or(ParseError::UnexpectedEnd)? {
                    b'"' | b'\\' => {}
                    _ => return Err(ParseError::InvalidSyntax),
                },
                0x20..=0x7e => {}
                _ => return Err(ParseError::InvalidSyntax),
            }
        }
    }

    fn parse_binary(&mut self) -> Result<BareItem, ParseError> {
        debug_assert_eq!(self.take(), Some(b':'));
        loop {
            match self.take().ok_or(ParseError::UnexpectedEnd)? {
                b':' => return Ok(BareItem::Other),
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'+' | b'/' | b'=' => {}
                _ => return Err(ParseError::InvalidSyntax),
            }
        }
    }

    fn parse_token(&mut self) -> Result<BareItem, ParseError> {
        let start = self.pos;
        while self.peek().is_some_and(is_token_byte) {
            self.pos += 1;
        }
        if self.pos == start {
            Err(ParseError::InvalidSyntax)
        } else {
            Ok(BareItem::Other)
        }
    }

    fn parse_bare_item(&mut self) -> Result<BareItem, ParseError> {
        match self.peek().ok_or(ParseError::UnexpectedEnd)? {
            b'?' => {
                self.pos += 1;
                match self.take().ok_or(ParseError::UnexpectedEnd)? {
                    b'0' => Ok(BareItem::Boolean(false)),
                    b'1' => Ok(BareItem::Boolean(true)),
                    _ => Err(ParseError::InvalidSyntax),
                }
            }
            b'-' | b'0'..=b'9' => self.parse_integer().map(BareItem::Integer),
            b'"' => self.parse_quoted(),
            b':' => self.parse_binary(),
            b'@' => {
                self.pos += 1;
                self.parse_integer().map(|_| BareItem::Other)
            }
            _ => self.parse_token(),
        }
    }

    fn parse_params(&mut self) -> Result<(), ParseError> {
        while self.take_if(b';') {
            let _ = self.parse_key()?;
            if self.take_if(b'=') {
                if self.is_empty() {
                    return Err(ParseError::TrailingEquals);
                }
                let _ = self.parse_bare_item()?;
            }
        }
        Ok(())
    }
}

fn is_key_first(byte: u8) -> bool {
    byte.is_ascii_lowercase() || byte == b'*'
}

fn is_key_rest(byte: u8) -> bool {
    is_key_first(byte) || byte.is_ascii_digit() || matches!(byte, b'_' | b'-' | b'.')
}

fn is_token_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric()
        || matches!(
            byte,
            b'!' | b'#'
                | b'$'
                | b'%'
                | b'&'
                | 0x27
                | b'*'
                | b'+'
                | b'-'
                | b'.'
                | b'^'
                | b'_'
                | 0x60
                | b'|'
                | b'~'
                | b'/'
                | b':'
        )
}

pub fn parse_priority(input: &[u8], initial: Priority) -> Result<Priority, ParseError> {
    let mut cursor = Cursor::new(input);
    let mut priority = initial;

    cursor.skip_spaces();
    if cursor.is_empty() {
        return Ok(priority);
    }

    loop {
        let key = cursor.parse_key()?;
        let value = if cursor.take_if(b'=') {
            if cursor.is_empty() {
                return Err(ParseError::TrailingEquals);
            }
            Some(cursor.parse_bare_item()?)
        } else {
            None
        };

        match key {
            Key::Urgency => match value {
                Some(BareItem::Integer(value))
                    if i64::from(URGENCY_HIGH) <= value && value <= i64::from(URGENCY_LOW) =>
                {
                    priority.urgency =
                        u8::try_from(value).map_err(|_| ParseError::InvalidUrgency)?;
                }
                _ => return Err(ParseError::InvalidUrgency),
            },
            Key::Incremental => match value {
                None => priority.incremental = true,
                Some(BareItem::Boolean(value)) => priority.incremental = value,
                _ => return Err(ParseError::InvalidIncremental),
            },
            Key::Other => {}
        }

        cursor.parse_params()?;
        cursor.skip_spaces();
        if cursor.is_empty() {
            break;
        }
        if !cursor.take_if(b',') {
            return Err(ParseError::InvalidSyntax);
        }
        cursor.skip_spaces();
        if cursor.is_empty() {
            return Err(ParseError::UnexpectedEnd);
        }
    }

    Ok(priority)
}

pub fn parse_item_with_params(input: &[u8]) -> Result<(), ParseError> {
    let mut cursor = Cursor::new(input);
    let _ = cursor.parse_bare_item()?;
    cursor.parse_params()?;
    cursor.skip_spaces();
    if cursor.is_empty() {
        Ok(())
    } else {
        Err(ParseError::InvalidSyntax)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_known_priority_members() {
        assert_eq!(
            parse_priority(b"u=2, i", Priority::default()),
            Ok(Priority {
                urgency: 2,
                incremental: true
            })
        );
        assert_eq!(
            parse_priority(b"i=?0, u=7", Priority::default()),
            Ok(Priority {
                urgency: 7,
                incremental: false
            })
        );
    }

    #[test]
    fn trailing_equals_is_rejected() {
        assert_eq!(
            parse_priority(b"u=", Priority::default()),
            Err(ParseError::TrailingEquals)
        );
        assert_eq!(
            parse_item_with_params(b"?1;foo="),
            Err(ParseError::TrailingEquals)
        );
    }

    #[test]
    fn urgency_domain_is_exact() {
        for urgency in URGENCY_HIGH..=URGENCY_LOW {
            let input = [b'u', b'=', b'0' + urgency];
            assert_eq!(
                parse_priority(&input, Priority::default()).unwrap().urgency,
                urgency
            );
        }
        assert_eq!(
            parse_priority(b"u=8", Priority::default()),
            Err(ParseError::InvalidUrgency)
        );
    }

    #[test]
    fn differential_against_current_c_priority_oracle() {
        for input in [
            b"u=0".as_slice(),
            b"u=7, i".as_slice(),
            b"i=?0, u=2".as_slice(),
        ] {
            let rust = parse_priority(input, Priority::default()).unwrap();
            let c =
                nghttp3::parse_priority_oracle(input, nghttp3::PriorityValue::default()).unwrap();
            assert_eq!(u32::from(rust.urgency), c.urgency);
            assert_eq!(rust.incremental, c.incremental);
        }

        assert!(nghttp3::parse_priority_oracle(b"u=", nghttp3::PriorityValue::default()).is_err());
    }
}

#[cfg(kani)]
mod verification {
    use super::*;

    #[kani::proof]
    fn priority_trailing_equals_is_rejected() {
        assert_eq!(
            parse_priority(b"u=", Priority::default()),
            Err(ParseError::TrailingEquals)
        );
    }

    #[kani::proof]
    fn parameter_trailing_equals_is_rejected() {
        assert_eq!(
            parse_item_with_params(b"?1;foo="),
            Err(ParseError::TrailingEquals)
        );
    }

    #[kani::proof]
    fn single_digit_urgency_matches_domain() {
        let digit: u8 = kani::any();
        kani::assume(digit <= 9);
        let input = [b'u', b'=', b'0' + digit];
        assert_eq!(
            parse_priority(&input, Priority::default()).is_ok(),
            digit <= URGENCY_LOW
        );
    }
}

                | b'%'
                | b'&'
                | b'\''
                | b'*'
                | b'+'
                | b'-'
                | b'.'
                | b'^'
                | b'_'
                | 0x60
                | b'|'
                | b'~'
                | b'/'
                | b':'
        )
}

pub fn parse_priority(input: &[u8], initial: Priority) -> Result<Priority, ParseError> {
    let mut cursor = Cursor::new(input);
    let mut priority = initial;

    cursor.skip_spaces();
    if cursor.is_empty() {
        return Ok(priority);
    }

    loop {
        let key = cursor.parse_key()?;
        let value = if cursor.take_if(b'=') {
            if cursor.is_empty() {
                return Err(ParseError::TrailingEquals);
            }
            Some(cursor.parse_bare_item()?)
        } else {
            None
        };

        match key {
            Key::Urgency => match value {
                Some(BareItem::Integer(value))
                    if i64::from(URGENCY_HIGH) <= value && value <= i64::from(URGENCY_LOW) =>
                {
                    priority.urgency =
                        u8::try_from(value).map_err(|_| ParseError::InvalidUrgency)?;
                }
                _ => return Err(ParseError::InvalidUrgency),
            },
            Key::Incremental => match value {
                None => priority.incremental = true,
                Some(BareItem::Boolean(value)) => priority.incremental = value,
                _ => return Err(ParseError::InvalidIncremental),
            },
            Key::Other => {}
        }

        cursor.parse_params()?;
        cursor.skip_spaces();
        if cursor.is_empty() {
            break;
        }
        if !cursor.take_if(b',') {
            return Err(ParseError::InvalidSyntax);
        }
        cursor.skip_spaces();
        if cursor.is_empty() {
            return Err(ParseError::UnexpectedEnd);
        }
    }

    Ok(priority)
}

pub fn parse_item_with_params(input: &[u8]) -> Result<(), ParseError> {
    let mut cursor = Cursor::new(input);
    let _ = cursor.parse_bare_item()?;
    cursor.parse_params()?;
    cursor.skip_spaces();
    if cursor.is_empty() {
        Ok(())
    } else {
        Err(ParseError::InvalidSyntax)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_known_priority_members() {
        assert_eq!(
            parse_priority(b"u=2, i", Priority::default()),
            Ok(Priority { urgency: 2, incremental: true })
        );
        assert_eq!(
            parse_priority(b"i=?0, u=7", Priority::default()),
            Ok(Priority { urgency: 7, incremental: false })
        );
    }

    #[test]
    fn trailing_equals_is_rejected() {
        assert_eq!(
            parse_priority(b"u=", Priority::default()),
            Err(ParseError::TrailingEquals)
        );
        assert_eq!(
            parse_item_with_params(b"?1;foo="),
            Err(ParseError::TrailingEquals)
        );
    }

    #[test]
    fn urgency_domain_is_exact() {
        for urgency in URGENCY_HIGH..=URGENCY_LOW {
            let input = [b'u', b'=', b'0' + urgency];
            assert_eq!(
                parse_priority(&input, Priority::default()).unwrap().urgency,
                urgency
            );
        }
        assert_eq!(
            parse_priority(b"u=8", Priority::default()),
            Err(ParseError::InvalidUrgency)
        );
    }

    #[test]
    fn differential_against_current_c_priority_oracle() {
        for input in [
            b"u=0".as_slice(),
            b"u=7, i".as_slice(),
            b"i=?0, u=2".as_slice(),
        ] {
            let rust = parse_priority(input, Priority::default()).unwrap();
            let c =
                nghttp3::parse_priority_oracle(input, nghttp3::PriorityValue::default()).unwrap();
            assert_eq!(u32::from(rust.urgency), c.urgency);
            assert_eq!(rust.incremental, c.incremental);
        }

        assert!(
            nghttp3::parse_priority_oracle(b"u=", nghttp3::PriorityValue::default()).is_err()
        );
    }
}

#[cfg(kani)]
mod verification {
    use super::*;

    #[kani::proof]
    fn priority_trailing_equals_is_rejected() {
        assert_eq!(
            parse_priority(b"u=", Priority::default()),
            Err(ParseError::TrailingEquals)
        );
    }

    #[kani::proof]
    fn parameter_trailing_equals_is_rejected() {
        assert_eq!(
            parse_item_with_params(b"?1;foo="),
            Err(ParseError::TrailingEquals)
        );
    }

    #[kani::proof]
    fn single_digit_urgency_matches_domain() {
        let digit: u8 = kani::any();
        kani::assume(digit <= 9);
        let input = [b'u', b'=', b'0' + digit];
        assert_eq!(
            parse_priority(&input, Priority::default()).is_ok(),
            digit <= URGENCY_LOW
        );
    }
}
