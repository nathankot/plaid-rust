//! Representations of a user's bank account.

use api::types::*;
use rustc_serialize::{ Decodable, Decoder };

/// # Account
/// Represents one account associated with the given `User`.
#[derive(Debug)]
pub struct Account {
    /// The unique id of the account.
    pub id: UID,
    /// An id unique to the accounts of a particular access token
    pub item_id: UID,
    /// The total amount of funds in the account
    pub current_balance: Amount,
    /// The Current Balance less any outstanding holds or debits that
    /// have not yet posted to the account. May sometimes not be available.
    pub available_balance: Option<Amount>,
    /// The financial institution associated with the account.
    pub institution: Institution,
    /// The classification of this account.
    /// [See here for more info](https://plaid.com/docs/api/#connect-account-types).
    pub account_type: String,
    /// A more detailed classification of the account.
    /// This is not always available.
    /// [See here for a list of possible types][sub-types].
    /// [sub-types]: https://plaid.com/docs/api/#connect-account-subtypes
    pub account_subtype: Option<String>,
    /// The user's bank account number.
    /// Only available when using `api::product::Auth`.
    pub account_number: Option<String>,
    /// The user's routing number.
    /// Only available when using `api::product::Auth`.
    pub routing_number: Option<String>,
    /// The user's wire routing number.
    /// Only available when using `api::product::Auth`.
    pub wire_routing_number: Option<String>,
    /// Meta-data associated with this account
    pub meta: Option<Meta>
}

#[derive(Debug, RustcDecodable)]
/// Any meta-data associated with the account.
pub struct Meta {
    /// Name of the account (e.g "Plaid credit card".)
    pub name: Option<Name>,
    /// Number associated with the name.
    pub number: Option<String>,
    /// Any limit associated with the account, if it's a credit card.
    pub limit: Option<Amount>
}

/// Accounts are decodable from JSON.
impl Decodable for Account {

    fn decode<D: Decoder>(decoder: &mut D) -> Result<Account, D::Error> {
        decoder.read_struct("root", 8, |decoder| {
            let (current_balance, available_balance) =
                try!(decoder.read_struct_field("balance", 0, |d| {
                    d.read_struct("balance", 2, |d| {
                        let c: Amount = try!(d.read_struct_field("current", 0, |d| Decodable::decode(d)));
                        let a: Option<Amount> = try!(d.read_struct_field("available", 1, |d| Decodable::decode(d)));
                        Ok((c, a))
                    })
                }));

            let (account_number, routing_number, wire_routing_number) =
                try!(decoder.read_struct_field("numbers", 1, |d| {
                    d.read_option(|d, exists| {
                        if exists {
                            d.read_struct("numbers", 3, |d| {
                                Ok((try!(d.read_struct_field("account", 0, |d| Decodable::decode(d))),
                                    try!(d.read_struct_field("routing", 1, |d| Decodable::decode(d))),
                                    try!(d.read_struct_field("wireRouting", 2, |d| Decodable::decode(d)))))
                            })
                        } else {
                            Ok((None, None, None))
                        }
                    })
                }));

            Ok(Account {
                id: try!(decoder.read_struct_field("_id", 2, |d| Decodable::decode(d))),
                item_id: try!(decoder.read_struct_field("_item", 3, |d| Decodable::decode(d))),
                current_balance: current_balance,
                available_balance: available_balance,
                institution: try!(decoder.read_struct_field("institution_type", 4, |d| Decodable::decode(d))),
                account_type: try!(decoder.read_struct_field("type", 5, |d| Decodable::decode(d))),
                account_subtype: try!(decoder.read_struct_field("subtype", 6, |d| Decodable::decode(d))),
                account_number: account_number,
                routing_number: routing_number,
                wire_routing_number: wire_routing_number,
                meta: try!(decoder.read_struct_field("meta", 7, |d| Decodable::decode(d)))
            })
        })
    }

}

#[cfg(test)]
mod tests {

    use api::types::*;
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
                "subtype": "checking",
                "numbers": {
                    "routing": "021000021",
                    "account": "9900009606",
                    "wireRouting": "021000022"
                }
            }
        "##).unwrap();

        assert_eq!(acc.id, "YzzrzBrO9OSzo6BXwAvVuL5dmMKMqkhOoEqeo".to_string());
        assert_eq!(acc.item_id, "aWWVW4VqGqIdaP495QyOSVLN1nzjLwhXaPDJJ".to_string());
        assert_eq!(acc.current_balance, 7255.23 as Amount);
        assert_eq!(acc.available_balance, Some(7205.23 as Amount));
        assert_eq!(acc.institution, "fake_institution".to_string());
        assert_eq!(acc.account_type, "depository".to_string());
        assert_eq!(acc.account_subtype, Some("checking".to_string()));
        assert_eq!(acc.account_number, Some("9900009606".to_string()));
        assert_eq!(acc.routing_number, Some("021000021".to_string()));
        assert_eq!(acc.wire_routing_number, Some("021000022".to_string()));
        assert_eq!(acc.meta.unwrap().name, Some("Plaid Credit Card".to_string()));
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
        assert_eq!(acc.current_balance, 7255.23 as Amount);
        assert_eq!(acc.available_balance, None);
        assert_eq!(acc.institution, "fake_institution".to_string());
        assert_eq!(acc.account_type, "depository".to_string());
        assert_eq!(acc.account_subtype, None);
        assert_eq!(acc.account_number, None);
        assert_eq!(acc.routing_number, None);
        assert_eq!(acc.wire_routing_number, None);
    }

}
