//! Representations of banking transactions.

use api::data as t;
use rustc_serialize::{ Decodable, Decoder };

/// Represents a single transaction associated with a given `Account`.
#[derive(Debug)]
pub struct Transaction {
    /// The unique identifier of this transaction.
    pub id: t::UID,
    /// The associated `Account`.
    pub account_id: t::UID,
    /// Dollar value as as float. It is positive to indicate money
    /// moving out of the account, and negative to indicate that
    /// money is moving in.
    pub amount: t::Amount,
    /// The category to which this account belongs.
    /// [A list can be found here](https://plaid.com/docs/api/#all-categories).
    pub category_id: t::CategoryID,
    /// The context in which the transaction occurred.
    pub context: Context,
    /// An hierarchical list of the categories in which
    /// this transaction belongs to.
    pub categories: Vec<String>,
    /// When `true`, then this transaction is cleared and immutable.
    /// When `false`, then it is posted and subject to change in the future.
    pub pending: bool,
    /// The date on which the transaction took place.
    /// Plaid standardizes using the ISO 8601 format.
    pub date: t::Date,
    /// Transaction meta data
    pub meta: Option<Meta>
}

/// Represents meta data associated with the transaction
#[derive(RustcDecodable, Debug)]
pub struct Meta {
    /// The location in which the transaction most likely occured.
    pub location: t::Address
}

impl Decodable for Transaction {

    fn decode<D: Decoder>(decoder: &mut D) -> Result<Transaction, D::Error> {
        decoder.read_struct("root", 9, |d| {
            Ok(Transaction {
                id: try!(d.read_struct_field("_id", 0, |d| Decodable::decode(d))),
                account_id: try!(d.read_struct_field("_account", 1, |d| Decodable::decode(d))),
                amount: try!(d.read_struct_field("amount", 2, |d| Decodable::decode(d))),
                category_id: try!(d.read_struct_field("category_id", 3, |d| Decodable::decode(d))),
                context: try!(d.read_struct_field("type", 4, |d| Decodable::decode(d))),
                categories: try!(d.read_struct_field("category", 5, |d| Decodable::decode(d))),
                pending: try!(d.read_struct_field("pending", 6, |d| Decodable::decode(d))),
                date: try!(d.read_struct_field("date", 7, |d| Decodable::decode(d))),
                meta: try!(d.read_struct_field("meta", 8, |d| Decodable::decode(d)))
            })
        })
    }

}

/// The context in which a transaction took place
#[derive(Debug)]
pub enum Context {
    /// A phyical place
    Place,
    /// An online transaction
    Digital,
    /// Usually banking transactions
    Special,
    /// Could not be determined
    Unresolved
}

impl Decodable for Context {

    fn decode<D: Decoder>(decoder: &mut D) -> Result<Context, D::Error> {
        let s: String = try!(decoder.read_struct("root", 1, |d| {
            d.read_struct_field("primary", 0, |d| Decodable::decode(d))
        }));

        Ok(match s.as_ref() {
            "place" => Context::Place,
            "digital" => Context::Digital,
            "special" => Context::Special,
            _ => Context::Unresolved
        })
    }

}

#[cfg(test)]
mod tests {

    use api::types::*;
    use rustc_serialize::json;

    #[test]
    fn test_decode_transaction() {
        let transaction: Transaction = json::decode(r##"
            {
                "_account": "testaccount",
                "_id": "testtransactionid",
                "amount": 12.70,
                "date": "2016-03-12",
                "name": "Golden Crepes",
                "meta": {
                    "location": {
                        "address": "262 W 15th St",
                        "city": "New York",
                        "state": "NY",
                        "zip": "10011",
                        "coordinates": {
                            "lat": 40.740352,
                            "lon": -74.001761
                        }
                    }
                },
                "pending": false,
                "type": {
                "primary": "place"
                },
                "category": [
                "Food and Drink",
                "Restaurants"
                ],
                "category_id": "13005000",
                "score": {
                    "location": {
                        "address": 1,
                        "city": 1,
                        "state": 1
                    },
                "name": 0.9
                }
            }
        "##).unwrap();

        assert_eq!(transaction.id, "testtransactionid".to_string());
        assert_eq!(transaction.account_id, "testaccount".to_string());
        assert_eq!(transaction.amount, 12.70 as Amount);
        assert_eq!(transaction.category_id, 13005000 as CategoryID);
        assert_eq!(transaction.meta.unwrap().location.street.unwrap(), "262 W 15th St".to_string());
    }

}
