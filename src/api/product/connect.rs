use api::product::{ Product };
use api::account::Account;
use api::transaction::Transaction;

use rustc_serialize::Decodable;

/// `Connect` is the endpoint you need to fetch transaction for a `User`
#[derive(Debug)]
pub struct Connect;

/// Representation of data that is retrieved from the `Connect` product.
#[derive(Debug, RustcDecodable)]
pub struct ConnectData {
    /// List of accounts associated with the user
    pub accounts: Vec<Account>,
    /// List of transactions associated with the user
    pub transactions: Vec<Transaction>
}

impl Product for Connect {
    type Data = ConnectData;
    fn endpoint_component<'a>(&self) -> &'a str { "/connect" }
    fn description<'a>(&self) -> &'a str { "Connect" }
}
