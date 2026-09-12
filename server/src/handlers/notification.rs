use std::sync::Arc;

use axum::extract::{Path, Query, State};
use lunar::entities::notifications;
use uuid::Uuid;

use crate::{
    adapters::{
        jwt::Claims,
        pagination::{PaginatedResponse, PaginationParams},
    },
    dto::common::RowCount,
    errors::service_error::ServiceError,
    response::ApiResponse,
    services::notification_service::NotificationServiceExt,
    states::AppState,
};

pub async fn fetch_notifications(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    Query(pagination): Query<PaginationParams>,
) -> Result<ApiResponse<PaginatedResponse<Vec<notifications::Model>>>, ServiceError> {
    let notifications = state
        .services
        .notification_service
        .fetch_notifications(&claims, &pagination)
        .await?;

    Ok(ApiResponse::builder()
        .data(notifications)
        .message("fetch notifications")
        .build())
}

pub async fn count_unread(
    State(state): State<Arc<AppState>>,
    claims: Claims,
) -> Result<ApiResponse<RowCount>, ServiceError> {
    let resp = state
        .services
        .notification_service
        .count_unread(&claims)
        .await?;

    Ok(ApiResponse::builder().data(resp).build())
}

pub async fn mark_read(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    Path(notification_identifier): Path<Uuid>,
) -> Result<ApiResponse<()>, ServiceError> {
    state
        .services
        .notification_service
        .mark_read(&claims, &notification_identifier)
        .await?;

    Ok(ApiResponse::builder().message("notification read").build())
}
