use tokio::sync::mpsc;
use tracing::error;

#[derive(Debug)]
pub struct Relay {}

impl Relay {
    pub async fn new<T: Send + 'static>(mut receiver: mpsc::Receiver<T>, sender: mpsc::Sender<T>) -> Self {
        tokio::spawn(async move {
            while let Some(message) = receiver.recv().await {
                if let Err(e) = sender.send(message).await {
                    error!("message was not sent: {}", e);
                }
            }
        });

        Self {}
    }
}
