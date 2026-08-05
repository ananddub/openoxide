use super::{message::NotificationMessage, senders::parse_addresses};
use crate::db::models::notif_email::NotifEmail;
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    message::{MultiPart, SinglePart, header::ContentType},
    transport::smtp::{
        authentication::Credentials,
        client::{Tls, TlsParameters},
    },
};

pub async fn send_email(cfg: &NotifEmail, msg: &NotificationMessage) -> Result<(), String> {
    let recipients = parse_addresses(&cfg.to_addresses);
    if recipients.is_empty() {
        return Err("email notification has no recipients".to_string());
    }

    let mut builder = Message::builder()
        .from(
            cfg.from_address
                .parse()
                .map_err(|e| format!("invalid from address {}: {e}", cfg.from_address))?,
        )
        .subject(msg.subject());

    for address in &recipients {
        builder = builder.to(address
            .parse()
            .map_err(|e| format!("invalid recipient address {address}: {e}"))?);
    }

    let email = builder
        .multipart(
            MultiPart::alternative().singlepart(
                SinglePart::builder()
                    .header(ContentType::TEXT_PLAIN)
                    .body(msg.to_plain_text()),
            ),
        )
        .map_err(|e| format!("could not build email: {e}"))?;

    let port =
        u16::try_from(cfg.smtp_port).map_err(|_| format!("invalid smtp port {}", cfg.smtp_port))?;

    let tls = TlsParameters::new(cfg.smtp_server.clone()).map_err(|e| {
        format!(
            "could not build TLS parameters for {}: {e}",
            cfg.smtp_server
        )
    })?;

    let mut transport = if port == 465 {
        AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&cfg.smtp_server)
            .port(port)
            .tls(Tls::Wrapper(tls))
    } else {
        AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&cfg.smtp_server)
            .port(port)
            .tls(Tls::Required(tls))
    };

    if !cfg.username.is_empty() {
        transport =
            transport.credentials(Credentials::new(cfg.username.clone(), cfg.password.clone()));
    }

    transport
        .build()
        .send(email)
        .await
        .map_err(|e| format!("smtp send failed: {e}"))?;

    Ok(())
}

pub async fn send_email_to(
    cfg: &NotifEmail,
    recipient: &str,
    msg: &NotificationMessage,
) -> Result<(), String> {
    let mut targeted = cfg.clone();
    targeted.to_addresses = serde_json::to_string(&[recipient]).map_err(|e| e.to_string())?;
    send_email(&targeted, msg).await
}
