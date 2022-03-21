use std::{fmt::{Display, Write}, convert::Infallible, str::FromStr};


#[derive(Debug)]
pub struct Delimiter {
    pub start: char,
    pub end: char,
}

impl Display for Delimiter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.start);
        f.write_str("<=>");
        f.write_char(self.end)
    }
}

#[derive(Debug)]
pub enum DelimiterParseErr {
    MissingSeparator,
    TooManyArgumentDelimiters,
    MustBeChar
}

impl std::error::Error for DelimiterParseErr {

}

impl Display for DelimiterParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DelimiterParseErr::MissingSeparator => f.write_str("Missing Delimiter Separator `<=>`"),
            DelimiterParseErr::TooManyArgumentDelimiters => f.write_str("Expected exactly two delimiters: one start and one end delimiter"),
            DelimiterParseErr::MustBeChar =>f.write_str("Delimiter must be a char"),
        }
    }
}

impl From<Infallible> for DelimiterParseErr {
    fn from(never: Infallible) -> Self {
        match never {}
    }
}

impl FromStr for Delimiter {
    type Err = DelimiterParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split("<=>").collect();
        if parts.len() == 1 {
            Err(DelimiterParseErr::MissingSeparator)?
        }
        if parts.len() > 2 {
            Err(DelimiterParseErr::TooManyArgumentDelimiters)?
        }
        let start = parts[0].chars().next().ok_or(DelimiterParseErr::MustBeChar)?;
        let end = parts[1].chars().next().ok_or(DelimiterParseErr::MustBeChar)?;
        Ok(Delimiter { start, end })
    }
}