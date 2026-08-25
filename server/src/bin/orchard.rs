use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    sync::Arc,
    time::Duration,
};

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    http::{header, Method, StatusCode},
    response::{self, IntoResponse},
    routing::get,
    Router,
};
use dotenv::dotenv;
use lunar::{data_engine, error::LunarError};
use orchard_lib::{
    config::{AppConfig, Environment},
    errors::app_error::AppError,
    routes::router::load_routes,
    shutdown::shutdown_signal,
    states::GraphQlState,
};
use seaography::async_graphql;
use tokio::net::TcpListener;
use tower_http::{
    cors::{Any, CorsLayer},
    timeout::TimeoutLayer,
    trace::{self, TraceLayer},
};
use tracing::Level;
use tracing_subscriber::EnvFilter;

use orchard_migration::{Migrator, MigratorTrait};

#[axum::debug_handler]
async fn graphql_playground(
    State(GraphQlState { endpoint, .. }): State<GraphQlState>,
) -> impl IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new(&endpoint)))
}

#[axum::debug_handler]
async fn graphql_handler(
    State(GraphQlState { schema, .. }): State<GraphQlState>,
    headers: axum::http::HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let req = req.into_inner().data(headers);
    let response = schema.execute(req).await;
    if response.is_err() {
        tracing::warn!(errors = ?response.errors, "GraphQL errors in response");
    }
    response.into()
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenv().ok();

    let app_config = AppConfig::from_env()?;

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(Level::INFO.into()))
        .init();

    let cors = if app_config.environment == Environment::Production {
        CorsLayer::new()
            .allow_origin(app_config.allowed_origins.clone())
            .allow_methods([
                Method::GET,
                Method::POST,
                Method::PUT,
                Method::DELETE,
                Method::OPTIONS,
            ])
            .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
    } else {
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any)
    };

    let lunar = data_engine::DataEngine::new(&app_config.database_url).await?;

    lunar
        .run_migrations()
        .await
        .map_err(|e| LunarError::DbConnectError(e.to_string()))?;

    let db = lunar.connection().to_owned();
    let db_conn = Arc::new(db.clone());

    Migrator::up(&db, None)
        .await
        .map_err(|e| LunarError::DbConnectError(e.to_string()))?;

    let schema = orchard_lib::query_root::schema(db, Some(100), app_config.complexity_limit)
        .map_err(|err| AppError::GraphQLError(err.to_string()))?;

    let graphql_state = GraphQlState {
        schema,
        endpoint: app_config.graphql_endpoint.clone(),
    };

    let http_routes = load_routes(&db_conn);

    let graphql_router = Router::new()
        .route(
            &app_config.graphql_endpoint,
            get(graphql_playground).post(graphql_handler),
        )
        .with_state(graphql_state);

    let app = Router::new()
        .merge(graphql_router)
        .merge(http_routes)
        .layer(cors)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(25),
        ));

    let ip_address = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, app_config.port));
    tracing::info!(
        "Visit GraphQL Playground at http://{}{}",
        ip_address,
        app_config.graphql_endpoint
    );
    tracing::info!("Service health check at http://{}/health", ip_address,);
    axum::serve(
        TcpListener::bind(ip_address)
            .await
            .map_err(|e| AppError::InternalError(format!("failed to bind to {ip_address}: {e}")))?,
        app,
    )
    .with_graceful_shutdown(shutdown_signal())
    .await
    .map_err(|err| AppError::InternalError(err.to_string()))?;

    Ok(())
}
