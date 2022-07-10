use std::fmt;

mod category;

pub use category::Category;

/// An account number to identify an account.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Number(u32);

impl Number {
    /// Create a new [Number] with a positive integer
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    pub fn number(&self) -> u32 {
        self.0
    }
}

impl From<u32> for Number {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<Number> for u32 {
    fn from(number: Number) -> Self {
        number.0
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
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
    /// This trims and returns Some(AccountName) if it is not an empty string,
    /// otherwise it return None.
    pub fn new<T: AsRef<str>>(name: T) -> Option<Self> {
        let name = name.as_ref().trim().to_owned();
        if name.is_empty() {
            None
        } else {
            Some(Name(name))
        }
    }

    /// Move the inner string out of AccountName thus consuming it
    pub fn into_inner(self) -> String {
        self.0
    }

    pub fn name(&self) -> &str {
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
