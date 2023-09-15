use tokio::time::{sleep, Duration};
use tokio_util::sync::CancellationToken;

pub struct Session {}

impl Session {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn sleep(&mut self, delay: u64) -> Result<(), String> {
        let confirmation = CancellationToken::new();
        let inner = confirmation.clone();
        tokio::spawn(async move {
            println!("Rust: sleeping");
            sleep(Duration::from_millis(delay)).await;
            inner.cancel();
            println!("Rust: wakeup");
        });
        confirmation.cancelled().await;
        Ok(())
    }
}
