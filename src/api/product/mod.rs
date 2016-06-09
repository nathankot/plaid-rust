//! Product definitions that provide endpoint and response deserialization
//! information.

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
pub use self::balance::*;
pub use self::info::*;

pub mod connect;
pub mod auth;
pub mod info;
pub mod balance;
