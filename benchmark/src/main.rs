// async use
use async_thrift::server;
use async_std::task;
use async_std::io::Error;
mod async_thrift_test;
use futures::future::*;
// sync use
mod original_thrift_test;
use std::thread;
// util
mod util;

/// config parameter
// number of clients
const THREAD_NUM: i32 = 100;
// number of calls for each client
const LOOP_NUM: i32 = 1000;
// change the mode of bench
const SYNC_MODE: bool = false;

fn run_sync_both() {
    util::print_config(THREAD_NUM, LOOP_NUM);

    thread::spawn(|| original_thrift_test::server::run());
    // time
    let start = time::now();

    let mut list = Vec::new();
    for i in 0..THREAD_NUM {
        list.push(thread::spawn(|| original_thrift_test::client::run(LOOP_NUM)));
    }


    for task in list {
        task.join();
    }

    let end = time::now();

    util::print_result(String::from("sync"), THREAD_NUM * LOOP_NUM, (end - start).num_milliseconds());
}

async fn run_async_both() {
    util::print_config(THREAD_NUM, LOOP_NUM);

    async_std::task::spawn(async_thrift_test::server::run_server("127.0.0.1:9090"));
    // time
    let start = time::now();

    let mut list = Vec::new();
    for i in 0..THREAD_NUM {
        list.push(async_std::task::spawn(async_thrift_test::client::run_client("127.0.0.1:9090", LOOP_NUM)));
    }


    let f = join_all(list);
    f.await;

    let end = time::now();
    //
    util::print_result(String::from("async"), THREAD_NUM * LOOP_NUM, (end - start).num_milliseconds());
}

fn main() {
    if !SYNC_MODE {
        task::block_on(run_async_both());
    } else {
        run_sync_both();
    }
}

