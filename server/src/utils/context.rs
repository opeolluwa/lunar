use std::sync::Arc;
use std::collections::HashSet;

use axum::http::HeaderMap;
use sea_orm::DatabaseConnection;
use seaography::async_graphql::{self, Context};
use uuid::Uuid;

use crate::{
    adapters::jwt::Claims,
    errors::app_error::AppError,
    repositories::{
        base::Repository,
        revoked_tokens::{TokenBlacklistRepository, TokenBlacklistRepositoryTrait},
    },
    services::workspace_member_service::WorkspaceMemberService,
};

pub struct RequestContext<'a> {
    pub db_conn: &'a DatabaseConnection,
    pub api_key: &'a str,
}

pub fn extract_request_context<'a>(ctx: &'a Context<'_>) -> Result<RequestContext<'a>, AppError> {
    let db_conn = ctx
        .data::<DatabaseConnection>()
        .map_err(|err| AppError::InternalError(err.message))?;

    let headers = ctx
        .data::<HeaderMap>()
        .map_err(|_| AppError::InternalError("Missing request headers".to_string()))?;

    let api_key = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| AppError::InternalError("Missing Authorization header".to_string()))?;

    Ok(RequestContext { db_conn, api_key })
}

pub fn extract_db_conn<'a>(ctx: &'a Context<'_>) -> Result<&'a DatabaseConnection, AppError> {
    ctx.data::<DatabaseConnection>()
        .map_err(|err| AppError::InternalError(err.message))
}

/// Extract and validate JWT claims from the Authorization header that
/// `orchard.rs` injects into the GraphQL request context. Also honours the
/// token blacklist, mirroring the REST authentication middleware.
pub async fn extract_claims(ctx: &Context<'_>) -> Result<Claims, AppError> {
    let db = extract_db_conn(ctx)?;

    let headers = ctx
        .data::<HeaderMap>()
        .map_err(|_| AppError::InternalError("Missing request headers".to_string()))?;

    let authorization = headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .ok_or(AppError::InvalidToken)?;

    let token = authorization.strip_prefix("Bearer ").unwrap_or(authorization);
    let claims = Claims::from_token(token).map_err(|_| AppError::InvalidToken)?;

    if let Some(jti) = claims.jti {
        let blacklist = TokenBlacklistRepository::init(&Arc::new(db.clone()));
        if blacklist
            .is_revoked(&jti)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
        {
            return Err(AppError::InvalidToken);
        }
    }

    Ok(claims)
}

/// Reject the mutation unless the caller is a member of every referenced
/// workspace.
pub async fn ensure_workspace_access(
    ctx: &Context<'_>,
    workspace_identifiers: Vec<Uuid>,
) -> async_graphql::Result<()> {
    let unique: Vec<Uuid> = workspace_identifiers
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    if unique.is_empty() {
        return Ok(());
    }

    let claims = extract_claims(ctx)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;
    let db = extract_db_conn(ctx)?;
    let members = WorkspaceMemberService::init(&Arc::new(db.clone()));

    for workspace_identifier in unique {
        members
            .assert_member(workspace_identifier, &claims.email)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
    }

    Ok(())
}
