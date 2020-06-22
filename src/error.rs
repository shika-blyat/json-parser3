use crate::tokens::Token;

#[derive(Debug)]
pub struct JsonError<'a> {
    pub reason: ErrReason<'a>,
}

#[derive(Debug)]
pub enum ErrReason<'a> {
    UnclosedString,
    UnknownKeyword(&'a str),
    UnexpectedChar(char),
    UnexpectedTok(Token<'a>),
    Expected(&'static str),
}
