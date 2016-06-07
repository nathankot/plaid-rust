//! The product module

use std::fmt::Debug;
use std::any::Any;

/// Anything that implements `Product` can be used as a product.
pub trait Product : Sync + Debug {
    /// The response data that is associated with this product
    type Data: Debug + Any;
    /// The endpoint of the product with leading slash, e.g `/connect`
    fn endpoint_component<'a>(&self) -> &'a str;
    /// A textual representation of the product, e.g `Connect`
    fn description<'a>(&self) -> &'a str;
}

/// `Connect` is the endpoint you need to fetch transaction for a `User`
#[derive(Debug)]
pub struct Connect;

/// Use `Auth` to authorize ACH payments
#[derive(Debug)]
pub struct Auth;

/// Use `Info` to get live account balances
#[derive(Debug)]
pub struct Info;

/// Use `Income` to determine the `User`'s yearly income
#[derive(Debug)]
pub struct Income;

/// Use `Risk` to get a credit risk score computed by Plaid
#[derive(Debug)]
pub struct Risk;

impl Product for Connect {
    type Data = ();
    fn endpoint_component<'a>(&self) -> &'a str { "/connect" }
    fn description<'a>(&self) -> &'a str { "Connect" }
}

impl Product for Auth {
    type Data = ();
    fn endpoint_component<'a>(&self) -> &'a str { "/auth" }
    fn description<'a>(&self) -> &'a str { "Auth" }
}

impl Product for Info {
    type Data = ();
    fn endpoint_component<'a>(&self) -> &'a str { "/info" }
    fn description<'a>(&self) -> &'a str { "Info" }
}

impl Product for Income {
    type Data = ();
    fn endpoint_component<'a>(&self) -> &'a str { "/income" }
    fn description<'a>(&self) -> &'a str { "Income" }
}

impl Product for Risk {
    type Data = ();
    fn endpoint_component<'a>(&self) -> &'a str { "/risk" }
    fn description<'a>(&self) -> &'a str { "Risk" }
}
