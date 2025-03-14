use auth_rs::auth_proto::auth_service_server::AuthServiceServer;
use auth_rs::config::state::AppState;
use auth_rs::util::shutdown_signal;
use auth_rs::{config::ENV, service::auth};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let state = AppState::new().await;

    println!("server running on [::1]:{}", ENV.port);

    Server::builder()
        .add_service(AuthServiceServer::new(auth::Service::new(state.clone())))
        .serve_with_shutdown(
            format!("127.0.0.1:{}", ENV.port).parse()?,
            shutdown_signal(),
        )
        .await?;

    println!("closing connections ... ");
    if let Err(e) = state.shutdown().await {
        eprintln!("error closing connections : {:?}", e);
    } else {
        println!("connections closed successfully");
    }
    println!("server stopped gracefully");

    Ok(())
}
