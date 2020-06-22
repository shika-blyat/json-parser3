use crate::{
    ast::JsonValue,
    error::{ErrReason, JsonError},
    tokens::{Spanned, Token},
};
use std::{collections::HashMap, iter::Peekable, vec::IntoIter};
macro_rules! val {
    ($name: ident, $val: ident, $pattern: pat, $variant: path) => {
        fn $name(&mut self) -> Option<Spanned<JsonValue<'a>>> {
            match self.peek() {
                Some(Spanned { elem: $pattern, .. }) => {
                    if let Spanned {
                        span,
                        elem: $pattern,
                    } = self.next().unwrap()
                    {
                        return Some(Spanned {
                            span,
                            elem: $variant($val),
                        });
                    }
                    unreachable!()
                }
                _ => None,
            }
        }
    };
}

macro_rules! token {
    ($name: ident, $p: pat, $s: literal) => {
        pub fn $name(&mut self) -> Result<Spanned<Token<'a>>, Spanned<JsonError<'a>>> {
            if let Some(Spanned { elem: $p, .. }) = self.peek() {
                return Ok(self.next().unwrap());
            }
            Err(Spanned {
                span: self.next_span(),
                elem: JsonError {
                    reason: ErrReason::Expected($s),
                },
            })
        }
    };
}

pub struct Parser<'a, I>
where
    I: Iterator<Item = Spanned<Token<'a>>>,
{
    tokens: Peekable<I>,
}
impl<'a, I: Iterator<Item = Spanned<Token<'a>>>> Parser<'a, I> {
    pub fn json(mut self) -> Result<Spanned<JsonValue<'a>>, Spanned<JsonError<'a>>> {
        let val = self.value()?;
        match self.tokens.next() {
            Some(Spanned {
                elem: Token::EOF, ..
            }) => return Ok(val),
            Some(Spanned { elem, span }) => {
                return Err(Spanned {
                    span,
                    elem: JsonError {
                        reason: ErrReason::UnexpectedTok(elem),
                    },
                })
            }
            _ => unreachable!(),
        }
    }
    pub fn value(&mut self) -> Result<Spanned<JsonValue<'a>>, Spanned<JsonError<'a>>> {
        self.array().or_else(|e| match e {
            Spanned {
                elem:
                    JsonError {
                        reason: ErrReason::Expected("left bracket"),
                    },
                ..
            } => self.object().or_else(|e| match e {
                Spanned {
                    elem:
                        JsonError {
                            reason: ErrReason::Expected("left brace"),
                        },
                    ..
                } => self
                    .null()
                    .or_else(|| self.bool())
                    .or_else(|| self.str())
                    .or_else(|| self.num())
                    .ok_or_else(|| Spanned {
                        span: self.next_span(),
                        elem: JsonError {
                            reason: ErrReason::Expected("a value"),
                        },
                    }),
                e => return Err(e),
            }),
            e => return Err(e),
        })
    }
    fn object(&mut self) -> Result<Spanned<JsonValue<'a>>, Spanned<JsonError<'a>>> {
        let start = self.lbrace()?.span.start;
        let mut values = HashMap::new();
        while let Some(Spanned {
            elem: JsonValue::Str(key),
            ..
        }) = self.str()
        {
            self.colon()?;
            let val = self.value()?;
            values.insert(key, val);
            if let Err(_) = self.comma() {
                break;
            }
        }
        let end = self.rbrace()?.span.end;
        Ok(Spanned {
            span: start..end,
            elem: JsonValue::Object(values),
        })
    }
    fn array(&mut self) -> Result<Spanned<JsonValue<'a>>, Spanned<JsonError<'a>>> {
        let start = self.lbracket()?.span.start;
        let mut values = vec![];
        while let Ok(val) = self.value() {
            if let Err(_) = self.comma() {
                break;
            }
            values.push(val);
        }
        let end = self.rbracket()?.span.end;
        Ok(Spanned {
            span: start..end,
            elem: JsonValue::Array(values),
        })
    }
    fn null(&mut self) -> Option<Spanned<JsonValue<'a>>> {
        match self.peek() {
            Some(Spanned {
                elem: Token::Null, ..
            }) => {
                if let Spanned {
                    span,
                    elem: Token::Null,
                } = self.next().unwrap()
                {
                    return Some(Spanned {
                        span,
                        elem: JsonValue::Null,
                    });
                }
                unreachable!()
            }
            _ => None,
        }
    }
    fn next_span(&mut self) -> std::ops::Range<usize> {
        match self.peek() {
            Some(Spanned { span, .. }) => span.clone(),
            None => unreachable!(),
        }
    }
    fn next(&mut self) -> Option<Spanned<Token<'a>>> {
        self.tokens.next()
    }
    fn peek(&mut self) -> Option<&'_ Spanned<Token<'a>>> {
        self.tokens.peek()
    }
    token!(lbrace, Token::LBrace, "left brace");
    token!(rbrace, Token::RBrace, "right brace");
    token!(lbracket, Token::LBracket, "left bracket");
    token!(rbracket, Token::RBracket, "right bracket");
    token!(comma, Token::Comma, "comma");
    token!(colon, Token::Colon, "colon");
    val!(num, val, Token::Num(val), JsonValue::Num);
    val!(bool, val, Token::Bool(val), JsonValue::Bool);
    val!(str, val, Token::Str(val), JsonValue::Str);
}

impl<'a> Parser<'a, IntoIter<Spanned<Token<'a>>>> {
    pub fn new(tokens: Vec<Spanned<Token<'a>>>) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
        }
    }
}
