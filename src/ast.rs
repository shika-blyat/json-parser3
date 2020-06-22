use std::collections::HashMap;

use crate::tokens::Spanned;

#[derive(Debug)]
pub enum JsonValue<'a> {
    Null,
    Num(isize),
    Str(&'a str),
    Bool(bool),
    Array(Vec<Spanned<JsonValue<'a>>>),
    Object(HashMap<&'a str, Spanned<JsonValue<'a>>>),
}
