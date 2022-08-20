use std::{fmt, str::FromStr};

use enum_iterator::IntoEnumIterator;

use crate::balance::Balance;

/// These are the different types of an Account can be associated with.
#[derive(Debug, Clone, Copy, IntoEnumIterator, PartialEq, Eq, PartialOrd, Ord)]
pub enum Category {
    Asset,
    Liability,
    Equity,
    Income,
    Expenses,
}

impl Category {
    /// Return an iterator that iterates over all elements that are
    /// considered to be debit elements.
    ///
    /// These are Asset and Expenses.
    pub fn debits() -> DebitIter {
        DebitIter::new()
    }

    /// Return an iterator that iterates over all elements that are
    /// considered to be credit elements.
    ///
    /// These are Liability, Equity and Income.
    pub fn credits() -> CreditIter {
        CreditIter::new()
    }

    /// Create a transaction that increases this type of Category
    pub fn increase(&self, amount: u32) -> Option<Balance> {
        match self {
            Category::Asset => Balance::debit(amount),
            Category::Liability => Balance::credit(amount),
            Category::Equity => Balance::credit(amount),
            Category::Income => Balance::credit(amount),
            Category::Expenses => Balance::debit(amount),
        }
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Asset => f.write_str("Asset"),
            Self::Liability => f.write_str("Liability"),
            Self::Equity => f.write_str("Equity"),
            Self::Income => f.write_str("Income"),
            Self::Expenses => f.write_str("Expenses"),
        }
    }
}

impl FromStr for Category {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "Asset" => Ok(Self::Asset),
            "Liability" => Ok(Self::Liability),
            "Equity" => Ok(Self::Equity),
            "Income" => Ok(Self::Income),
            "Expenses" => Ok(Self::Expenses),
            _ => Err(ParseError),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Unable to parse category")
    }
}

impl std::error::Error for ParseError {}

/// Iterator over all debit categories.
pub struct DebitIter {
    debits: Vec<Category>,
}

/// Iterator over all credit categories.
pub struct CreditIter {
    credits: Vec<Category>,
}

impl DebitIter {
    fn new() -> Self {
        Self {
            debits: vec![Category::Asset, Category::Expenses],
        }
    }
}

impl CreditIter {
    fn new() -> Self {
        Self {
            credits: vec![Category::Liability, Category::Equity, Category::Income],
        }
    }
}

impl IntoIterator for DebitIter {
    type Item = Category;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.debits.into_iter()
    }
}

impl IntoIterator for CreditIter {
    type Item = Category;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.credits.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use quickcheck::{Arbitrary, Gen};
    use quickcheck_macros::quickcheck;

    #[derive(Debug, Clone, Copy)]
    struct DebitCategory(Category);

    impl Arbitrary for DebitCategory {
        fn arbitrary(g: &mut Gen) -> Self {
            let cat = *g
                .choose(&Category::debits().into_iter().collect::<Vec<_>>())
                .unwrap();
            Self(cat)
        }
    }

    #[derive(Debug, Clone, Copy)]
    struct CreditCategory(Category);

    impl Arbitrary for CreditCategory {
        fn arbitrary(g: &mut Gen) -> Self {
            let cat = *g
                .choose(&Category::credits().into_iter().collect::<Vec<_>>())
                .unwrap();
            Self(cat)
        }
    }

    impl Arbitrary for Category {
        fn arbitrary(g: &mut Gen) -> Self {
            *g.choose(
                &Category::credits()
                    .into_iter()
                    .chain(Category::debits())
                    .collect::<Vec<_>>(),
            )
            .unwrap()
        }
    }

    #[quickcheck]
    fn account_category_increase_for_debits_should_be_debit_transactions(
        category: DebitCategory,
        amount: u32,
    ) -> bool {
        let inc = category.0.increase(amount);

        inc == Balance::debit(amount)
    }

    #[quickcheck]
    fn account_category_increase_for_credits_should_be_credit_transactions(
        category: CreditCategory,
        amount: u32,
    ) -> bool {
        let inc = category.0.increase(amount);

        inc == Balance::credit(amount)
    }

    #[quickcheck]
    fn category_to_string_then_parse_should_be_original(category: Category) -> bool {
        category == category.to_string().parse().unwrap()
    }
}
