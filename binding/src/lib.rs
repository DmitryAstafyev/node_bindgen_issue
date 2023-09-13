use node_bindgen::derive::node_bindgen;
use session::{api::Controller, Session};
use std::{convert::TryFrom, thread};
use tokio::runtime::Runtime;
use tokio_util::sync::CancellationToken;

fn u64_from_i64(id: i64) -> Result<u64, String> {
    u64::try_from(id).map_err(|_| String::from("ID of job is invalid"))
}

struct Jobs {
    api: Option<Controller>,
    confirmation: CancellationToken,
}

#[node_bindgen]
impl Jobs {
    // Self methods
    #[node_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            api: None,
            confirmation: CancellationToken::new(),
        }
    }

    #[node_bindgen(mt)]
    async fn init(&mut self) -> Result<(), String> {
        let (mut session, api) = Session::new();
        self.api = Some(api);
        let rt = Runtime::new().map_err(|e| format!("Could not start tokio runtime: {e}"))?;
        let confirmation = self.confirmation.clone();
        thread::spawn(move || {
            rt.block_on(async {
                if let Err(err) = session.init(confirmation.clone()).await {
                    eprintln!("Fail to init unbound session: {err}");
                } else {
                    println!("Rust: Unbound session is started");
                    confirmation.cancelled().await;
                    println!("Rust: Unbound session is closed");
                }
            })
        });
        Ok(())
    }

    #[node_bindgen]
    async fn destroy(&self) -> Result<(), String> {
        self.api
            .as_ref()
            .ok_or(String::from("Session unavailable"))?
            .shutdown()
            .await?;
        Ok(())
    }

    #[node_bindgen]
    async fn abort(&self, id: i64) -> Result<(), String> {
        self.api
            .as_ref()
            .ok_or(String::from("Session unavailable"))?
            .cancel(&u64_from_i64(id)?)
            .await
    }

    #[node_bindgen]
    async fn sleep(&self, id: i64, delay: i64) -> Result<(), String> {
        self.api
            .as_ref()
            .ok_or(String::from("Session unavailable"))?
            .sleep(u64_from_i64(id)?, u64_from_i64(delay)?)
            .await
    }
}
