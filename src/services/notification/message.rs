use serde::{Deserialize, Serialize};

/// Severity of a notification, used by providers that support priority levels
/// (gotify, ntfy, pushover) and to pick decoration colors/emoji.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotificationLevel {
    Info,
    Warning,
    Critical,
}

impl NotificationLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Info => "INFO",
            Self::Warning => "WARNING",
            Self::Critical => "CRITICAL",
        }
    }

    pub fn emoji(&self) -> &'static str {
        match self {
            Self::Info => "ℹ️",
            Self::Warning => "⚠️",
            Self::Critical => "🚨",
        }
    }

    /// Discord embed color (decimal RGB).
    pub fn discord_color(&self) -> i64 {
        match self {
            Self::Info => 3_447_003,      // blue
            Self::Warning => 16_098_851,  // amber
            Self::Critical => 15_158_332, // red
        }
    }

    /// Teams MessageCard theme color (hex, no leading #).
    pub fn teams_color(&self) -> &'static str {
        match self {
            Self::Info => "0076D7",
            Self::Warning => "F5A623",
            Self::Critical => "E74C3C",
        }
    }

    /// gotify priority (0-10)
    pub fn gotify_priority(&self) -> i64 {
        match self {
            Self::Info => 3,
            Self::Warning => 6,
            Self::Critical => 9,
        }
    }

    /// ntfy priority (1-5)
    pub fn ntfy_priority(&self) -> i64 {
        match self {
            Self::Info => 3,
            Self::Warning => 4,
            Self::Critical => 5,
        }
    }
}

/// A provider-agnostic notification. Each sender renders this into whatever
/// payload shape its provider expects.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NotificationMessage {
    pub title: String,
    pub body: String,
    pub level: NotificationLevel,
    pub timestamp: i64,
    /// Optional deep link back into the panel for this event.
    pub url: Option<String>,
    /// Extra key/value context rendered as fields by providers that support it
    /// and appended to the body by those that don't.
    pub fields: Vec<(String, String)>,
}

impl NotificationMessage {
    pub fn new(title: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            body: body.into(),
            level: NotificationLevel::Info,
            timestamp: chrono::Utc::now().timestamp(),
            url: None,
            fields: Vec::new(),
        }
    }

    pub fn level(mut self, level: NotificationLevel) -> Self {
        self.level = level;
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn field(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.fields.push((key.into(), value.into()));
        self
    }

    /// Single-line summary used as a subject / notification title.
    pub fn subject(&self) -> String {
        format!("{} {}", self.level.emoji(), self.title)
    }

    /// Plain text rendering. Providers with no rich formatting use this as-is;
    /// fields and URL are folded into the text since there's nowhere else to put them.
    pub fn to_plain_text(&self) -> String {
        let mut out = format!("{}\n\n{}", self.subject(), self.body);

        for (key, value) in &self.fields {
            out.push_str(&format!("\n{}: {}", key, value));
        }

        if let Some(url) = &self.url {
            out.push_str(&format!("\n\n{}", url));
        }

        out
    }
}
