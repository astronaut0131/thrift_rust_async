// async
use async_thrift::server;
use async_std::task;
use async_std::io::Error;

mod async_thrift_test;

use futures::future::*;

// sync
mod original_thrift_test;
use std::thread;


const THREAD_NUM: i32 = 1000;
const LOOP_NUM: i32 = 1000;

// change the mode of bench
const SYNC_MODE: bool = true;

fn run_sync_both() {
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
    println!("sync mode done! duration:{:?} ms", (end - start).num_milliseconds());
}

async fn run_async_both() {
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
    println!("async mode done! duration:{:?} ms", (end - start).num_milliseconds());
}

fn main() {
    if !SYNC_MODE {
        task::block_on(run_async_both());
    } else {
        run_sync_both();
    }
}