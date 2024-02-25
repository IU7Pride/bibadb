#![allow(
    clippy::multiple_crate_versions,
    clippy::unwrap_used,
    clippy::expect_used
)]

use biba_core::{main::prelude::*, services::MultiplexService};
use cli::Config;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let config = cli::Config::try_from(cli::Args::parse())
        .change_context(AppError::InitializationError)
        .attach_printable("Couldn't get config file.")?;

    let logger = &config.logger;

    let _guard = logger.init_logger().unwrap();
    tracing::info!("Logger: {logger:?}");

    let addr = config.address;
    tracing::info!("Listening on {addr}");

    let app = router(&config);
    #[cfg(all(feature = "swagger", debug_assertions))]
    let app = app.merge(openapi_doc());

    let reflection_service = tonic_reflection::server::Builder::configure()
        .build()
        .unwrap();
    let grpc = tonic::transport::Server::builder()
        .add_service(reflection_service)
        .add_service(GreeterServer::new(MyGreeter::default()))
        .into_service();
    let service = MultiplexService::new(app, grpc);

    axum::Server::bind(&addr)
        .serve(tower::make::Shared::new(service))
        .await
        .change_context(AppError::StartUpError)
        .attach_printable("Failed to start axum server")?;

    Ok(())
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
fn router(config: &Config) -> Router {
    let router = Router::new();

    router.nest(
        ApiV1::to_path(),
        api_router_v1()
            .expect("couldn't get API routes")
            .layer(ServiceBuilder::new().layer(config.get_cors_configuration())),
    )
}
