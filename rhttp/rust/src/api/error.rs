use crate::api::http::HttpResponseBody;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub enum RhttpError {
    RhttpCancelError,
    RhttpTimeoutError,
    RhttpRedirectError,
    RhttpStatusCodeError(u16, Vec<(String, String)>, HttpResponseBody),
    RhttpInvalidCertificateError(String),
    RhttpInvalidPeerCertificateError,
    RhttpNoTlsInfoFoundError,
    RhttpParsingPeerCertificateError,
    RhttpConnectionError(String),
    RhttpUnknownError(String),
}

// Flutter Rust Bridge only supports anyhow, so we define string constants for the error messages.
pub(crate) const STREAM_CANCEL_ERROR: &str = "STREAM_CANCEL_ERROR";

impl Display for RhttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RhttpError::RhttpCancelError => write!(f, "RhttpCancelError"),
            RhttpError::RhttpTimeoutError => write!(f, "RhttpTimeoutError"),
            RhttpError::RhttpRedirectError => write!(f, "RhttpRedirectError"),
            RhttpError::RhttpStatusCodeError(i, _, _) => {
                write!(f, "RhttpStatusCodeError: {i}")
            }
            RhttpError::RhttpInvalidCertificateError(d) => {
                write!(f, "RhttpInvalidCertificateError: {d}")
            }
            RhttpError::RhttpConnectionError(e) => write!(f, "RhttpConnectionError: {e}"),
            RhttpError::RhttpUnknownError(e) => write!(f, "{}", e),
            RhttpError::RhttpInvalidPeerCertificateError => {
                write!(f, "RhttpInvalidPeerCertificateError")
            }
            RhttpError::RhttpNoTlsInfoFoundError => write!(f, "RhttpNoTlsInfoFoundError"),
            RhttpError::RhttpParsingPeerCertificateError => {
                write!(f, "RhttpParsingPeerCertificateError")
            }
        }
    }
}

impl std::error::Error for RhttpError {}
