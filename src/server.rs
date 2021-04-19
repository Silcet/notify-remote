use crate::{notify_remote::*, notify_remote_grpc::*};

use grpc::{ServerHandlerContext, ServerRequestSingle, ServerResponseUnarySink};

#[derive(Default)]
pub struct NotifierImpl;

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
        let message = req.message.get_message();
        println!("Got message: {}", message);
        if message.len() > 10 {
            r.set_return_code(1);
            r.set_error("Message is too long".to_string());
        } else {
            r.set_return_code(0);
            r.set_error("".to_string());
        }
        resp.finish(r)
    }
}
