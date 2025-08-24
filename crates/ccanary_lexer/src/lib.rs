//! The rustc_lexer we have at home.
//!
//! You can imagine this as a baby-ified version of Rust's implementation.
//!
//! https://github.com/rust-lang/rust/blob/master/compiler/rustc_lexer

mod cursor;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Base {
    Decimal,
    Octal,
    Hex,
    Binary,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Suffix {
    U,
    L,
    UL,
    LL,
    ULL,
    F,
    L,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StringPrefix {
    None,
    L,
    U,
    U8,
    U,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LiteralKind {
    Integer { base: Base, suffix: Suffix },
    Float { suffix: Suffix },
    Char,
    String { prefix: StringPrefix },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    LineComment,
    BlockComment,
    Whitespace,
    Identifier,

    Literal {
        kind: LiteralKind,
    },

    /// ;
    Semi,
    /// ,
    Comma,
    /// .
    Dot,
    // (
    OpenParan,
    // )
    ClosedParan,
    // {
    OpenBrace,
    // }
    ClosedBrace,
    // #
    Pound,
    // =
    Equals,
    // !
    Bang,
    // <
    Lessthan,
    // >
    Greaterthan,
    // -
    Minus,
    // +
    Plus,
    // &
    And,
    // |
    Or,
    // *
    Star,
    // /
    Slash,
    // ^
    Caret,
    // %
    Percent,
}
