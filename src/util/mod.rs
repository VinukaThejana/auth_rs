pub mod verify;

use base64::prelude::*;
use rand::Rng;
use serde::{Deserialize, Deserializer};
use std::{sync::Arc, time::SystemTime};
use tokio::signal::{self};

use crate::{config::state::AppState, error::AppError};

pub fn now() -> usize {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .try_into()
        .unwrap()
}

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

pub fn generate_otp() -> String {
    let mut rng = rand::rng();
    (0..6)
        .map(|_| rng.random_range(0..=9).to_string())
        .collect()
}

pub async fn validate_otp(state: AppState, key: &str, otp: &str) -> Result<(), AppError> {
    let mut conn = state.get_redis_conn().await.map_err(AppError::Other)?;

    let value: Option<String> = redis::pipe()
        .arg("GET")
        .arg(key)
        .arg("DEL")
        .arg(key)
        .ignore()
        .query_async(&mut conn)
        .await
        .map_err(|err| AppError::Other(err.into()))?;
    let value = value.ok_or(AppError::OTPInvalid(anyhow::anyhow!("otp is invalid")))?;
    if value != otp {
        return Err(AppError::OTPInvalid(anyhow::anyhow!("otp is invalid")));
    }

    Ok(())
}
