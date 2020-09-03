// async use
use async_thrift::server;
use async_std::task;
use async_std::io::Error;

mod async_thrift_test;

use futures::future::*;

// sync use
mod sync_thrift_test;

use std::thread;
use std::time::Duration;
use std::cell::RefCell;
use crate::util::{handle_time, parse_args};
use async_std::net::TcpStream;
use async_thrift::transport::async_socket::TAsyncTcpChannel;
use async_thrift::protocol::async_binary::{TAsyncBinaryInputProtocol, TAsyncBinaryOutputProtocol};
use async_thrift::transport::async_framed::{TAsyncFramedReadTransport, TAsyncFramedWriteTransport};
use async_thrift::transport::TAsyncIoChannel;
use thrift::transport::TTcpChannel;
use std::fs::File;
use std::sync::Arc;


// util
mod util;

// const
const CONFIG_LOCATION: usize = 0;
const SYNC_LOCATION: usize = 1;
const ASYNC_LOCATION: usize = 2;

// const
const RUN_CLIENT: usize = 0;
const RUN_SERVER: usize = 1;
const RUN_SYNC: usize = 2;
const RUN_ASYNC: usize = 3;
const THREAD_NUM: usize = 4;
const LOOP_NUM: usize = 5;
const ADDR: usize = 6;

// print format
const PRINT_CSV: bool = false;

///
const DEFAULT_RUN_CLIENT: &str = "true";
const DEFAULT_RUN_SERVER: &str = "true";
const DEFAULT_RUN_SYNC: &str = "false";
const DEFAULT_RUN_ASYNC: &str = "true";
const DEFAULT_THREAD_NUM: &str = "2000";
const DEFAULT_LOOP_NUM: &str = "1000";
const DEFAULT_ADDR: &str = "127.0.0.1:9090";

// run sync server and client
fn run_sync_both(output: &mut Vec<String>, args: Arc<Vec<String>>) {
    println!("begin sync benchmark...");

    if args[RUN_SERVER] == String::from("true") {
        // print config
        output[CONFIG_LOCATION] = util::format_config(args[THREAD_NUM].parse::<i32>().unwrap(),
                                                      args[LOOP_NUM].parse::<i32>().unwrap());
        // start server
        let addr = Clone::clone(&args[ADDR]);

        let server = thread::spawn(move || sync_thrift_test::server::run(addr.as_str()));
        // we need to wait the server to run
        thread::sleep(Duration::from_secs(2));

        if args[RUN_CLIENT] != String::from("true") {
            println!("server is online");
            server.join();

            return;
        }
    }

    if args[RUN_CLIENT] == String::from("true") {
        // time clock start here
        let start = time::Instant::now();

        // build client thread
        let mut list = Vec::new();


        for i in 0..args[THREAD_NUM].parse::<i32>().unwrap() {
            // to ensure tcp sync queue is enough
            let mut stream = std::net::TcpStream::connect(args[ADDR].as_str()).unwrap();
            // build client
            let loop_num = args[LOOP_NUM].parse::<i32>().unwrap();
            //
            list.push(thread::spawn(move || sync_thrift_test::client::run(stream,
                                                                          loop_num)));
        }

        // to collect time result from client
        let mut res = Vec::new();
        for task in list {
            res.push(task.join().unwrap().unwrap());
        }

        // time clock end here;
        let end = time::Instant::now();

        // handle raw time result to statistic
        let time_statistic = handle_time(res);
        output[SYNC_LOCATION] = util::format_result(String::from("sync"),
                                                    args[THREAD_NUM].parse::<i64>().unwrap() * args[LOOP_NUM].parse::<i64>().unwrap(),
                                                    (end - start).num_milliseconds(),
                                                    time_statistic[0], time_statistic[1],
                                                    time_statistic[2], time_statistic[3],
                                                    time_statistic[4], time_statistic[5],
                                                    time_statistic[6]);
    }

    println!("sync finished!");
}

// run async server and client
async fn run_async_both(output: &mut Vec<String>, args: Arc<Vec<String>>) {
    println!("begin async benchmark...");

    // print config
    output[CONFIG_LOCATION] = util::format_config(args[THREAD_NUM].parse::<i32>().unwrap(),
                                                  args[LOOP_NUM].parse::<i32>().unwrap());

    let mut server = None;
    let addr = &args[ADDR];

    if args[RUN_SERVER] == String::from("true") {
        server = Some(async_std::task::spawn(async_thrift_test::server::run_server(Clone::clone(addr))));
        if args[RUN_CLIENT] != String::from("true") {
            println!("server is online");
            server.unwrap().await;

            return;
        }
    }

    if args[RUN_CLIENT] == String::from("true") {
        // time
        let mut list = Vec::new();
        let start = time::Instant::now();

        for i in 0..args[THREAD_NUM].parse::<i32>().unwrap() {
            // build client
            list.push(async_std::task::spawn(async_thrift_test::client::run_client(Clone::clone(addr), args[LOOP_NUM].parse::<i32>().unwrap())));
        }

        // time clock start here
        let raw_time_result = join_all(list).await;

        // time clock end here;
        let end = time::Instant::now();

        // to collect time result from client
        let mut res = Vec::new();
        for task in raw_time_result {
            res.push(task.unwrap());
        }

        // handle raw time result to statistic
        let time_statistic = handle_time(res);
        if !PRINT_CSV {
            output[ASYNC_LOCATION] = util::format_result(String::from("async"), (THREAD_NUM * LOOP_NUM) as i64,
                                                         (end - start).num_milliseconds(),
                                                         time_statistic[0], time_statistic[1],
                                                         time_statistic[2], time_statistic[3],
                                                         time_statistic[4], time_statistic[5],
                                                         time_statistic[6]);
        } else {
            output[ASYNC_LOCATION] = util::format_result_csv(String::from("async"), THREAD_NUM as i64, LOOP_NUM as i64,
                                                             (end - start).num_milliseconds(),
                                                             time_statistic[0], time_statistic[1],
                                                             time_statistic[2], time_statistic[3],
                                                             time_statistic[4], time_statistic[5],
                                                             time_statistic[6]);
        }
    }

    if args[RUN_SERVER] == String::from("true") {
        server.unwrap().cancel().await;
    }

    println!("async finished!");
}

fn main() {
    let mut args: Vec<String> = vec![String::from(DEFAULT_RUN_CLIENT),
                                     String::from(DEFAULT_RUN_SERVER),
                                     String::from(DEFAULT_RUN_SYNC),
                                     String::from(DEFAULT_RUN_ASYNC),
                                     String::from(DEFAULT_THREAD_NUM),
                                     String::from(DEFAULT_LOOP_NUM),
                                     String::from(DEFAULT_ADDR)];

    parse_args(&mut args);


    let mut output = vec![String::new(), String::new(), String::new()];

    util::print_welcome();

    let arc_args = Arc::new(args);
    // async part
    if arc_args[RUN_ASYNC] == String::from("true") {
        task::block_on(run_async_both(&mut output, Arc::clone(&arc_args)));
    }
    // sync part
    if arc_args[RUN_SYNC] == String::from("true") {
        run_sync_both(&mut output, Arc::clone(&arc_args));
    }

    util::print_result(&output);
}

