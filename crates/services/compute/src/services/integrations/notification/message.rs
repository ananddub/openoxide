use serde::{Deserialize, Serialize};

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

    pub fn discord_color(&self) -> i64 {
        match self {
            Self::Info => 3_447_003,
            Self::Warning => 16_098_851,
            Self::Critical => 15_158_332,
        }
    }

    pub fn teams_color(&self) -> &'static str {
        match self {
            Self::Info => "0076D7",
            Self::Warning => "F5A623",
            Self::Critical => "E74C3C",
        }
    }

    pub fn gotify_priority(&self) -> i64 {
        match self {
            Self::Info => 3,
            Self::Warning => 6,
            Self::Critical => 9,
        }
    }

    pub fn ntfy_priority(&self) -> i64 {
        match self {
            Self::Info => 3,
            Self::Warning => 4,
            Self::Critical => 5,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NotificationMessage {
    pub title: String,
    pub body: String,
    pub level: NotificationLevel,
    pub timestamp: i64,
    pub url: Option<String>,
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

    pub fn subject(&self) -> String {
        format!("{} {}", self.level.emoji(), self.title)
    }

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
