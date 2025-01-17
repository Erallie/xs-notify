use std::string::FromUtf8Error;

use crate::xsoverlay::XSOverlayMessage;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum XSNotifyError {
    #[error("IO Error {0}")]
    IoError(#[from] std::io::Error),
    #[error("Failed To Parse: {0}")]
    ParseError(#[from] serde_json::Error),
    #[error("Windows core error: {0}")]
    CoreError(#[from] windows::core::Error),
    #[error("Send error: {0}")]
    SendError(#[from] tokio::sync::mpsc::error::SendError<XSOverlayMessage>),
    #[error("Reqwest Error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Tauri Error: {0}")]
    TauriError(#[from] tauri::Error),
    #[error("Utf Error: {0}")]
    UtfError(#[from] FromUtf8Error),
    #[error("{0}")]
    Custom(String),
}

// Implementing From<&str> for XSNotifyError
impl From<&str> for XSNotifyError {
    fn from(s: &str) -> Self {
        XSNotifyError::Custom(s.to_string())
    }
}

// Implementing From<String> for XSNotifyError
impl From<String> for XSNotifyError {
    fn from(s: String) -> Self {
        XSNotifyError::Custom(s)
    }
}

// Implementing the `std::convert::From` trait allows you to use the `?` operator
// and other conversion methods seamlessly.
impl std::convert::From<XSNotifyError> for String {
    fn from(err: XSNotifyError) -> Self {
        err.to_string()
    }
}

// we must manually implement serde::Serialize
impl serde::Serialize for XSNotifyError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
