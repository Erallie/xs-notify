use crate::error::XSNotifyError;
use serde::{Deserialize, Serialize};
use tokio::{net::UdpSocket, sync::mpsc};

#[derive(Serialize, Deserialize, Debug)]
pub struct XSOverlayMessage {
    /// 1 = Notification Popup, 2 = MediaPlayer Information, will be extended later on.
    pub messageType: i32,
    /// Only used for Media Player, changes the icon on the wrist.
    pub index: i32,
    /// How long the notification will stay on screen for in seconds
    pub timeout: f32,
    /// Height notification will expand to if it has content other than a title. Default is 175
    pub height: f32,
    /// Opacity of the notification, to make it less intrusive. Setting to 0 will set to 1.
    pub opacity: f32,
    /// Notification sound volume.
    pub volume: f32,
    /// File path to .ogg audio file. Can be "default", "error", or "warning". Notification will be silent if left empty.
    pub audioPath: String,
    /// Notification title, supports Rich Text Formatting
    pub title: String,
    /// Notification content, supports Rich Text Formatting, if left empty, notification will be small.
    pub content: String,
    /// Set to true if using Base64 for the icon image
    pub useBase64Icon: bool,
    /// Base64 Encoded image, or file path to image. Can also be "default", "error", or "warning"
    pub icon: String,
    /// Somewhere to put your app name for debugging purposes
    pub sourceApp: String,
}

async fn connect_udp(host: &String, port: usize) -> Result<UdpSocket, XSNotifyError> {
    // using port 0 so the OS allocates a available port automatically
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    socket.connect(format!("{host}:{port}")).await?;
    Ok(socket)
}

pub async fn xs_notify(rx: &mut mpsc::UnboundedReceiver<XSOverlayMessage>, host: &String, port: usize) -> Result<(), XSNotifyError> {
    let socket = connect_udp(&host, port).await?;
    while let Some(msg) = rx.recv().await {
        log::info!("Sending notification from {}", msg.sourceApp);
        let data = serde_json::to_string(&msg)?;
        socket.send(data.as_bytes()).await?;
    }
    Ok(())
}
