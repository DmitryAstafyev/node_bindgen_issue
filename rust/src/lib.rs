pub mod api;
mod signal;

use api::{Controller, API};
use signal::Signal;
use std::collections::HashMap;
use tokio::{
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    time::{sleep, Duration},
};
use tokio_util::sync::CancellationToken;

pub struct Session {
    rx: Option<UnboundedReceiver<API>>,
    session_api: Controller,
}

impl Session {
    pub fn new() -> (Self, Controller) {
        let (tx, rx): (UnboundedSender<API>, UnboundedReceiver<API>) = unbounded_channel();
        let session_api = Controller::new(tx);
        (
            Self {
                rx: Some(rx),
                session_api: session_api.clone(),
            },
            session_api,
        )
    }

    pub async fn init(&mut self, confirmation: CancellationToken) -> Result<(), String> {
        let mut rx = self
            .rx
            .take()
            .ok_or(String::from("Session already exists"))?;
        let session_api = self.session_api.clone();
        tokio::spawn(async move {
            let mut jobs: HashMap<u64, Signal> = HashMap::new();
            println!("Rust: waiting for commands");
            while let Some(api) = rx.recv().await {
                jobs.retain(|_id, signal| {
                    let cancelled = signal.is_cancelled();
                    !cancelled
                });
                match api {
                    API::Sleep(tx_result, id, delay) => {
                        let signal = Signal::new(String::from("Sleep"));
                        jobs.insert(id, signal.clone());
                        let api = session_api.clone();
                        tokio::spawn(async move {
                            println!("Job \"Sleep\" has been called");
                            sleep(Duration::from_millis(delay)).await;
                            if tx_result.send(()).is_err() {
                                eprintln!("Fail to send result from sleeping");
                            }
                            signal.confirm();
                            let _ = api.remove_command(id);
                        });
                    }
                    API::Cancel(id) => {
                        if let Some(signal) = jobs.get(&id) {
                            signal.invoke();
                            println!("Cancel signal has been sent to job {} ({id})", signal.alias);
                        } else {
                            println!("Fail to cancel job; id {id} doesn't exist.");
                        }
                    }
                    API::Shutdown(tx) => {
                        jobs.iter().for_each(|(_uuid, signal)| {
                            signal.invoke();
                        });
                        for (_id, signal) in jobs.iter() {
                            signal.confirmed().await;
                        }
                        jobs.clear();
                        if tx.send(()).is_err() {
                            eprintln!("Fail to send shutdown confirmation");
                        }
                        break;
                    }
                    API::Remove(id) => if jobs.remove(&id).is_some() {},
                }
            }
            confirmation.cancel();
            println!("Rust: loop is closed");
        });
        Ok(())
    }
}
