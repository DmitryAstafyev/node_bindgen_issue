use tokio::sync::{mpsc::UnboundedSender, oneshot};

#[derive(Debug)]
pub enum API {
    Shutdown(oneshot::Sender<()>),
    Sleep(oneshot::Sender<()>, u64),
}

#[derive(Clone, Debug)]
pub struct Controller {
    tx: UnboundedSender<API>,
}

impl Controller {
    pub fn new(tx: UnboundedSender<API>) -> Self {
        Self { tx }
    }

    pub async fn shutdown(&self) -> Result<(), String> {
        let (tx, rx): (oneshot::Sender<()>, oneshot::Receiver<()>) = oneshot::channel();
        self.tx
            .send(API::Shutdown(tx))
            .map_err(|_| String::from("Fail to send API::Shutdown"))?;
        rx.await
            .map_err(|e| format!("Fail to get response from API::Shutdown: {e:?}"))
    }

    pub async fn sleep(&self, delay: u64) -> Result<(), String> {
        let (tx_results, rx_results) = oneshot::channel();
        self.tx
            .send(API::Sleep(tx_results, delay))
            .map_err(|_| String::from("Fail to send call Job::Sleep"))?;
        rx_results.await.map_err(|e| format!("channel error: {e}"))
    }
}
