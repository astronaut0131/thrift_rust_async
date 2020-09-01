// async use
use async_thrift::server;
use async_std::task;
use async_std::io::Error;

mod async_thrift_test;

use futures::future::*;

// sync use
mod original_thrift_test;

use std::thread;
use std::time::Duration;
use std::cell::RefCell;
use crate::util::handle_time;

// util
mod util;

// const
const CONFIG_LOCATION: usize = 0;
const SYNC_LOCATION: usize = 1;
const ASYNC_LOCATION: usize = 2;

/// config parameter
// number of clients
const THREAD_NUM: i32 = 100;
// number of calls for each client
const LOOP_NUM: i32 = 1000;

// run sync server and client
fn run_sync_both(output: &mut Vec<String>) {
    println!("begin sync benchmark...");

    // print config
    output[CONFIG_LOCATION] = util::format_config(THREAD_NUM, LOOP_NUM);
    // start server
    thread::spawn(|| original_thrift_test::server::run());
    // we need to wait the server to run
    thread::sleep(Duration::from_secs(2));

    // time clock start here
    let start = time::now();

    // build client thread
    let mut list = Vec::new();
    for i in 0..THREAD_NUM {
        list.push(thread::spawn(|| original_thrift_test::client::run(LOOP_NUM)));
    }

    // to collect time result from client
    let mut res = Vec::new();
    for task in list {
        res.push(task.join().unwrap().unwrap());
    }

    // time clock end here;
    let end = time::now();

    // handle raw time result to statistic
    let time_statistic = handle_time(res);
    output[SYNC_LOCATION] = util::format_result(String::from("sync"), (THREAD_NUM * LOOP_NUM) as i64,
                                                 (end - start).num_milliseconds(),
                                                 time_statistic[0], time_statistic[1],
                                                 time_statistic[2], time_statistic[3],
                                                 time_statistic[4], time_statistic[5],
                                                time_statistic[6]);

    println!("sync finished!");
}

// run async server and client
async fn run_async_both(output: &mut Vec<String>) {
    println!("begin async benchmark...");
    let server = async_std::task::spawn(async_thrift_test::server::run_server("127.0.0.1:9090"));

    // time
    let mut list = Vec::new();
    for i in 0..THREAD_NUM {
        // async_thrift_test::client::run_client("127.0.0.1:9090", LOOP_NUM).await;
        list.push(async_std::task::spawn(async_thrift_test::client::run_client("127.0.0.1:9090", LOOP_NUM)));
    }

    // time clock start here
    let start = time::now();
    //
    let raw_time_result = join_all(list).await;

    // time clock end here;
    let end = time::now();

    // to collect time result from client
    let mut res = Vec::new();
    for task in raw_time_result{
        res.push(task.unwrap());
    }

    // handle raw time result to statistic
    let time_statistic = handle_time(res);
    server.cancel().await;
    output[ASYNC_LOCATION] = util::format_result(String::from("async"), (THREAD_NUM * LOOP_NUM) as i64,
                                                 (end - start).num_milliseconds(),
                                                 time_statistic[0], time_statistic[1],
                                                 time_statistic[2], time_statistic[3],
                                                 time_statistic[4], time_statistic[5],
                                                 time_statistic[6]);

    println!("async finished!");
}

fn main() {
    let mut output = vec![String::new(), String::new(), String::new()];

    util::print_welcome();

    // async part
    task::block_on(run_async_both(&mut output));
    // sync part
    run_sync_both(&mut output);

    util::print_result(&output);
}

