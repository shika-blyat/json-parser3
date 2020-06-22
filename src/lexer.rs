use crate::{
    error::{ErrReason, JsonError},
    tokens::{Spanned, Token},
};
use std::{iter::Peekable, str::CharIndices};

pub struct Lexer<'a> {
    chars: Peekable<CharIndices<'a>>,
    source: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            chars: source.char_indices().peekable(),
            source,
        }
    }
    pub fn tokenize(mut self) -> Result<Vec<Spanned<Token<'a>>>, Spanned<JsonError<'a>>> {
        let mut tokens = vec![];
        while let Some((pos, c)) = self.next() {
            let token = match c {
                '"' => self.str(pos)?,
                c if c.is_ascii_alphabetic() => self.keyword(pos)?,
                c if c.is_ascii_digit() => self.num(pos),
                c if c.is_ascii_whitespace() => continue,
                '[' => Spanned {
                    span: pos..pos + 1,
                    elem: Token::LBracket,
                },
                ']' => Spanned {
                    span: pos..pos + 1,
                    elem: Token::RBracket,
                },
                '{' => Spanned {
                    span: pos..pos + 1,
                    elem: Token::LBrace,
                },
                '}' => Spanned {
                    span: pos..pos + 1,
                    elem: Token::RBrace,
                },
                ':' => Spanned {
                    span: pos..pos + 1,
                    elem: Token::Colon,
                },
                ',' => Spanned {
                    span: pos..pos + 1,
                    elem: Token::Comma,
                },
                _ => {
                    return Err(Spanned {
                        span: pos..pos + 1,
                        elem: JsonError {
                            reason: ErrReason::UnexpectedChar(c),
                        },
                    })
                }
            };
            tokens.push(token);
        }
        let end = tokens.last().unwrap().span.end;
        tokens.push(Spanned {
            elem: Token::EOF,
            span: end..end,
        });
        Ok(tokens)
    }
    fn str(&mut self, pos: usize) -> Result<Spanned<Token<'a>>, Spanned<JsonError<'a>>> {
        let mut size = 1;
        while let Some((_, c)) = self.peek() {
            if *c != '"' {
                self.next();
                size += 1;
            } else {
                self.next();
                return Ok(Spanned {
                    elem: Token::Str(&self.source[pos + 1..pos + size]),
                    span: pos..pos + size,
                });
            }
        }
        return Err(Spanned {
            span: pos + size - 1..pos + size,
            elem: JsonError {
                reason: ErrReason::UnclosedString,
            },
        });
    }
    fn num(&mut self, pos: usize) -> Spanned<Token<'a>> {
        let mut size = 1;
        while let Some((_, c)) = self.peek() {
            if c.is_ascii_digit() {
                self.next();
                size += 1;
            } else {
                break;
            }
        }
        Spanned {
            elem: Token::Num(self.source[pos..pos + size].parse().unwrap()),
            span: pos..pos + size,
        }
    }
    fn keyword(&mut self, pos: usize) -> Result<Spanned<Token<'a>>, Spanned<JsonError<'a>>> {
        let mut size = 1;
        while let Some((_, c)) = self.peek() {
            if c.is_ascii_alphabetic() {
                self.next();
                size += 1;
            } else {
                break;
            }
        }
        Ok(Spanned {
            elem: match &self.source[pos..pos + size] {
                "true" => Token::Bool(true),
                "false" => Token::Bool(false),
                "null" => Token::Null,
                s => {
                    return Err(Spanned {
                        span: pos..pos + size,
                        elem: JsonError {
                            reason: ErrReason::UnknownKeyword(s),
                        },
                    })
                }
            },
            span: pos..pos + size,
        })
    }
    fn next(&mut self) -> Option<(usize, char)> {
        self.chars.next()
    }
    fn peek(&mut self) -> Option<&'_ (usize, char)> {
        self.chars.peek()
    }
}
