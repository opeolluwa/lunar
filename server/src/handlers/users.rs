use std::sync::Arc;

use axum::extract::State;
use axum_typed_multipart::TypedMultipart;

use crate::services::user_service::UserServiceTrait;
use crate::{
    adapters::{
        authentication::SetNewPasswordRequest, jwt::Claims, profile::UploadProfilePictureRequest,
        request::AuthenticatedRequest, users::PartialUserProfile,
        workspace_member::AccountWorkspaceResponse,
    },
    entities::users,
    errors::{app_error::AppError, service_error::ServiceError},
    response::{ApiResponse, ApiResponseBuilder},
    states::AppState,
};

pub async fn retrieve_information(
    State(state): State<Arc<AppState>>,
    claims: Claims,
) -> Result<ApiResponse<users::Model>, ServiceError> {
    let user_data = state
        .services
        .user_service
        .retrieve_information(claims.user_identifier)
        .await?;

    Ok(ApiResponseBuilder::new()
        .data(user_data)
        .message("User's profile fetched successfully")
        .build())
}

pub async fn update_password(
    State(state): State<Arc<AppState>>,
    AuthenticatedRequest { data, claims }: AuthenticatedRequest<SetNewPasswordRequest>,
) -> Result<ApiResponse<()>, ServiceError> {
    state
        .services
        .user_service
        .update_password(&data, &claims.user_identifier)
        .await?;

    Ok(ApiResponseBuilder::new()
        .message("User's profile fetched successfully")
        .build())
}

pub async fn update_profile_picture(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    request: TypedMultipart<UploadProfilePictureRequest>,
) -> Result<ApiResponse<users::Model>, ServiceError> {
    state
        .services
        .user_service
        .update_profile_picture(request, &claims.user_identifier)
        .await?;

    let updated_profile = state
        .services
        .user_service
        .retrieve_information(claims.user_identifier)
        .await?;

    Ok(ApiResponseBuilder::new()
        .data(updated_profile)
        .message("profile updated successfully")
        .build())
}

pub async fn update_profile(
    State(state): State<Arc<AppState>>,
    AuthenticatedRequest { data, claims }: AuthenticatedRequest<PartialUserProfile>,
) -> Result<ApiResponse<users::Model>, ServiceError> {
    let updated_profile = state
        .services
        .user_service
        .update_profile(&data, &claims.user_identifier)
        .await?;

    Ok(ApiResponseBuilder::new()
        .data(updated_profile)
        .message("profile updated successfully")
        .build())
}

/// All workspaces connected to the authenticated account.
pub async fn list_account_workspaces(
    State(state): State<Arc<AppState>>,
    claims: Claims,
) -> Result<ApiResponse<Vec<AccountWorkspaceResponse>>, AppError> {
    let workspaces = state
        .services
        .workspace_member_service
        .list_workspaces_for_account(&claims.email)
        .await?;

    Ok(ApiResponseBuilder::new()
        .data(workspaces)
        .message("Account workspaces fetched successfully")
        .build())
}
