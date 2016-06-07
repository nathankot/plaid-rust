/// # Transaction
/// Represents a single transaction
struct Transaction {
    id: str,
    /// The associated `Account`
    account_id: str,
    /// Dollar value as as float. It is positive to indicate money
    /// moving out of the account, and negative to indicate that
    /// money is moving in.
    amount: f64,
    /// When `true`, then this transaction is cleared and immutable.
    /// When `false`, then it is posted and subject to change in the future.
    pending: bool,
}
