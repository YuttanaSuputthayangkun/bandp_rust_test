#[allow(unused)]
#[doc(hidden)]
mod boss_babys_revenge;

mod http_request;

#[allow(unused)]
#[doc(hidden)]
mod limited_input_range;

#[allow(unused)]
#[doc(hidden)]
mod supermans_chicken_rescue;

#[doc(hidden)]
#[tokio::main]
async fn main() {
    // let result = boss_babys_revenge::check_boss_behavior("SRSSRRR".try_into().unwrap());
    // println!("result: {result:?}");

    // test_broadcast().await;
    // test_monitor().await;
}

#[doc(hidden)]
#[allow(unused)]
async fn test_broadcast() {
    use http_request::broadcast_transaction::*;
    let transaction = Transaction {
        symbol: Symbol::try_from("ETC").unwrap(),
        price: 555.try_into().unwrap(),
        time_stamp: TimeStamp::now(),
    };
    let result = broadcast(&transaction).await;
    println!("broadcast result: {result:?}");
}

#[doc(hidden)]
#[allow(unused)]
async fn test_monitor() {
    use http_request::monitor_transaction::*;
    let tx_hash = "abcdefg";
    let result = monitor(tx_hash).await;
    println!("monitor result: {result:?}");
}

#[doc(hidden)]
#[allow(unused)]
async fn test_broadcast_monitor_integration() {
    use http_request::broadcast_transaction::*;
    use http_request::monitor_transaction::*;

    let transaction = Transaction {
        symbol: Symbol::try_from("ETC").unwrap(),
        price: 555.try_into().unwrap(),
        time_stamp: TimeStamp::now(),
    };
    let result = broadcast(&transaction).await;
    if let Err(_e) = result {
        todo!(); // handle error
    }
    let result = monitor(&result.unwrap().tx_hash).await;
    match result {
        Ok(_) => todo!(),
        Err(e) => match e {
            http_request::monitor_transaction::Error::RequestErr(_) => todo!(),
            http_request::monitor_transaction::Error::InvalidResponseBody => todo!(),
            http_request::monitor_transaction::Error::Failed => todo!(),
            http_request::monitor_transaction::Error::Pending => todo!(),
            http_request::monitor_transaction::Error::Dne => todo!(),
        },
    }
}
