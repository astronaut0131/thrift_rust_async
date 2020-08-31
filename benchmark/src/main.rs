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

    output[CONFIG_LOCATION] = util::format_config(THREAD_NUM, LOOP_NUM);

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

    output[SYNC_LOCATION] = util::format_result(String::from("sync"), THREAD_NUM * LOOP_NUM, (end - start).num_milliseconds());

    println!("sync finished!");
}

// run async server and client
async fn run_async_both(output: &mut Vec<String>) {
    println!("begin async benchmark...");

    let server = async_std::task::spawn(async_thrift_test::server::run_server("127.0.0.1:9090"));
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
    server.cancel().await;
    output[ASYNC_LOCATION] = util::format_result(String::from("async"), THREAD_NUM * LOOP_NUM, (end - start).num_milliseconds());

    println!("async finished!");
}

// print welcome
fn print_welcome(){
    println!("******************************************");
    println!("*        E-01 benchmark for rust rpc     *");
    println!("*             Version : 0.1.0            *");
    println!("******************************************");
    println!("---------------------------   Benchmark Start! --------------------------");
}

// print all the result
fn print_result(output: &Vec<String>){
    println!();
    println!();
    println!("---------------------------   Benchmark Finished! --------------------------");
    for line in output {
        println!();
        println!("{}", line);
    }
}

fn main() {
    let mut output = vec![String::new(), String::new(), String::new()];

    print_welcome();

    task::block_on(run_async_both(&mut output));
    thread::sleep(Duration::from_secs(2));
    run_sync_both(&mut output);

    print_result(&output);
}

