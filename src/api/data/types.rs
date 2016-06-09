//! Type aliases that map to common types.

/// Unique identifiers in Plaid are represented as a globally unique hash.
pub type UID = String;

/// Category identifiers are represented by an unsigned integer.
pub type CategoryID = u32;

/// All amounts are represented in a 64-bit floating-point type.
/// This is for legacy reasons and may change in the future.
pub type Amount = f64;

/// A user's bank account username.
pub type Username = String;

/// A user's bank account password.
pub type Password = String;

/// A user's multi-factor authentication code.
pub type MFACode = String;

/// A user's secret access token
pub type AccessToken = String;

/// Your client id from the Plaid dashboard.
pub type ClientID = String;

/// Your client secret from the Plaid dashboard.
pub type ClientSecret = String;

/// A user's institution. See [here for a list](https://plaid.com/docs/api/#institutions).
pub type Institution = String;

/// A PIN number
pub type PIN = String;

/// Dates are simply stored as their original `String` representation.
/// It is up to you to parse it with your favorite date/time library.
pub type Date = String;
