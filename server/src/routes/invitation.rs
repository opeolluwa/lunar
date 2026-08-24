use std::sync::Arc;

use axum::{
    routing::{delete, get, post},
    Router,
};

use crate::{
    handlers::invitation::{
        accept_invitation, invite_workspace_member, list_workspace_members, remove_workspace_member,
        revoke_invitation,
    },
    states::AppState,
};

pub(super) fn invitation_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/workspaces/{workspace_id}/invitations",
            post(invite_workspace_member),
        )
        .route("/invitations/accept", post(accept_invitation))
        .route(
            "/invitations/{invitation_id}",
            delete(revoke_invitation),
        )
        .route(
            "/workspaces/{workspace_id}/members",
            get(list_workspace_members),
        )
        .route(
            "/workspaces/{workspace_id}/members/{member_id}",
            delete(remove_workspace_member),
        )
        .with_state(state)
}
