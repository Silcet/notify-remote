use crate::{notify_remote::*, notify_remote_grpc::*};

use grpc::{ServerHandlerContext, ServerRequestSingle, ServerResponseUnarySink};

use notify_rust::Notification;

#[derive(Default)]
pub struct NotifierImpl {}

impl NotifierImpl {
    pub fn new() -> NotifierImpl {
        NotifierImpl {}
    }
}

impl Notifier for NotifierImpl {
    fn notify(
        &self,
        _: ServerHandlerContext,
        req: ServerRequestSingle<NotificationRequest>,
        resp: ServerResponseUnarySink<NotificationReply>,
    ) -> grpc::Result<()> {
        let mut r = NotificationReply::new();
        let summary = req.message.get_summary();
        let body = req.message.get_body();

        println!("Got notification: {}, {}", summary, body);

        match Notification::new().summary(summary).body(body).show() {
            Ok(_) => r.set_return_code(0),
            Err(e) => {
                r.set_return_code(1);
                r.set_error(e.to_string());
            }
        }

        resp.finish(r)
    }
}
