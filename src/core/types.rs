use hmac::Hmac;
use sha2::Sha256;

pub type Key = String;
pub type Value = String;
pub type KeyValueString = (Key, Value);
pub type HmacSha256 = Hmac<Sha256>;
