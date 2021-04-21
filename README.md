# notify-remote
Send notifications from a server to your host computer 
over GRPC and leverage your native notification system.

This project contains both the server and client scripts 
needed.

## Getting started
0. Install dependencies:

    0.1. [Install Rust](https://www.rust-lang.org/tools/install)

    0.2. [Install protobuf compiler](), needed by the grpc-protobuf crate

1. Clone, build and install the project (❤️ cargo):
    
    ```sh
    cargo install --git https://github.com/Silcet/notify-remote.git --branch main
    ```

2. Run the server locally:

    It will be served in port `5001` if no other port is specified.
    ```sh
    notify-remote-server -p <port>
    ```
    This will print the current IP of the host. Note it down for later (\<host-ip>).

3. Copy the client over to your remote worker:

    Use scp for example:

    ```sh
    scp ~/.cargo/bin/notify-remote <user>@<remote-worker-ip>:<path>
    ```
    Remember to place the binary in a folder in the `$PATH` or add the placement folder to the `$PATH`.

4. Run `notify-remote` after a long-running command:

    ```sh
    sleep 5; notify-remote --ip <host-ip> "Summary of the notification" "Body of the notification"
    ```
    This example will pop a notification in your host after 5 seconds with the chosen summary and body


## TODO
- [ ] Add tests
- [ ] Add more fileds to the notification (relevance, icon, etc..)
- [ ] Add authentication
- [ ] Handle notification errors without panic
