use std::sync::Arc;

use chrono::{Local, TimeDelta};
use rand::RngExt;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::{
    errors::service_error::ServiceError,
    repositories::{
        base::Repository,
        otp::{OtpRepository, OtpRepositoryExt},
    },
};

const OTP_VALIDITY: TimeDelta = TimeDelta::minutes(5);

fn generate_otp() -> String {
    let code: u32 = rand::rng().random_range(100_000..=999_999);
    code.to_string()
}
#[derive(Debug, Clone)]
pub struct OtpService {
    otp_repository: OtpRepository,
}

impl Repository for OtpService {
    fn init(db_conn: &Arc<DatabaseConnection>) -> Self {
        Self {
            otp_repository: OtpRepository::init(db_conn),
        }
    }
}

pub(crate) trait OtpServiceExt {
    async fn new_otp_for_user(&self, user_identifier: &str) -> Result<String, ServiceError>;

    async fn validate_otp_for_user(
        &self,
        user_identifier: &Uuid,
        otp: &str,
    ) -> Result<bool, ServiceError>;
}

impl OtpServiceExt for OtpService {
    async fn new_otp_for_user(&self, user_identifier: &str) -> Result<String, ServiceError> {
        let otp = generate_otp();
        self.otp_repository
            .new_with_user(user_identifier, &otp)
            .await
            .map_err(ServiceError::from)?;

        Ok(otp)
    }

    async fn validate_otp_for_user(
        &self,
        user_identifier: &Uuid,
        otp: &str,
    ) -> Result<bool, ServiceError> {
        if let Some(stored_otp) = self
            .otp_repository
            .find_latest_by_user(user_identifier)
            .await
            .map_err(ServiceError::from)?
        {
            let now = Local::now();

            let is_not_expired = now
                .signed_duration_since(stored_otp.created_at.with_timezone(&Local))
                <= OTP_VALIDITY;
            let is_match = stored_otp.code == otp;
            Ok(is_match && is_not_expired)
        } else {
            Ok(false)
        }
    }
}
