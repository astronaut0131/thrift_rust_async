use thrift::server;
use async_std::task;
use async_std::io::Error;
use crate::tutorial::{CalculatorSyncProcessor, CalculatorSyncHandler};

mod client;
mod tutorial;

async fn run_server(){
    let processor = CalculatorSyncProcessor::new(PartHandler {});
    let mut s = server::server_main::TServer::new(processor);

    s.listen("127.0.0.1:9090").await;
}

async fn run_client(){
    client::run_client("127.0.0.1:9090").await;
}

async fn run() {
    futures::join!(run_server(), run_client());
}

fn main() {
    task::block_on(run());
}

struct PartHandler;

impl CalculatorSyncHandler for PartHandler {
    fn handle_add(&self, num1: i32, num2: i32) -> thrift::Result<i32> {
        Ok(num1 + num2)
    }
}