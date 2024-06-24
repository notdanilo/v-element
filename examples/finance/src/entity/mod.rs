use crate::{Account, Identity};

pub struct Entity {
    pub identities: Vec<Identity>,
    pub accounts: Vec<Account>
}
