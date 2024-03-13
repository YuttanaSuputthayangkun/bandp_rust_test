#![allow(unused)]

mod boss_babys_revenge;
mod http_request;
mod supermans_chicken_rescue;

fn main() {
    println!("Hello, world!");

    let result = boss_babys_revenge::check_boss_behavior("SRSSRRR".try_into().unwrap());
    println!("result: {result:?}");
}
