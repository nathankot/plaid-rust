//! The product module

use std::fmt::Debug;
use std::any::Any;
use rustc_serialize::Decodable;
use api::client::payload::Payload;

/// Anything that implements `Product` can be used as a product.
pub trait Product : Any + Sync + Debug {
    /// The response data that is associated with this product
    type Data: Debug + Any + Decodable;
    /// The endpoint of the product for the given payload.
    /// With leading slash, e.g `/connect/get`
    fn endpoint<'a, 'b>(&self, &'b Payload) -> &'a str;
    /// A textual representation of the product, e.g `Connect`
    fn description<'a>(&self) -> &'a str;
}

pub use self::connect::*;
pub use self::auth::*;

mod connect;
mod auth;

// /// Use `Auth` to authorize ACH payments
// #[derive(Debug)]
// pub struct Auth;

// impl Product for Auth {
//     type Data = ();
//     fn endpoint_component<'a>(&self) -> &'a str { "/auth" }
//     fn description<'a>(&self) -> &'a str { "Auth" }
// }

// /// Use `Info` to get live account balances
// #[derive(Debug)]
// pub struct Info;

// impl Product for Info {
//     type Data = ();
//     fn endpoint_component<'a>(&self) -> &'a str { "/info" }
//     fn description<'a>(&self) -> &'a str { "Info" }
// }

// /// Use `Income` to determine the `User`'s yearly income
// #[derive(Debug)]
// pub struct Income;

// impl Product for Income {
//     type Data = ();
//     fn endpoint_component<'a>(&self) -> &'a str { "/income" }
//     fn description<'a>(&self) -> &'a str { "Income" }
// }

// /// Use `Risk` to get a credit risk score computed by Plaid
// #[derive(Debug)]
// pub struct Risk;

// impl Product for Risk {
//     type Data = ();
//     fn endpoint_component<'a>(&self) -> &'a str { "/risk" }
//     fn description<'a>(&self) -> &'a str { "Risk" }
// }
