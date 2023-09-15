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

    pub async fn init(&mut self, confirmation: CancellationToken) -> Result<(), String> {
        let mut rx = self
            .rx
            .take()
            .ok_or(String::from("Session already exists"))?;
        tokio::spawn(async move {
            let mut jobs: Vec<CancellationToken> = Vec::new();
            println!("Rust: waiting for commands");
            while let Some(api) = rx.recv().await {
                jobs.retain(|token| {
                    let cancelled = token.is_cancelled();
                    !cancelled
                });
                match api {
                    API::Sleep(tx_result, delay) => {
                        let progress = CancellationToken::new();
                        jobs.push(progress.clone());
                        tokio::spawn(async move {
                            println!("Rust: Job \"Sleep\" has been called");
                            sleep(Duration::from_millis(delay)).await;
                            if tx_result.send(()).is_err() {
                                eprintln!("Rust: Fail to send result from sleeping");
                            }
                            progress.cancel();
                        });
                    }
                    API::Shutdown(tx) => {
                        for token in jobs.iter() {
                            token.cancelled().await;
                        }
                        jobs.clear();
                        if tx.send(()).is_err() {
                            eprintln!("Rust: Fail to send shutdown confirmation");
                        }
                        break;
                    }
                }
            }
            confirmation.cancel();
            println!("Rust: loop is closed");
        });
        Ok(())
    }
}
