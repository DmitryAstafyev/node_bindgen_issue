pub mod api;

use api::{Controller, API};
use tokio::{
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    time::{sleep, Duration},
};
use tokio_util::sync::CancellationToken;

pub struct Session {
    rx: Option<UnboundedReceiver<API>>,
}

impl Session {
    pub fn new() -> (Self, Controller) {
        let (tx, rx): (UnboundedSender<API>, UnboundedReceiver<API>) = unbounded_channel();
        let session_api = Controller::new(tx);
        (Self { rx: Some(rx) }, session_api)
    }

    pub async fn init(&mut self) -> Result<(), String> {
        let mut rx = self
            .rx
            .take()
            .ok_or(String::from("Session already exists"))?;
        let shutdown = CancellationToken::new();
        let inner = shutdown.clone();
        tokio::spawn(async move {
            println!("Rust: waiting for commands");
            while let Some(api) = rx.recv().await {
                match api {
                    API::Sleep(tx_result, delay) => {
                        let task = inner.clone();
                        tokio::spawn(async move {
                            println!("Rust: Job \"Sleep\" has been called");
                            sleep(Duration::from_millis(delay)).await;
                            if tx_result.send(()).is_err() {
                                eprintln!("Rust: Fail to send result from sleeping");
                            }
                            task.cancel();
                        });
                    }
                }
            }
            println!("Rust: loop is closed");
        });
        shutdown.cancelled().await;
        Ok(())
    }
}
