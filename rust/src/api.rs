use tokio::sync::{mpsc::UnboundedSender, oneshot};

#[derive(Debug)]
pub enum API {
    Shutdown(oneshot::Sender<()>),
    Cancel(u64),
    Sleep(oneshot::Sender<()>, u64, u64),
    Remove(u64),
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

    pub async fn cancel(&self, operation_id: &u64) -> Result<(), String> {
        self.tx
            .send(API::Cancel(*operation_id))
            .map_err(|_| String::from("Fail to send API::Cancel"))
    }

    pub async fn sleep(&self, id: u64, delay: u64) -> Result<(), String> {
        let (tx_results, rx_results) = oneshot::channel();
        self.tx
            .send(API::Sleep(tx_results, id, delay))
            .map_err(|_| String::from("Fail to send call Job::Sleep"))?;
        rx_results.await.map_err(|e| format!("channel error: {e}"))
    }

    pub(crate) fn remove_command(&self, id: u64) -> Result<(), String> {
        self.tx
            .send(API::Remove(id))
            .map_err(|_| String::from("Fail to send call Job::SomeJob"))?;
        Ok(())
    }
}
