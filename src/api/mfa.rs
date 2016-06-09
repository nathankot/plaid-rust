//! This module contains data structures and implementations
//! related to multi-factor-authentication.

use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};

/// Represents one of the different types of multi-factor-authentication
/// challenges Plaid supports.
///
/// Todo: support all mfa challenges
#[derive(Debug, Eq, PartialEq)]
pub enum Challenge {
    /// A token-based authorization, this token will be sent to one of
    /// the user's registered devices.
    Code,
    /// A list of possible challenge devices, in which the user should
    /// choose one and then pass along the selection using `api::client::payload::AuthenticateOptions`.
    /// It is in the form of `(device_type, device_mask)`.
    DeviceList(Vec<(Device, String)>),
    /// A list of questions that need to be answered.
    Questions(Vec<String>),
    /// A list of multi-choice selections
    Selections(Vec<(String, Vec<String>)>)
}

/// Represents a response to a previously given MFA challenge.
#[derive(Debug, Eq, PartialEq)]
pub enum Response {
    /// A response to a code challenge, providing the code
    /// that was sent to the user's device.
    Code(String),
    /// Responses to a previously given list of questions.
    Questions(Vec<String>),
    /// Responses to a previously given list of selections.
    Selections(Vec<String>)
}

impl Encodable for Response {

    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        match *self {
            Response::Code(ref str) => e.emit_str(str),
            Response::Questions(ref answers) => answers.encode(e),
            Response::Selections(ref answers) => answers.encode(e)
        }
    }

}

struct DeviceAndMask(Device, String);
impl Decodable for DeviceAndMask {

    fn decode<D: Decoder>(decoder: &mut D) -> Result<DeviceAndMask, D::Error> {
        decoder.read_struct("root", 2, |d| {
            let mask = try!(d.read_struct_field("mask", 0, |d| Decodable::decode(d)));
            let device = try!(d.read_struct_field("type", 1, |d| Decodable::decode(d)));
            Ok(DeviceAndMask(device, mask))
        })
    }

}

struct Question(String);
impl Decodable for Question {

    fn decode<D: Decoder>(d: &mut D) -> Result<Question, D::Error> {
        d.read_struct("root", 1, |d| {
            let s = try!(d.read_struct_field("question", 0, |d| Decodable::decode(d)));
            Ok(Question(s))
        })
    }

}

struct Selection(String, Vec<String>);
impl Decodable for Selection {

    fn decode<D: Decoder>(d: &mut D) -> Result<Selection, D::Error> {
        d.read_struct("root", 2, |d| {
            let question: String = try!(d.read_struct_field("question", 0, |d| Decodable::decode(d)));
            let answers: Vec<String> = try!(d.read_struct_field("answers", 1, |d| Decodable::decode(d)));
            Ok(Selection(question, answers))
        })
    }

}

impl Decodable for Challenge {

    fn decode<D: Decoder>(d: &mut D) -> Result<Challenge, D::Error> {
        d.read_struct("root", 2, |d| {
            let t: String = try!(d.read_struct_field("type", 0, |d| Decodable::decode(d)));
            match t.as_ref() {
                "device" => Ok(Challenge::Code),
                "questions" => {
                    let list: Vec<Question> = try!(d.read_struct_field("mfa", 1, |d| Decodable::decode(d)));
                    let list = list.iter().map(|&Question(ref s)| s.clone()).collect();
                    Ok(Challenge::Questions(list))
                },
                "list" => {
                    let list: Vec<DeviceAndMask> = try!(d.read_struct_field("mfa", 1, |d| Decodable::decode(d)));
                    let list = list.iter().map(|&DeviceAndMask(ref device, ref mask)| (*device, mask.clone())).collect();
                    Ok(Challenge::DeviceList(list))
                },
                "selections" => {
                    let list: Vec<Selection> = try!(d.read_struct_field("mfa", 1, |d| Decodable::decode(d)));
                    let list = list.iter().map(|&Selection(ref question, ref answers)| (question.clone(), answers.clone())).collect();
                    Ok(Challenge::Selections(list))
                },
                p => Err(d.error(format!("Un-supported MFA preference: {}", p).as_ref()))
            }
        })
    }

}

