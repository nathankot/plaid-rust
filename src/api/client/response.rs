//! Response

use api::product::Product;
use api::mfa;
use api::user::User;

/// # Response
///
/// Represents the response from the last API request.
/// This does not encapsulate any errors, rather it indicates different
/// stages of the user lifecycle.
///
/// It **does not** implement `Decodable` because it is simply an enum that
/// represents a specific meta data on top of any `Decodable` types that it wraps.
///
/// For error handling, `Result` is used alongside `plaid::api::error::Error`.
#[derive(Debug)]
pub enum Response<P: Product> {
    /// Waiting on multifactor authentication code from the user
    MFA(User, mfa::Challenge),
    /// Returned when a request is made for a `Product` that is not
    /// currently enabled for the given `User`.
    ///
    /// If this occurs, you should upgrade the `User` so that they have
    /// access to the `Product`.
    ProductNotEnabled(User, P),
    /// We have sucessfully fetched the available data pertaining to the
    /// given `Product`.
    ProductData(P::Data),
    /// We have successfully authenticated the user, and have retrieved
    /// the relevant `Product::Data` along with that authentication.
    Authenticated(User, P::Data),
    /// Nothing is known about the user and no requests have been made
    Unknown
}
