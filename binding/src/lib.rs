use node_bindgen::derive::node_bindgen;
use session::Session;
use std::thread;
use tokio::runtime::Runtime;
use tokio_util::sync::CancellationToken;

struct Jobs {}

#[node_bindgen]
impl Jobs {
    // Self methods
    #[node_bindgen(constructor)]
    pub fn new() -> Self {
        Self {}
    }

    #[node_bindgen(mt)]
    async fn sleep(&self, delay: i64) -> Result<(), String> {
        let mut session = Session::new();
        let rt = Runtime::new().map_err(|e| format!("Could not start tokio runtime: {e}"))?;
        let confirmation = CancellationToken::new();
        let inner = confirmation.clone();
        thread::spawn(move || {
            rt.block_on(async {
                if let Err(err) = session.sleep(delay as u64).await {
                    eprintln!("Rust: Fail to sleep unbound session: {err}");
                }
                inner.cancel();
            })
        });
        confirmation.cancelled().await;
        Ok(())
    }
}
