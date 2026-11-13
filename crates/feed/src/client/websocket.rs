use async_trait::async_trait;
use futures_util::StreamExt;
use tokio_tungstenite::connect_async;

pub struct WSClient {

}

// impl WSClient {
//     pub async fn connect(url: &str) -> Result<Self, String> {
//         let (stream, response) = connect_async(url).await?;
//         let (write, read) = stream.split();
//
//     }
// }

#[async_trait]
pub trait WebsocketClient {
    async fn connect() -> Self;
    async fn on_message() -> Result<(), String>;
    async fn on_error(e: String);
    async fn on_close() -> Result<(), String>;
    async fn send_ping() -> Result<(), String>;
}