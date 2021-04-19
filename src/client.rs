use crate::notify_remote::*;
use crate::notify_remote_grpc::NotifierClient;

pub async fn notify(client: &NotifierClient) {
    let mut message = NotificationRequest::new();
    message.message = "Hi there this is way longer".to_string();
    println!("Sending: {:?}", message);

    let response = client
        .notify(grpc::RequestOptions::new(), message)
        .drop_metadata()
        .await
        .expect("notify");

    println!("{:?}", response);
    println!(
        "Return: {} | Error: {}",
        response.return_code, response.error
    );
}
