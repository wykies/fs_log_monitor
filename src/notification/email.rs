use std::fs;
use std::path::Path;

use anyhow::Context;
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct EmailConfig {
    from_name: String,
    pass: String,
    #[serde(default = "EmailConfig::default_from_email")]
    from_email: String,
    #[serde(default = "EmailConfig::default_to_email")]
    to_email: String,
    #[serde(default = "EmailConfig::default_subject")]
    subject: String,
}

impl EmailConfig {
    fn default_from_email() -> String {
        "wykies.notices@gmail.com".to_string()
    }
    fn default_to_email() -> String {
        "it@wykies.com".to_string()
    }
    fn default_subject() -> String {
        "Notification from connection monitor".to_string()
    }
}

pub struct Email {
    from_mailbox: Mailbox,
    to_mailbox: Mailbox,
    subject: String,
    transport: SmtpTransport,
}
impl Email {
    pub fn new(config_folder: &Path) -> anyhow::Result<Self> {
        let filename = config_folder.join("e.data");
        let file_contents = fs::read_to_string(&filename)
            .with_context(|| format!("failed to read email settings from {filename:?}"))?;
        let email_config: EmailConfig = serde_json::from_str(&file_contents)
            .with_context(|| format!("failed to parse contents of {filename:?} as email config"))?;
        let from_mailbox = Mailbox {
            name: Some(email_config.from_name),
            email: email_config
                .from_email
                .parse()
                .context("failed to parse from email address")?,
        };
        let to_mailbox: Mailbox = email_config
            .to_email
            .parse()
            .context("failed to parse to email address")?;
        let transport = SmtpTransport::relay("smtp.gmail.com")
            .context("failed to build SmtpTransport")?
            .credentials(Credentials::new(email_config.from_email, email_config.pass))
            .build();
        let subject = email_config.subject;
        Ok(Self {
            from_mailbox,
            to_mailbox,
            subject,
            transport,
        })
    }

    pub fn send(msg: &str, config_folder: &Path) -> anyhow::Result<()> {
        let email = Self::new(config_folder)?;

        let email_msg = Message::builder()
            .from(email.from_mailbox.clone())
            .to(email.to_mailbox.clone())
            .subject(email.subject.clone())
            .body(msg.to_string())?;
        email
            .transport
            .send(&email_msg)
            .context("failed to send email")?;
        Ok(())
    }
}