/// Represents a device that can be used for multifactor authentication
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Device {
    /// Code sent to the user's email
    Email,
    /// Code sent to the user's phone number via sms
    Phone,
    /// Verify a credit card number
    Card
}

impl Decodable for Device {

    fn decode<D: Decoder>(decoder: &mut D) -> Result<Device, D::Error> {
        let s = try!(decoder.read_str());
        match s.as_ref() {
            "email" => Ok(Device::Email),
            "phone" => Ok(Device::Phone),
            "card" => Ok(Device::Card),
            _ => Err(decoder.error("Unknown mfa device type"))
        }
    }

}

impl Encodable for Device {

    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        match *self {
            Device::Email => e.emit_str("email"),
            Device::Phone => e.emit_str("phone"),
            Device::Card => e.emit_str("card")
        }
    }

}

#[cfg(test)]
mod tests {

    use api::mfa;
    use api::mfa::{Device, Challenge};
    use rustc_serialize::json;

    #[test]
    fn test_decoding_code_challenges() {
        let r: mfa::Challenge = json::decode(r##"
            { "type": "device",
              "mfa": { "message": "Code sent to xxx-xxx-5309" },
              "access_token": "xxxxx" }
        "##).unwrap();
        assert_eq!(format!("{:?}", r), "Code");
    }

    #[test]
    fn test_decoding_question_challenges() {
        let r: mfa::Challenge = json::decode(r##"
            { "type": "questions",
              "mfa": [{"question": "What was the name of your first pet?"}, {"question": "Whats your name?"}],
              "access_token": "xxxxx" }
        "##).unwrap();

        assert_eq!(format!("{:?}", r), format!("{:?}", Challenge::Questions(vec![
            "What was the name of your first pet?".to_string(),
            "Whats your name?".to_string()
        ])));
    }

    #[test]
    fn test_decoding_selection_challenges() {
        let r: mfa::Challenge = json::decode(r##"
            {
                "type": "selections",
                "mfa": [{
                    "question": "Did you open account 'Checking: 000' in Las Vegas?",
                    "answers": ["Yes", "No"]
                }, {
                    "question": "Did you open account 'Savings: 111' in Las Vegas?",
                    "answers": ["Yes", "No"]
                }],
                "access_token": "xxxxx"
            }
        "##).unwrap();

        assert_eq!(format!("{:?}", r), format!("{:?}", Challenge::Selections(vec![
            ("Did you open account 'Checking: 000' in Las Vegas?".to_string(), vec!["Yes".to_string(), "No".to_string()]),
            ("Did you open account 'Savings: 111' in Las Vegas?".to_string(), vec!["Yes".to_string(), "No".to_string()])
        ])));
    }

    #[test]
    fn test_decoding_device_list_challenges() {
        let r: mfa::Challenge = json::decode(r##"
            {   "type": "list",
                "mfa": [
                    {"mask":"t..t@plaid.com", "type":"email"},
                    {"mask":"xxx-xxx-5309", "type":"phone"},
                    {"mask":"SafePass Card", "type":"card"}
                ],
                "access_token": "xxxxx" }
        "##).unwrap();
        assert_eq!(format!("{:?}", r), format!("{:?}", Challenge::DeviceList(vec![
            (Device::Email, "t..t@plaid.com".to_string()),
            (Device::Phone, "xxx-xxx-5309".to_string()),
            (Device::Card, "SafePass Card".to_string())
        ])));
    }

    #[test]
    fn test_decoding_device() {
        let r: mfa::Device = json::decode(r##""email""##).unwrap();
        assert_eq!(format!("{:?}", r), "Email");
        let r: mfa::Device = json::decode(r##""phone""##).unwrap();
        assert_eq!(format!("{:?}", r), "Phone");
        let r: mfa::Device = json::decode(r##""card""##).unwrap();
        assert_eq!(format!("{:?}", r), "Card");
    }

}
