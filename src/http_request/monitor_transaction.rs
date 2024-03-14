#[cfg(doc)]
use crate::http_request::broadcast_transaction::*;

#[doc(hidden)]
const MONITOR_URL: &str = "https://mock-node-wgqbnxruha-as.a.run.app/check";

// this is used internally exclusively for deserialization
#[doc(hidden)]
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "UPPERCASE")]
enum ServerTransactionStatus {
    Confirmed,
    Failed,
    Pending,
    Dne,
}

// this is used internally exclusively for deserialization
#[doc(hidden)]
#[derive(Debug, serde::Deserialize)]
struct Response {
    tx_status: ServerTransactionStatus,
}

/// An enumeration of possible errors which can occur while trying to call [`monitor`]
#[derive(Debug)]
pub enum Error {
    /// Server or network error.
    /// The client might not be able to connect to the network.
    /// Or the network is unavailable.
    /// Please check for the availability and call [`broadcast`] again.
    RequestErr(reqwest::Error),
    /// The response body(JSON) cannot be parsed into supported response structure.
    /// Please contact the network administrator.
    InvalidResponseBody,
    /// Transaction has failed to process.
    /// Please check the tx_hash sent.
    /// It's possible that the `price` in possession is now insufficient, or the `symbol` doesn't exist within the network.
    Failed,
    /// Transaction awaiting proccess.
    /// The transaction data has reached the network. Depend on the amount of the transaction, this might take some time.
    /// Please wait then call [`monitor`] again to check if the status has changed.
    Pending,
    /// Transaction `D`oes `n`ot `e`xist.
    /// It's possible that the request sent from [`broadcast`] doesn't reach the server.
    /// Make sure to check the result from [`broadcast`] if it's a network error, try sending again.
    Dne,
}

pub type MonitorResult = Result<(), Error>;

/// Use [`broadcast`] to get a [`TransactionResponse`] then supply its tx_hash to this function.
/// The result of the function contains possible [`Error`] to identify the failure cause.
/// Please check for more information on how to handle each error.
///
/// # Examples
/// ```
/// use http_request::broadcast_transaction::*;
/// use http_request::monitor_transaction::*;

/// let transaction = Transaction {
///     symbol: Symbol::try_from("ETC").unwrap(),
///     price: 555.try_into().unwrap(),
///     time_stamp: TimeStamp::now(),
/// };
/// let result = broadcast(&transaction).await;
/// if let Err(_e) = result {
///     todo!(); // handle error
/// }
/// let result = monitor(&result.unwrap().tx_hash).await;
/// match result {
///     Ok(_) => todo!(),
///     Err(e) => match e {
///         http_request::monitor_transaction::Error::RequestErr(_) => todo!(),
///         http_request::monitor_transaction::Error::InvalidResponseBody => todo!(),
///         http_request::monitor_transaction::Error::Failed => todo!(),
///         http_request::monitor_transaction::Error::Pending => todo!(),
///         http_request::monitor_transaction::Error::Dne => todo!(),
///     },
/// }
/// ```
pub async fn monitor(tx_hash: &str) -> MonitorResult {
    let url = format!("{}/{}", MONITOR_URL, tx_hash);
    let client = reqwest::Client::new();
    let response = client.get(url).send().await.map_err(Error::RequestErr)?; // map err and early return
    let response_body = response.text().await.map_err(Error::RequestErr)?; // map err and early return
    let response: Response =
        serde_json::from_str(&response_body).map_err(|_| Error::InvalidResponseBody)?; // map err and early return
    match response.tx_status {
        ServerTransactionStatus::Confirmed => Ok(()),
        // match negative parsed transaction status into errors to comply with the Rust's convention
        ServerTransactionStatus::Failed => Err(Error::Failed),
        ServerTransactionStatus::Pending => Err(Error::Pending),
        ServerTransactionStatus::Dne => Err(Error::Dne),
    }
}
