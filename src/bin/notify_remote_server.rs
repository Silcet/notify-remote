use notify_remote::{notify_remote_grpc::NotifierServer, server::NotifierImpl};
use std::thread;

fn main() {
    let service_def = NotifierServer::new_service_def(NotifierImpl::new());

    let mut server_builder = grpc::ServerBuilder::new_plain();
    server_builder.add_service(service_def);
    server_builder.http.set_port(5001);
    let server = server_builder.build().expect("build");

    println!("Server started on addr {}", server.local_addr());

    loop {
        thread::park();
    }
}
