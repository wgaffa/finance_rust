use std::{error::Error, fmt, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Identifier(String);

/// Identifier is any alphanumeric character and \[_\]
impl Identifier {
    pub fn new<T: AsRef<str>>(identifier: T) -> Option<Self> {
        let identifier = identifier.as_ref();
        if identifier.len() == 0 {
            return None;
        }

        if identifier.chars().all(|x| x.is_alphanumeric() || x == '_') {
            Some(Self(identifier.into()))
        } else {
            None
        }
    }
}

impl FromStr for Identifier {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Identifier::new(s).ok_or(ParseError)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Unable to parse identifier")
    }
}

impl Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    use quickcheck_macros::quickcheck;

    use test_case::test_case;

    #[test_case("valid")]
    #[test_case("valid_underscore")]
    #[test_case("valid_underscore_342")]
    #[test_case("23")]
    fn new_given_valid_identifiers_should_return_some(input: &str) {
        let actual = Identifier::new(input);

        assert_eq!(actual, Some(Identifier(input.to_owned())));
    }

    #[test_case("")]
    #[test_case("43%")]
    fn new_given_invalid_identifiers_should_return_none(input: &str) {
        assert_eq!(Identifier::new(input), None)
    }

    // Identifier::new(x) == x.parse::<Identifier>().ok()
    #[quickcheck]
    fn new_should_be_equal_to_parse(input: String) -> bool {
        let left = Identifier::new(&input);
        let right = input.parse::<Identifier>().ok();

        left == right
    }
}
