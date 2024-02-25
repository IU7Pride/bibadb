use crate::{
    prelude::*,
    router::{ApiV1, ApiVersion, RouteError, RouterApiExt},
    ApiDoc,
};

pub fn api_router_v1() -> Result<Router<()>, RouteError> {
    Router::new()
        .with_context::<ApiV1, ApiDoc>()
        .api_route("/hello", &Method::GET, hello)
        .unwrap()
}

/// Hi
#[allow(clippy::unused_async)]
#[cfg_attr(all(feature = "swagger", debug_assertions), utoipa::path(
        get,
        context_path = ApiV1::to_path(),
        path = "/hello",
        responses(
            (status = 200, description = "Hello World!")
        )
    ))]
pub async fn hello() -> &'static str {
    "Hello World!"
}
