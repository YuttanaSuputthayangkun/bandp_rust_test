const MONITOR_URL: &str = "https://mock-node-wgqbnxruha-as.a.run.app/check";

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "UPPERCASE")]
enum ServerTransactionStatus {
    Confirmed,
    Failed,
    Pending,
    Dne,
}

#[derive(Debug, serde::Deserialize)]
struct Response {
    tx_status: ServerTransactionStatus,
}

#[derive(Debug)]
pub enum Error {
    RequestErr(reqwest::Error),
    InvalidResponseBody,
    Failed,
    Pending,
    Dne,
}

pub type MonitorResult = Result<(), Error>;

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
