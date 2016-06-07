//! Client

/// # Client
///
/// Represents a Plaid API consumer. Encapsulates the `endpoint`,
/// `client_id` and `secret` of the consumer.
pub struct Client<'a> {
    /// E.g `https://api.plaid.com`
    pub endpoint: &'a str,
    /// Your application's `client_id`
    pub client_id: &'a str,
    /// Your application's `secret`
    pub secret: &'a str,
}
