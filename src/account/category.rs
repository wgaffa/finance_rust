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
}
