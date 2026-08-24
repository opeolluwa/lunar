use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::{
    adapters::{
        invitation::{
            AcceptInvitationRequest, AcceptInvitationResponse, AcceptedWorkspaceInfo,
            InviteWorkspaceMemberRequest, InviteWorkspaceMemberResponse, WorkspaceMemberResponse,
        },
        request::AuthenticatedRequest,
    },
    errors::app_error::AppError,
    states::AppState,
};

pub async fn invite_workspace_member(
    State(state): State<Arc<AppState>>,
    Path(workspace_id): Path<Uuid>,
    AuthenticatedRequest { claims, data }: AuthenticatedRequest<InviteWorkspaceMemberRequest>,
) -> Result<(StatusCode, Json<InviteWorkspaceMemberResponse>), AppError> {
    let response = state
        .services
        .invitation_service
        .invite_member(workspace_id, &claims, &data)
        .await?;

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn accept_invitation(
    State(state): State<Arc<AppState>>,
    AuthenticatedRequest { claims, data }: AuthenticatedRequest<AcceptInvitationRequest>,
) -> Result<(StatusCode, Json<AcceptInvitationResponse>), AppError> {
    let workspace = state
        .services
        .invitation_service
        .accept(&data.token, &claims)
        .await?;

    Ok((
        StatusCode::OK,
        Json(AcceptInvitationResponse {
            message: "Invitation accepted".into(),
            workspace: Some(AcceptedWorkspaceInfo {
                identifier: workspace.identifier.to_string(),
                name: workspace.name,
                description: workspace.description,
            }),
        }),
    ))
}

pub async fn revoke_invitation(
    State(state): State<Arc<AppState>>,
    Path(invitation_id): Path<Uuid>,
    claims: crate::adapters::jwt::Claims,
) -> Result<(StatusCode, Json<AcceptInvitationResponse>), AppError> {
    state
        .services
        .invitation_service
        .revoke(invitation_id, &claims)
        .await?;

    Ok((
        StatusCode::OK,
        Json(AcceptInvitationResponse {
            message: "Invitation revoked".into(),
            workspace: None,
        }),
    ))
}

pub async fn list_workspace_members(
    State(state): State<Arc<AppState>>,
    Path(workspace_id): Path<Uuid>,
    claims: crate::adapters::jwt::Claims,
) -> Result<(StatusCode, Json<Vec<WorkspaceMemberResponse>>), AppError> {
    let members = state
        .services
        .workspace_member_service
        .list_members_for_account(workspace_id, &claims.email)
        .await?;

    Ok((
        StatusCode::OK,
        Json(
            members
                .into_iter()
                .map(|m| WorkspaceMemberResponse {
                    identifier: m.identifier.to_string(),
                    email: m.member_email,
                    role: m.role,
                    created_at: m.created_at.to_rfc3339(),
                })
                .collect(),
        ),
    ))
}

pub async fn remove_workspace_member(
    State(state): State<Arc<AppState>>,
    Path((workspace_id, member_id)): Path<(Uuid, Uuid)>,
    claims: crate::adapters::jwt::Claims,
) -> Result<StatusCode, AppError> {
    state
        .services
        .workspace_member_service
        .remove_member(workspace_id, member_id, &claims.email)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
