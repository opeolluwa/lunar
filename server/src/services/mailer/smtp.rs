use lettre::{
    message::{header, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};

use super::email_sender::{EmailMessage, EmailSender};
use crate::errors::service_error::ServiceError;

#[derive(Clone)]
pub struct SmtpEmailSender {
    mailer: SmtpTransport,
}

impl SmtpEmailSender {
    pub fn new(
        host: &str,
        port: u16,
        username: &str,
        password: &str,
        encryption: &str,
    ) -> Result<Self, ServiceError> {
        let creds = Credentials::new(username.to_string(), password.to_string());

        let mailer = match encryption {
            "none" => SmtpTransport::builder_dangerous(host)
                .port(port)
                .credentials(creds)
                .build(),
            _ => SmtpTransport::relay(host)
                .map_err(|e| ServiceError::InternalError(e.to_string()))?
                .port(port)
                .credentials(creds)
                .build(),
        };

        Ok(Self { mailer })
    }
}

impl EmailSender for SmtpEmailSender {
    fn send_email(&self, message: EmailMessage) -> Result<(), ServiceError> {
        let from = format!("{} <{}>", message.from_name, message.from_address)
            .parse()
            .map_err(|e| ServiceError::InternalError(format!("invalid from address: {e}")))?;

        let to = format!("{} <{}>", message.to_name, message.to_address)
            .parse()
            .map_err(|e| ServiceError::InternalError(format!("invalid to address: {e}")))?;

        let email = Message::builder()
            .from(from)
            .to(to)
            .subject(message.subject)
            .multipart(
                MultiPart::alternative().singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_HTML)
                        .body(message.html_body),
                ),
            )
            .map_err(|e| ServiceError::InternalError(e.to_string()))?;

        self.mailer
            .send(&email)
            .map_err(|e| ServiceError::InternalError(e.to_string()))?;

        Ok(())
    }
}
