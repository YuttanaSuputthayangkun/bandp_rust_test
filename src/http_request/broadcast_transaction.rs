use std::time::{self, SystemTime, UNIX_EPOCH};

const BROADCAST_URL: &str = "https://mock-node-wgqbnxruha-as.a.run.app/broadcast";

#[derive(Clone, PartialEq, Eq)]
pub struct Symbol(String);

pub const SYMBOL_LENGTH: usize = 3;

#[derive(Debug, PartialEq, Eq)]
pub enum IntoSymbolError {
    InvalidLength,
    InvalidCharacter,
}

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

#[derive(Clone, Copy)]
pub struct Price(u64);

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

#[derive(Clone, Copy)]
pub struct TimeStamp(u64);

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

#[derive(Debug, serde::Serialize)]
pub struct Transaction {
    pub symbol: Symbol,
    pub price: Price,
    pub time_stamp: TimeStamp,
}

#[derive(Debug, serde::Deserialize)]
pub struct TransactionResponse {
    pub tx_hash: String,
}

#[derive(Debug)]
pub enum Error {
    RequestError(reqwest::Error),
    InvalidResponseBody,
}

pub type BroadcastResponse = Result<TransactionResponse, Error>;

pub async fn broadcast(transaction: &Transaction) -> BroadcastResponse {
    let trasaction_string = serde_json::to_string(&transaction).unwrap(); // expected to be all valid
    let client = reqwest::Client::new();
    let result = client
        .post(BROADCAST_URL)
        .json(&trasaction_string)
        .send()
        .await
        .map_err(Error::RequestError)?; // network error
    let status = result.status();
    let err_result = result.error_for_status_ref().map_err(Error::RequestError)?;
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
