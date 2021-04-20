use clap::{crate_version, value_t, App, Arg};
use futures::executor;
use grpc::prelude::*;
use grpc::ClientConf;
use std::net::{AddrParseError, Ipv4Addr};
use std::num::ParseIntError;

use notify_remote::client::notify;
use notify_remote::notify_remote::NotificationRequest;
use notify_remote::notify_remote_grpc::NotifierClient;

fn is_ip(val: String) -> Result<(), String> {
    let ip: Result<Ipv4Addr, AddrParseError> = val.parse();
    match ip {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{}: {}", val, e.to_string())),
    }
}

fn is_port(val: String) -> Result<(), String> {
    let port: Result<u16, ParseIntError> = val.parse();
    match port {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Invalid port {}: {}", val, e.to_string())),
    }
}

fn main() {
    env_logger::init().unwrap();

    let arg_matches = App::new("notify-remote")
        .version(crate_version!())
        .author("Carlos Moro García <camorga1@gmail.com>")
        .about("Send a notification to a remote computer")
        .arg(
            Arg::with_name("ip")
                .short("o")
                .long("ip")
                .takes_value(true)
                .validator(is_ip)
                .help("The ip of the notify-remote-server"),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .validator(is_port)
                .takes_value(true)
                .default_value("5001")
                .help("The port used in the notify-remote-server"),
        )
        .arg(
            Arg::with_name("summary")
                .index(1)
                .required(true)
                .help("The summary of the notification"),
        )
        .arg(
            Arg::with_name("body")
                .index(2)
                .help("The body of the notification"),
        )
        .get_matches();

    let ip = arg_matches.value_of("ip").unwrap();
    let port = value_t!(arg_matches, "port", u16).unwrap();

    let client = NotifierClient::new_plain(ip, port, ClientConf::new()).expect("client");

    let mut notification = NotificationRequest::new();
    notification.set_summary(arg_matches.value_of("summary").unwrap().to_string());
    notification.set_body(arg_matches.value_of("body").unwrap().to_string());

    executor::block_on(async { notify(&client, notification).await });
}
