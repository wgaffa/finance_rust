use std::borrow::Cow;

use crate::Chart;

/// A ledger id is a string starting with any alphanumeric character [a-zA-Z0-9]
/// followed by any valid character in [a-zA-Z0-9_-]
#[derive(Debug, PartialEq, Eq)]
pub struct LedgerId(String);

impl LedgerId {
    pub fn new(id: &str) -> Option<Self> {
        id.starts_with(|x: char| x.is_ascii_alphanumeric())
            .then_some(())
            .and_then(|_| {
                id.chars()
                    .skip(1)
                    .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '-'))
                    .then_some(LedgerId(id.to_owned()))
            })
    }
}

pub struct Ledger {
    id: LedgerId,
    chart: Chart,
}

#[cfg(test)]
mod tests {
    use super::*;

    use proptest::proptest;

    proptest! {
        #[test]
        fn invalid_ledger_ids(s in "[_-][a-zA-Z0-9]*") {
            assert_eq!(LedgerId::new(&s), None);
        }
    }

    proptest! {
        #[test]
        fn valid_ledger_ids(s in "[a-zA-Z0-9][a-zA-Z0-9_-]*") {
            assert_eq!(LedgerId::new(&s), Some(LedgerId(s)))
        }
    }
}
