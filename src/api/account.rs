//! An account represents a user's specific bank account.

use super::types::*;
use rustc_serialize::{ Decodable, Decoder };

/// # Account
/// Represents one account associated with the given user.
pub struct Account {
    /// The unique id of the account.
    pub id: String,
    /// An id unique to the accounts of a particular access token
    pub item_id: String,
    /// The total amount of funds in the account
    pub current_balance: f64,
    /// The Current Balance less any outstanding holds or debits that
    /// have not yet posted to the account. May sometimes not be available.
    pub available_balance: Option<f64>,
    /// The financial institution associated with the account.
    pub institution: Institution,
    /// The classification of this account.
    /// [See here for more info](https://plaid.com/docs/api/#connect-account-types).
    account_type: String,
    /// A more detailed classification of the account.
    /// This is not always available.
    /// [See here for a list of possible types][sub-types].
    /// [sub-types]: https://plaid.com/docs/api/#connect-account-subtypes
    account_subtype: Option<String>
}

/// Accounts are decodable from JSON.
impl Decodable for Account {

    fn decode<D: Decoder>(decoder: &mut D) -> Result<Account, D::Error> {
        decoder.read_struct("root", 0, |decoder| {
            let (current_balance, available_balance) =
                try!(decoder.read_struct_field("balance", 2, |d| {
                    let c: f64 = try!(d.read_struct_field("current", 0, |d| Decodable::decode(d)));
                    let a: Option<f64> = try!(d.read_struct_field("available", 0, |d| Decodable::decode(d)));
                    Ok((c, a))
                }));

            Ok(Account {
                id: try!(decoder.read_struct_field("_id", 0, |d| Decodable::decode(d))),
                item_id: try!(decoder.read_struct_field("_item", 0, |d| Decodable::decode(d))),
                current_balance: current_balance,
                available_balance: available_balance,
                institution: try!(decoder.read_struct_field("institution_type", 0, |d| Decodable::decode(d))),
                account_type: try!(decoder.read_struct_field("type", 0, |d| Decodable::decode(d))),
                account_subtype: try!(decoder.read_struct_field("subtype", 0, |d| Decodable::decode(d)))
            })
        })
    }

}

#[cfg(test)]
mod tests {

    use super::*;
    use rustc_serialize::json;

    #[test]
    fn test_decode_wholesome_account() {
        let acc: Account = json::decode(r##"
            {
            "_id": "YzzrzBrO9OSzo6BXwAvVuL5dmMKMqkhOoEqeo",
            "_item": "aWWVW4VqGqIdaP495QyOSVLN1nzjLwhXaPDJJ",
            "_user": "bkkVkMVwQwfYmBMy9jzqHQob9O1KwpFaEyLOE",
            "balance": {
                "available": 7205.23,
                "current": 7255.23
            },
            "institution_type": "fake_institution",
            "meta": {
                "name": "Plaid Credit Card",
                "number": "3002"
            },
            "type": "depository",
            "subtype": "checking"
            }
        "##).unwrap();

        assert_eq!(acc.id, "YzzrzBrO9OSzo6BXwAvVuL5dmMKMqkhOoEqeo".to_string());
        assert_eq!(acc.item_id, "aWWVW4VqGqIdaP495QyOSVLN1nzjLwhXaPDJJ".to_string());
        assert_eq!(acc.current_balance, 7255.23 as f64);
        assert_eq!(acc.available_balance, Some(7205.23 as f64));
        assert_eq!(acc.institution, "fake_institution".to_string());
        assert_eq!(acc.account_type, "depository".to_string());
        assert_eq!(acc.account_subtype, Some("checking".to_string()));
    }

    #[test]
    fn test_decode_account_with_missing_values() {
        let acc: Account = json::decode(r##"
            {
            "_id": "YzzrzBrO9OSzo6BXwAvVuL5dmMKMqkhOoEqeo",
            "_item": "aWWVW4VqGqIdaP495QyOSVLN1nzjLwhXaPDJJ",
            "_user": "bkkVkMVwQwfYmBMy9jzqHQob9O1KwpFaEyLOE",
            "balance": { "current": 7255.23 },
            "institution_type": "fake_institution",
            "meta": {},
            "type": "depository"
            }
        "##).unwrap();

        assert_eq!(acc.id, "YzzrzBrO9OSzo6BXwAvVuL5dmMKMqkhOoEqeo".to_string());
        assert_eq!(acc.item_id, "aWWVW4VqGqIdaP495QyOSVLN1nzjLwhXaPDJJ".to_string());
        assert_eq!(acc.current_balance, 7255.23 as f64);
        assert_eq!(acc.available_balance, None);
        assert_eq!(acc.institution, "fake_institution".to_string());
        assert_eq!(acc.account_type, "depository".to_string());
        assert_eq!(acc.account_subtype, None);
    }

}
