use std::sync::{OnceLock, mpsc};

use rovapi_models::Stop;

static CHANNEL: OnceLock<(mpsc::Sender<Message>, egui::Context)> = OnceLock::new();

#[must_use]
pub enum Message {
    FetchedStops(Vec<Stop>),
}

impl Message {
    pub fn init(egui_ctx: egui::Context) -> mpsc::Receiver<Self> {
        let (tx, rx) = mpsc::channel();
        CHANNEL
            .set((tx, egui_ctx))
            .expect("Message channel already initialized");
        rx
    }

    pub fn send(self) {
        let (channel, egui_ctx) = CHANNEL.get().expect("CHANNEL wasn't initialized");

        if channel.send(self).is_err() {
            log::error!("CHANNEL closed, failed to send message");
            return;
        }

        egui_ctx.request_repaint();
    }
}
