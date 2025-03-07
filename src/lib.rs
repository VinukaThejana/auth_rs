pub mod config;
pub mod database;
pub mod error;
pub mod model;
pub mod service;
pub mod token;
pub mod util;

pub mod auth_proto {
    tonic::include_proto!("auth");
}
