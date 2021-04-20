use clap::{crate_version, value_t, App, Arg};
use local_ipaddress;
use notify_remote::{notify_remote_grpc::NotifierServer, server::NotifierImpl};
use std::thread;

fn main() {
    let arg_matches = App::new("notify-remote-server")
        .version(crate_version!())
        .author("Carlos Moro García <camorga1@gmail.com>")
        .about("Send a notification to a remote computer")
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .takes_value(true)
                .default_value("5001")
                .help("The port used for GRPC communication"),
        )
        .get_matches();

    let port = value_t!(arg_matches, "port", u16).unwrap_or_else(|e| e.exit());

    let service_def = NotifierServer::new_service_def(NotifierImpl::new());

    let mut server_builder = grpc::ServerBuilder::new_plain();
    server_builder.add_service(service_def);
    server_builder.http.set_port(port);
    let _server = server_builder.build().expect("build");

    println!(
        "Server started on addr {}:{}",
        local_ipaddress::get().unwrap(),
        port
    );

    loop {
        thread::park();
    }
}
