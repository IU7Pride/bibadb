#![allow(clippy::multiple_crate_versions, clippy::module_name_repetitions)]

#[cfg(all(feature = "swagger", debug_assertions))]
use axum::{routing::get, Router};

#[allow(unused_imports)]
use prelude::*;

pub mod config;
pub mod error;
pub mod grpc;
pub mod rest;
pub mod router;
pub mod services;

#[cfg_attr(all(feature = "swagger", debug_assertions), derive(OpenApi))]
#[cfg_attr(all(feature = "swagger", debug_assertions), openapi(
    paths(
        rest::hello,
    ),
    components(
        schemas(
            // Some Structs,
        )
    ),
    tags(
        (name = "biba", description = "BibaBD API")
    )
))]
pub struct ApiDoc;

/// Generate openapi documentation for the project
///
/// # Panics
///
/// Panics if `OpenAPI` couldn't be converted into YAML format
#[cfg(all(feature = "swagger", debug_assertions))]
#[allow(clippy::expect_used)]
pub fn openapi_doc() -> Router {
    use utoipa_rapidoc::RapiDoc;
    use utoipa_redoc::{Redoc, Servable};
    use utoipa_swagger_ui::SwaggerUi;

    /* Swagger-only routes */
    tracing::info!("Swagger ui available at /swagger-ui");

    /* Mount Swagger ui */
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        // There is no need to create `RapiDoc::with_openapi` because the OpenApi is served
        // via SwaggerUi instead we only make rapidoc to point to the existing doc.
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        .route(
            "/api-docs/openapi.yaml",
            get(|| async {
                ApiDoc::openapi()
                    .to_yaml()
                    .expect("Couldn't produce .yaml API scheme")
            }),
        )
    // Alternative to above
    // .merge(RapiDoc::with_openapi("/api-docs/openapi2.json", ApiDoc::openapi()).path("/rapidoc"))
}

pub mod prelude {
    pub use crate::error::AppError;
    pub use crate::ApiDoc;
    pub use axum::{
        async_trait,
        response::{IntoResponse, Response, Result as AxumResult},
        Extension, Json, Router,
    };
    pub use error_stack::{Context, Report, Result, ResultExt};
    pub use hyper::{client::HttpConnector, Body, Method, Request, StatusCode};
    pub use serde::{Deserialize, Serialize};
    pub use std::{
        collections::{HashMap, HashSet},
        hash::Hash,
        marker::PhantomData,
        str::FromStr,
        sync::Arc,
    };
    pub use thiserror::Error;
    #[cfg(all(feature = "swagger", debug_assertions))]
    pub use utoipa::{
        openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
        IntoParams, Modify, OpenApi, PartialSchema, ToSchema,
    };
    pub use uuid::Uuid;
}

pub mod main {
    pub mod prelude {
        pub use crate::grpc::*;
        #[cfg(all(feature = "swagger", debug_assertions))]
        pub use crate::openapi_doc;
        pub use crate::rest::api_router_v1;
        pub use crate::{
            config::{ConfigExt, LoggerExt},
            prelude::*,
            router::{ApiV1, ApiVersion, NoApi, RouterApiExt},
            ApiDoc,
        };
        pub use axum::{
            error_handling::HandleErrorLayer, middleware::from_fn_with_state, BoxError, Extension,
            Router,
        };
        pub use cli::Parser;
        pub use error_stack::{Result, ResultExt};
        pub use hyper::{Method, StatusCode};
        pub use std::{env, path::PathBuf};
        pub use tower::ServiceBuilder;
        pub use tower_http::{cors::CorsLayer, services::ServeDir};
        pub use tower_sessions::{MemoryStore, SessionManagerLayer};
        pub use tracing::Level;
        pub use uuid::Uuid;
    }
}
