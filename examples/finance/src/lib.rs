mod identity;
mod entity;
mod currency;
mod account;
mod bank;

pub use identity::*;
pub use entity::*;
pub use currency::*;
pub use account::*;
pub use bank::*;

pub struct Transaction {

}

// Generate it from https://en.wikipedia.org/wiki/ISO_3166-1
// https://crates.io/crates/rust_iso3166
pub enum Country {
    Brazil,
    Singapore,
    UnitedStates,
    Poland
}