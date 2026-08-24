use serde::{Deserialize, Serialize};
use validator::Validate;

// Body of the POST /workspace/:id/invitations request
#[derive(Debug, Deserialize, Validate, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InviteWorkspaceMemberRequest {
    #[validate(email)]
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct InviteWorkspaceMemberResponse {
    pub invitation_id: String,
    pub email: String,
    pub status: String,
}

// Body of the POST /invitations/accept request
#[derive(Debug, Deserialize, Validate, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AcceptInvitationRequest {
    #[validate(length(min = 1))]
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct AcceptedWorkspaceInfo {
    pub identifier: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize)]
pub struct AcceptInvitationResponse {
    pub message: String,
    pub workspace: Option<AcceptedWorkspaceInfo>,
}

#[derive(Debug, Serialize)]
pub struct WorkspaceMemberResponse {
    pub identifier: String,
    pub email: String,
    pub role: String,
    pub created_at: String,
}
