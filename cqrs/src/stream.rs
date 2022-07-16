use std::{error::Error, fmt, str::FromStr};

use crate::identifier::{self, Identifier};
use error_stack::{IntoReport, Report, ResultExt};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Stream {
    schema: Identifier,
    category: Identifier,
    id: Identifier, // Year/Month of transaction for example
}

impl Stream {
    pub fn new(schema: Identifier, category: Identifier, id: Identifier) -> Self {
        Self {
            schema,
            category,
            id,
        }
    }
}

impl FromStr for Stream {
    type Err = error_stack::Report<ParseError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s
            .split('.')
            .map(|x| {
                x.parse::<Identifier>()
                    .report()
                    .attach_printable(x.to_owned())
            })
            .collect::<Result<Vec<_>, Report<identifier::ParseError>>>() // This only gets the first Err variant
            .change_context(ParseError::InvalidStream)?;

        let split: [Identifier; 3] = split
            .try_into()
            .map_err(|x| error_stack::report!(ParseError::InvalidLength(x)))?;

        let [schema, category, id]: [Identifier; 3] = split;

        Ok(Stream {
            schema,
            category,
            id,
        })
    }
}

impl From<(Identifier, Identifier, Identifier)> for Stream {
    fn from(value: (Identifier, Identifier, Identifier)) -> Self {
        let (schema, category, id) = value;
        Self {
            schema,
            category,
            id,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ParseError {
    InvalidStream,
    InvalidLength(Vec<Identifier>),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    use quickcheck::Arbitrary;
    use quickcheck_macros::quickcheck;
}
