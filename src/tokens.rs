use std::ops::Range;

#[derive(Debug)]
pub struct Spanned<T> {
    pub elem: T,
    pub span: Range<usize>,
}

#[derive(Debug)]
pub enum Token<'a> {
    LBrace,   // [
    RBrace,   // ]
    LBracket, // {
    RBracket, // }
    Comma,    // ,
    Colon,    // :
    Str(&'a str),
    Num(isize),
    Bool(bool),
    Null,
    EOF,
}
