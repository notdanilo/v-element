use crate::{Bank, CreditCard};

pub struct Account {
    pub bank: Bank,
    pub credit_cards: Vec<CreditCard>,
}
