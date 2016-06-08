//! Defines a set of type alias to improve understandability.

/// A user's bank account username
pub type Username = String;

/// A user's bank account password
pub type Password = String;

/// A user's multi-factor authentication code
pub type MFACode = String;

/// A user's secret access token
pub type AccessToken = String;

/// Your client id from the Plaid dashboard
pub type ClientID = String;

/// Your client secret from the Plaid dashboard
pub type ClientSecret = String;

/// A user's institution. See [here for a list](https://plaid.com/docs/api/#institutions)
pub type Institution = String;

/// Dates are simply stored as their original `String` representation.
/// It is up to you to parse it with your favorite date/time library.
pub type Date = String;
