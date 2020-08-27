pub mod server;
pub mod transport;

use async_std::io::Error;

/// Result type returned by all runtime library functions.
///
/// As is convention this is a typedef of `std::result::Result`
/// with `E` defined as the `thrift::Error` type.
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use async_std::task;
    use crate::server;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn include() {
        server::server_main::test();
    }

    #[test]
    fn server() {
        let mut s = server::server_main::TServer::new();
        println!("here");
        task::block_on(s.listen("127.0.0.1:9090"));
    }
}
