use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum Token {
    INCR,
    DECR,
    MOVR,
    MOVL,
    SLOOP,
    ELOOP,
    INPT,
    OPT,
    NONE,
}

impl From<&char> for Token {
    fn from(c: &char) -> Self {
        match c {
            '+' => Self::INCR,
            '-' => Self::DECR,
            '>' => Self::MOVR,
            '<' => Self::MOVL,
            '[' => Self::SLOOP,
            ']' => Self::ELOOP,
            ',' => Self::INPT,
            '.' => Self::OPT,
            ' ' | '\n' | '\t' | '\r' => Self::NONE,
            _ => panic!("INVALID TOKEN: {}", c),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::INCR => write!(f, "+"),
            Self::DECR => write!(f, "-"),
            Self::MOVR => write!(f, ">"),
            Self::MOVL => write!(f, "<"),
            Self::SLOOP => write!(f, "["),
            Self::ELOOP => write!(f, "]"),
            Self::INPT => write!(f, ","),
            Self::OPT => write!(f, "."),
            Self::NONE => write!(f, ""),
        }
    }
}
