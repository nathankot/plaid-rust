//! # The Plaid API

pub mod user;
pub mod client;
pub mod error;
pub mod product;
pub mod mfa;

mod data;

pub use self::data::account;
pub use self::data::transaction;
pub use self::data::types;
