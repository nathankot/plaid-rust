use api::product::Product;
use api::mfa;

/// # Response
///
/// Represents the response from the last API request.
/// This does not encapsulate any errors, rather it indicates different
/// stages of the user lifecycle.
#[derive(Debug)]
pub enum Response<P: Product> {
    /// Waiting on MFA authentication code from the user
    MFA(mfa::Challenge),
    /// Returned when a request is made for a `Product` that is not
    /// currently enabled for the given `User`.
    ///
    /// If this occurs, you should upgrade the `User` so that they have
    /// access to the `Product`.
    ProductNotEnabled(P),
    /// User is authenticated successfully and we have data available
    Success(P::Data),
    /// Nothing is known about the user and no requests have been made
    Unknown
}