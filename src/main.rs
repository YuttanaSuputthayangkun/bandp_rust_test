#![allow(unused)]

mod boss_babys_revenge;
mod http_request;
mod limited_input_range;
mod supermans_chicken_rescue;

#[tokio::main]
async fn main() {
    // let result = boss_babys_revenge::check_boss_behavior("SRSSRRR".try_into().unwrap());
    // println!("result: {result:?}");

    // test_broadcast().await;
    test_monitor().await;
}

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

async fn test_monitor() {
    use http_request::monitor_transaction::*;
    let tx_hash = "abcdefg";
    let result = monitor(tx_hash).await;
    println!("monitor result: {result:?}");
}
