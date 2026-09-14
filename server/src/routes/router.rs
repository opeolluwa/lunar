use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Router};
use sea_orm::DatabaseConnection;

use crate::{
    response::ApiResponseBuilder,
    routes::{
        app::public_routes, auth::authentication_routes, country::country_routes,
        invitation::invitation_routes, notification::notification_routes, users::user_routes,
    },
    states::AppState,
};

pub fn load_routes(db_conn: &Arc<DatabaseConnection>) -> Router {
    let app_state = AppState::new(db_conn).expect("Failed to initialize app state");
    let state = Arc::new(app_state);

    Router::new()
        .merge(public_routes())
        .nest("/auth", authentication_routes(state.clone()))
        .nest("/countries", country_routes(state.clone()))
        .nest("/user", user_routes(state.clone()))
        .nest("/notifications", notification_routes(state.clone()))
        .nest("/invitations", invitation_routes(state.clone()))
        .fallback(async || {
            ApiResponseBuilder::<()>::new()
                .message(
                    "the resource you're looking does not exist or it has been permanently moved",
                )
                .status_code(StatusCode::NOT_FOUND)
                .build()
                .into_response()
        })
}
