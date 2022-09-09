#[derive(Debug)]
pub enum AccountError {
    AccountAlreadyOpened(u32),
}

impl std::fmt::Display for AccountError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AccountAlreadyOpened(id) => write!(f, "Account '{id}' has already been opened"),
        }
    }
}

impl std::error::Error for AccountError {}
