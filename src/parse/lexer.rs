#![allow(missing_docs)]

use std::str::CharIndices;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Token<'input> {
    EOL,
    IRIREF(&'input str),
    STRING_LITERAL_QUOTE(&'input str),
    BLANK_NODE_LABEL(&'input str),
    LANG_TAG(&'input str),
    PERIOD,
    DOUBLE_CARET,
}

pub struct Lexer<'input> {
    text: &'input str,
    lookahead: Option<(usize, char)>,
    chars: CharIndices<'input>,
}

impl<'input> Lexer<'input> {
    /// Create a new lexer from the source string
    pub fn new(text: &'input str) -> Self {
        let mut chars = text.char_indices();

        Lexer {
            text,
            lookahead: chars.next(),
            chars,
        }
    }

    /// Bump the current position in the source string by one character,
    /// returning the current character and byte position.
    #[allow(dead_code)]
    fn bump(&mut self) -> Option<(usize, char)> {
        let current = self.lookahead;
        self.lookahead = self.chars.next();
        current
    }

    #[allow(dead_code)]
    fn slice(&self, start: usize, end: usize) -> &'input str {
        &self.text[start..end]
    }

    /// Consume characters while the predicate matches for the current
    /// character, then return the consumed slice and the end byte
    /// position.
    #[allow(dead_code)]
    fn take_while<F>(&mut self, start: usize, mut keep_going: F) -> (usize, &'input str)
    where
        F: FnMut(char) -> bool,
    {
        self.take_until(start, |ch| !keep_going(ch))
    }

    /// Consume characters until the predicate matches for the next character
    /// in the lookahead, then return the consumed slice and the end byte
    /// position.
    #[allow(dead_code)]
    fn take_until<F>(&mut self, start: usize, mut terminate: F) -> (usize, &'input str)
    where
        F: FnMut(char) -> bool,
    {
        while let Some((end, ch)) = self.lookahead {
            if terminate(ch) {
                return (end, self.slice(start, end));
            } else {
                self.bump();
            }
        }
        unimplemented!();
    }

    fn take_double_caret(
        &mut self,
        start: usize,
    ) -> Result<(usize, Token<'input>, usize), ::std::io::Error> {
        if let Some((end, '^')) = self.lookahead {
            Ok((start, Token::DOUBLE_CARET, end))
        } else {
            unimplemented!()
        }
    }

    fn take_lang_tag(&mut self, _start: usize) -> Result<(usize, Token<'input>, usize), ::std::io::Error> {
        unimplemented!()
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Result<(usize, Token<'input>, usize), ::std::io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((start, ch)) = self.bump() {
            return Some(match ch {
                ' ' | '\t' => continue,
                '^' => self.take_double_caret(start),
                '.' => Ok((start, Token::PERIOD, start)),
                '@' => self.take_lang_tag(start),
                _ => panic!(),
            })
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_period() {
        let mut l = Lexer::new(".");
        assert_eq!(l.next().unwrap().unwrap().1, Token::PERIOD);
    }

    #[test]
    fn parse_double_caret() {
        let mut l = Lexer::new("^^");
        assert_eq!(l.next().unwrap().unwrap().1, Token::DOUBLE_CARET);
    }
}
