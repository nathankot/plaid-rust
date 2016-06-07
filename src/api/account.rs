/// # Account
/// Represents one account associated with the given user.
struct Account {
    /// Global unique identifier for this account
    id: str,
    /// Floating point dollar value representation of the current account balance
    current_balance: f64
}

mod tests {

    #[test]
    fn it_works() {
        let a = Account { id: "abcs123123123", current_balance: 10 };
        assert!(true);
    }

}
