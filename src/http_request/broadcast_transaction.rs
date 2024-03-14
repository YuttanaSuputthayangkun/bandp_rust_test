#[cfg(doc)]
use crate::http_request::monitor_transaction::monitor;
use std::time::{self, SystemTime, UNIX_EPOCH};

#[doc(hidden)]
const BROADCAST_URL: &str = "https://mock-node-wgqbnxruha-as.a.run.app/broadcast";

/// Transaction symbol, e.g., BTC, ETH.
/// Acting as a field of [`Transaction`].
/// Can be created from a `&str`.
#[derive(Clone, PartialEq, Eq)]
pub struct Symbol(#[doc(hidden)] String);

/// Length of valid [`Symbol`]
pub const SYMBOL_LENGTH: usize = 3;

/// An enumeration of possible errors which can occur while trying to create [`Symbol`] from String.
#[derive(Debug, PartialEq, Eq)]
pub enum IntoSymbolError {
    /// Expect length to be [`SYMBOL_LENGTH`]
    InvalidLength,
    /// English alphabet characters only
    InvalidCharacter,
}

/// [`Symbol`] can be created from String, but you have to make sure it's valid.
/// Checkout [`IntoSymbolError`].
impl<'a> TryFrom<&'a str> for Symbol {
    type Error = IntoSymbolError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            invalid_len if invalid_len.len() != SYMBOL_LENGTH => {
                Err(IntoSymbolError::InvalidLength)
            }
            invalid_char if !invalid_char.chars().all(|c| c.is_alphabetic()) => {
                Err(IntoSymbolError::InvalidCharacter)
            }
            valid => Ok(Symbol(valid.to_ascii_uppercase().to_string())),
        }
    }
}

impl std::fmt::Debug for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl serde::Serialize for Symbol {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

/// A price type for [`Transaction`]. Can be created from u64. Cannot be 0.
#[derive(Clone, Copy)]
pub struct Price(#[doc(hidden)] u64);

/// Price cannot be 0 value.
impl TryFrom<u64> for Price {
    type Error = &'static str;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value == 0 {
            Err("Price cannot be 0.")
        } else {
            Ok(Price(value))
        }
    }
}

impl std::fmt::Debug for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl serde::Serialize for Price {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

/// UNIX epoch time in seconds.
/// Acting as a field of [`Transaction`].
#[derive(Clone, Copy)]
pub struct TimeStamp(#[doc(hidden)] u64);

impl TimeStamp {
    pub fn new(t: time::SystemTime) -> Self {
        let seconds = t.duration_since(UNIX_EPOCH).unwrap().as_secs();
        TimeStamp(seconds)
    }

    pub fn now() -> Self {
        Self::new(SystemTime::now())
    }
}

impl Default for TimeStamp {
    fn default() -> Self {
        Self::now()
    }
}

impl std::fmt::Debug for TimeStamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl serde::Serialize for TimeStamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

/// A type used as an input for [`broadcast`]
#[derive(Debug, serde::Serialize)]
pub struct Transaction {
    /// Transaction symbol, e.g., BTC, ETH
    pub symbol: Symbol,
    /// Symbol price, e.g., 10000
    pub price: Price,
    /// Timestamp of price retrieval
    pub time_stamp: TimeStamp,
}

/// A result from [`broadcast`], can be used as an input of [`monitor`]
#[derive(Debug, serde::Deserialize)]
pub struct TransactionResponse {
    pub tx_hash: String,
}

/// An enumeration of possible errors which can occur while trying to call [`broadcast`]
#[derive(Debug)]
pub enum Error {
    /// Server or network error
    RequestError(reqwest::Error),
    /// The response body(JSON) cannot be parsed into [`TransactionResponse`]
    InvalidResponseBody,
}

pub type BroadcastResponse = Result<TransactionResponse, Error>;

/// Calling this will broadcast transaction through the network.
///
/// # Examples
///
/// ```
/// use http_request::broadcast_transaction::*;
/// let transaction = Transaction {
///     symbol: Symbol::try_from("ETC").unwrap(),
///     price: 555.try_into().unwrap(),
///     time_stamp: TimeStamp::now(),
/// };
/// let result = broadcast(&transaction).await;
/// ```
pub async fn broadcast(transaction: &Transaction) -> BroadcastResponse {
    let trasaction_string = serde_json::to_string(&transaction).unwrap(); // expected to be all valid
    let client = reqwest::Client::new();
    let result = client
        .post(BROADCAST_URL)
        .json(&trasaction_string)
        .send()
        .await
        .map_err(Error::RequestError)?; // network error
    let result_body = result.text().await.map_err(Error::RequestError)?; // encoding error
    let response: TransactionResponse =
        serde_json::from_str(&result_body).map_err(|_| Error::InvalidResponseBody)?; // the response body is not serializable
    Ok(response)
}

#[cfg(test)]
mod test {
    use super::*;

    mod input_validation {
        use super::*;

        #[test]
        fn symbol_invalid_length() {
            assert_eq!(Symbol::try_from("A"), Err(IntoSymbolError::InvalidLength));
            assert_eq!(
                Symbol::try_from("AAAA"),
                Err(IntoSymbolError::InvalidLength)
            );
        }

        #[test]
        fn symbol_invalid_character() {
            assert_eq!(
                Symbol::try_from("555"),
                Err(IntoSymbolError::InvalidCharacter)
            );
        }

        #[test]
        fn zero_price() {
            assert!(Price::try_from(0).is_err());
        }
    }
}
