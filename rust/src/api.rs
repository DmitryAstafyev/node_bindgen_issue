use tokio::sync::mpsc::UnboundedSender;
use tokio_util::sync::CancellationToken;

#[derive(Debug)]
pub enum API {
    Sleep(CancellationToken, u64),
}

#[derive(Clone, Debug)]
pub struct Controller {
    tx: UnboundedSender<API>,
}

impl Controller {
    pub fn new(tx: UnboundedSender<API>) -> Self {
        Self { tx }
    }

    pub async fn sleep(&self, delay: u64) -> Result<(), String> {
        let done = CancellationToken::new();
        self.tx
            .send(API::Sleep(done.clone(), delay))
            .map_err(|_| String::from("Fail to send call Job::Sleep"))?;
        done.cancelled().await;
        Ok(())
    }
}
