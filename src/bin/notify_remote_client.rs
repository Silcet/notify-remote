use futures::executor;
use grpc::prelude::*;
use grpc::ClientConf;

use notify_remote::client::notify;
use notify_remote::notify_remote_grpc::NotifierClient;

fn main() {
    env_logger::init().unwrap();

    let client = NotifierClient::new_plain("127.0.0.1", 5001, ClientConf::new()).expect("client");

    executor::block_on(async { notify(&client).await });
}
