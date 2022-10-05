use std::{fmt, num::NonZeroU32};

mod category;

pub use category::Category;

/// An account number to identify an account.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Number(NonZeroU32);

impl Number {
    /// Create a new [Number] with a positive integer
    pub fn new(value: u32) -> Option<Self> {
        NonZeroU32::new(value).map(|v| Self(v))
    }

    pub fn number(&self) -> u32 {
        self.0.get()
    }
}

impl From<Number> for u32 {
    fn from(number: Number) -> Self {
        number.0.get()
    }
}

impl From<NonZeroU32> for Number {
    fn from(v: NonZeroU32) -> Self {
        Self(v)
    }
}

impl TryFrom<u32> for Number {
    type Error = std::num::TryFromIntError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(Self(NonZeroU32::try_from(value)?))
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.get())
    }
}

/// An account name is a trimmed non-empty string.
///
/// # Examples
/// ```
/// use personal_finance::account;
///
/// let name = account::Name::new("  My Bank Account\n");
/// assert_eq!(name.unwrap(), "My Bank Account");
///
/// let name = account::Name::new("    ");
/// assert_eq!(name, None);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Name(String);

impl Name {
    /// Create a new AccountName
    ///
    /// This trims and returns Some([Name]) if it is not an empty string,
    /// otherwise it return None.
    pub fn new<T: AsRef<str>>(name: T) -> Option<Self> {
        let name = name.as_ref().trim().to_owned();
        if name.is_empty() {
            None
        } else {
            Some(Name(name))
        }
    }

    /// Move the inner string out of [Name] thus consuming it
    pub fn into_inner(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl PartialEq<String> for Name {
    fn eq(&self, other: &String) -> bool {
        self.0 == *other
    }
}

impl PartialEq<&str> for Name {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<String> for Name {
    fn partial_cmp(&self, other: &String) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialOrd<&str> for Name {
    fn partial_cmp(&self, other: &&str) -> Option<std::cmp::Ordering> {
        self.0.as_str().partial_cmp(*other)
    }
}

impl From<Name> for String {
    fn from(other: Name) -> Self {
        other.0
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_case::test_case;

    #[test_case("No leading" => Some(Name(String::from("No leading"))))]
    #[test_case("   Leading" => Some(Name(String::from("Leading"))))]
    #[test_case("Trailing\t" => Some(Name(String::from("Trailing"))))]
    #[test_case("\n Both \n" => Some(Name(String::from("Both"))))]
    #[test_case("\n  \n" => None)]
    fn account_name_new(input: &str) -> Option<Name> {
        Name::new(input)
    }
}
