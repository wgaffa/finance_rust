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

        let [schema, category, id] = TryInto::<[Identifier; 3]>::try_into(split)
            .map_err(|x| error_stack::report!(ParseError::InvalidLength(x)))?;

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
        Self::new(schema, category, id)
    }
}

#[derive(Debug, Clone)]
pub enum ParseError {
    InvalidStream,
    InvalidLength(Vec<Identifier>),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidStream => f.write_str("Stream id is invalid"),
            Self::InvalidLength(input) => write!(
                f,
                "Stream must contain atleast 3 identifiers, got {input:?}"
            ),
        }
    }
}

impl Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn parse_and_new_gives_equal_types(input: (String, String, String)) -> bool {
        let input = [input.0, input.1, input.2];
        use crate::identifier::Identifier;
        let stream_new = input
            .iter()
            .map(Identifier::new)
            .collect::<Option<Vec<_>>>()
            .map(|ids| Stream::new(ids[0].clone(), ids[1].clone(), ids[2].clone()));

        let stream_parse = input.join(".").parse::<Stream>().ok();

        stream_new == stream_parse
    }
}
