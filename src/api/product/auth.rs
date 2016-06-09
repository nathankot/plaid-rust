use api::product::{ Product };
use api::account::Account;
use api::client::Payload;

/// `Auth` is the endpoint you need in order to check that the user owns their account.
#[derive(Debug)]
pub struct Auth;

/// Representation of data that is retrieved from the `Auth` product.
#[derive(Debug, RustcDecodable)]
pub struct AuthData {
    /// List of accounts associated with the user
    pub accounts: Vec<Account>,
}

impl Product for Auth {
    type Data = AuthData;
    fn description<'a>(&self) -> &'a str { "Auth" }
    fn endpoint<'a, 'b>(&self, payload: &'b Payload) -> &'a str {
        match *payload {
            Payload::StepMFA(..) => "/auth/step",
            Payload::FetchData(..) => "/auth/get",
            Payload::Upgrade(..) => "/upgrade?upgrade_to=auth",
            _ => "/auth"
        }
    }
}
