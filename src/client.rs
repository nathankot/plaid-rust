/// # Client
///
/// Represents a Plaid API consumer. Encapsulates the `endpoint`,
/// `client_id` and `secret` of the consumer.
pub struct Client {
    /// E.g `https://api.plaid.com/`
    endpoint: str,
    client_id: str,
    secret: str
}
