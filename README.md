# bandp_rust_test
All problems are written in Rust.

I put emphasis on making sure the code is strongly typed. Therefore all inputs needs to be safely validated and convereted into their own type. Following this design, the main functions have no concerns over the validity of the data given to them.


#### Problem 1: Boss Baby's Revenge

This task uses O(N) time complexity, by iterating over peekable input characters to determine shoot-retalitate sequence. If an invalid sequence is detected negative result is returned early. Otherwise will continue to read until no characters left, then positive result is returned.

The main function is named `check_boss_behavior`.
Read more [here](./src/boss_babys_revenge.rs)


#### Problem 2: Superman's Chicken Rescue

This task uses O(N) time complexity. I brute force the roof tests over every chicken positions in parallel. Then finding the maximum result from the threads.

The main function is named `max_chicken_protected`.
Read more [here](./src/supermans_chicken_rescue.rs)


#### Problem 3: Transaction Broadcasting And Monitoring Client

For this task, I found myself unable to get valid response from the broadcast endpoint. 
So I can't really test the APIs. But I assume the objective of the task is about documentation and structures. So I only focus on that.

Input structures are designed solely from my imagination. Please pay it no mind.

Please run `cargo doc` to generate documents.

The main function is named `broadcast`.
Read more on broadcasting [here](./src/http_request/broadcast_transaction.rs)

The main function is named `monitor`.
Read more on monitoring [here](./src/http_request/monitor_transaction.rs)