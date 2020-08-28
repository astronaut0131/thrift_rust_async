use thrift::server;
use async_std::task;
use async_std::io::Error;

mod client;

pub type Result<T> = std::result::Result<T, Error>;

async fn run(){
    let mut s = crate::server::server_main::TServer::new();
    let c = client::try_run("127.0.0.1:9090");

    futures::join!(s.listen("127.0.0.1:9090"), c);
}

fn main() {
    task::block_on(run());
}