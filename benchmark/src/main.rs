// async
use async_thrift::server;
use async_std::task;
use async_std::io::Error;
mod async_thrift_test;


// sync
mod original_thrift_test;
use std::thread;


fn run_sync_both(){
    thread::spawn(|| original_thrift_test::server::run());
    original_thrift_test::client::run();
}

async fn run_async_both() {
    async_std::task::spawn(async_thrift_test::server::run_server("127.0.0.1:9090"));
    let client = async_thrift_test::client::run_client("127.0.0.1:9090");
    client.await;
}

fn main() {
    // run_sync_both();
    task::block_on(run_async_both());
}