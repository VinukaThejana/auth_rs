pub mod verify;

use base64::prelude::*;
use serde::{Deserialize, Deserializer};
use std::{sync::Arc, time::SystemTime};
use tokio::signal::{self};

pub async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to listen for ctrl-c signal");
    };

    #[cfg(unix)]
    let terminate = async {
        use tokio::signal::unix::{SignalKind, signal};

        signal(SignalKind::terminate())
            .expect("failed to install the SIGTERM handler")
            .recv()
            .await;

        signal(SignalKind::interrupt())
            .expect("failed to install the SIGINT handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            println!("\nreceived ctrl-c signal, shutting down ... ");
        },
            _ = terminate => {
            println!("\nreceived terminate signal, shutting down ... ");
        }
    }
}

pub fn deserialize_base64<'de, D>(deserializer: D) -> Result<Arc<Vec<u8>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    let bytes = BASE64_STANDARD
        .decode(s.as_bytes())
        .map_err(serde::de::Error::custom)?;

    Ok(Arc::new(bytes))
}

pub fn deserialize_arc_str<'de, D>(deserializer: D) -> Result<Arc<str>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    Ok(Arc::from(s.as_str()))
}

pub fn time_since_epoch() -> Result<i64, anyhow::Error> {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|err| anyhow::Error::new(err).context("failed to get the duration since epoch"))?
        .as_secs()
        .try_into()
        .map_err(|err| anyhow::Error::new(err).context("failed to convert time to i64"))
}
