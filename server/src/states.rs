use std::sync::Arc;

use async_graphql::dynamic::Schema;
use axum::extract::FromRef;
use sea_orm::DatabaseConnection;
use seaography::async_graphql;

use crate::config::AppConfig;
use crate::errors::app_error::AppError;
use crate::repositories::{
    base::Repository, country::CountryRepository, invitation::InvitationRepository,
    notification::NotificationRepository, revoked_tokens::TokenBlacklistRepository,
    user::UserRepository, workspace_member::WorkspaceMemberRepository,
};
use crate::services::{
    authentication_service::AuthenticationService, country_service::CountryService,
    invitation_service::InvitationService, mailer::smtp::SmtpEmailSender,
    notification_service::NotificationService, otp_service::OtpService, root_service::RootService,
    user_service::UserService, workspace_member_service::WorkspaceMemberService,
};

#[derive(Clone)]
pub struct Repositories {
    pub user: UserRepository,
    pub otp: OtpService,
    pub token_blacklist: TokenBlacklistRepository,
    pub country: CountryRepository,
    pub notification: NotificationRepository,
    pub invitation: InvitationRepository,
    pub workspace_member: WorkspaceMemberRepository,
}

#[derive(Clone)]
pub struct Contracts {
    pub email: SmtpEmailSender,
}

#[derive(Clone)]
pub struct ServicesState {
    pub user_service: UserService,
    pub root_service: RootService,
    pub auth_service: AuthenticationService,
    pub country_service: CountryService,
    pub notification_service: NotificationService,
    pub invitation_service: InvitationService,
    pub workspace_member_service: WorkspaceMemberService,
}

#[derive(Clone)]
pub struct GraphQlState {
    pub schema: Schema,
    pub endpoint: String,
}

pub struct AppState {
    pub services: ServicesState,
    pub database_connection: Arc<DatabaseConnection>,
    pub app_config: AppConfig,
}

impl Repositories {
    pub fn new(db: &Arc<DatabaseConnection>) -> Self {
        Self {
            user: UserRepository::init(db),
            otp: OtpService::init(db),
            token_blacklist: TokenBlacklistRepository::init(db),
            country: CountryRepository::init(db),
            notification: NotificationRepository::init(db),
            invitation: InvitationRepository::init(db),
            workspace_member: WorkspaceMemberRepository::init(db),
        }
    }
}

impl Contracts {
    pub fn new(app_config: &AppConfig) -> Result<Self, AppError> {
        let email = SmtpEmailSender::new(
            &app_config.smtp_host,
            app_config.smtp_port,
            &app_config.smtp_username,
            &app_config.smtp_password,
            &app_config.smtp_encryption,
        )
        .map_err(|e| AppError::OperationFailed(e.to_string()))?;

        Ok(Self { email })
    }
}

impl ServicesState {
    pub fn new(
        db_conn: &Arc<DatabaseConnection>,
        repos: Repositories,
        contracts: Contracts,
    ) -> Self {
        let member_service = WorkspaceMemberService::new(repos.workspace_member.clone());
        Self {
            user_service: UserService::new(repos.user.clone()),
            auth_service: AuthenticationService::new(
                repos.user,
                repos.otp,
                repos.token_blacklist,
                contracts.email,
            ),
            root_service: RootService::init(),
            country_service: CountryService::new(repos.country),
            notification_service: NotificationService::new(repos.notification),
            workspace_member_service: member_service.clone(),
            invitation_service: InvitationService::new(
                db_conn.clone(),
                repos.invitation,
                member_service,
            ),
        }
    }
}

impl FromRef<Arc<AppState>> for ServicesState {
    fn from_ref(state: &Arc<AppState>) -> Self {
        state.services.clone()
    }
}

impl FromRef<Arc<AppState>> for AuthenticationService {
    fn from_ref(state: &Arc<AppState>) -> AuthenticationService {
        state.services.auth_service.clone()
    }
}

impl FromRef<Arc<AppState>> for UserService {
    fn from_ref(state: &Arc<AppState>) -> UserService {
        state.services.user_service.clone()
    }
}

impl FromRef<Arc<AppState>> for CountryService {
    fn from_ref(state: &Arc<AppState>) -> CountryService {
        state.services.country_service.clone()
    }
}

impl FromRef<Arc<AppState>> for NotificationService {
    fn from_ref(state: &Arc<AppState>) -> NotificationService {
        state.services.notification_service.clone()
    }
}

impl FromRef<Arc<AppState>> for InvitationService {
    fn from_ref(state: &Arc<AppState>) -> InvitationService {
        state.services.invitation_service.clone()
    }
}

impl AppState {
    pub fn new(db: &DatabaseConnection) -> Result<Self, AppError> {
        let app_config = AppConfig::from_env()?;
        let db = Arc::new(db.clone());
        let contracts = Contracts::new(&app_config)?;
        let repositories = Repositories::new(&db);
        let services = ServicesState::new(&db, repositories, contracts);

        Ok(Self {
            services,
            database_connection: db,
            app_config,
        })
    }
}
