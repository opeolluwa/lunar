use std::sync::Arc;

use axum::{
    routing::{get, patch, post, put},
    Router,
};

use crate::{
    handlers::{
        auth::change_password,
        users::{
            list_account_workspaces, retrieve_information, update_password, update_profile,
            update_profile_picture,
        },
    },
    states::AppState,
};

pub(super) fn user_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/profile", get(retrieve_information))
        .route("/profile", put(update_profile))
        .route("/avatar", post(update_profile_picture))
        .route("/password", put(update_password))
        .route("/password", patch(change_password))
        .route("/workspaces", get(list_account_workspaces))
        .with_state(state)
}
