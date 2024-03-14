#![allow(unused)]

mod boss_babys_revenge;
mod http_request;
mod limited_input_range;
mod supermans_chicken_rescue;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    // let result = boss_babys_revenge::check_boss_behavior("SRSSRRR".try_into().unwrap());
    // println!("result: {result:?}");

    {
        use http_request::broadcast_transaction::*;
        let transaction = Transaction {
            symbol: Symbol::try_from("ETC").unwrap(),
            price: 555.try_into().unwrap(),
            time_stamp: TimeStamp::now(),
        };
        let result = broadcast(&transaction).await;
        println!("broadcast result: {result:?}");
    }
}
