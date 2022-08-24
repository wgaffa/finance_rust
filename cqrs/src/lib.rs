use behaviour::AccountError;
use events::Event;
use personal_finance::account::{Number, Name, Category};

pub mod behaviour;
pub mod events;
pub mod identifier;
pub mod stream;

pub struct Chart {
    data: personal_finance::entry::Chart,
    history: Vec<Event>,
}

impl Chart {
    pub fn open(&self, number: Number, name: Name, category: Category) -> Result<(), behaviour::AccountError> {
        let account_already_exist = self.data.iter().any(|x| x.number() == number);
        if account_already_exist {
            Err(AccountError::AccountAlreadyOpened(number.number()))
        } else {
            behaviour::open_account(number.number(), name.into(), category, &self.history);
            Ok(())
        }
    }
}
