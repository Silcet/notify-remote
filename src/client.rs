use crate::notify_remote::*;
use crate::notify_remote_grpc::NotifierClient;

pub async fn notify<'a>(client: &NotifierClient, notification: NotificationRequest) {
    let _response = client
        .notify(grpc::RequestOptions::new(), notification)
        .drop_metadata()
        .await
        .expect("notify");
}
