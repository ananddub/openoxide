use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskDesiredState {
    Running,
    Shutdown,
    Accepted,
}

impl fmt::Display for TaskDesiredState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Running => "running",
            Self::Shutdown => "shutdown",
            Self::Accepted => "accepted",
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskFilter {
    Id(String),
    Label(String, String),
    Name(String),
    Node(String),
    DesiredState(TaskDesiredState),
    UpToDate(bool),
    IsTask(bool),
}

impl TaskFilter {
    pub fn id(v: impl Into<String>) -> Self {
        Self::Id(v.into())
    }
    pub fn label(k: impl Into<String>, v: impl Into<String>) -> Self {
        Self::Label(k.into(), v.into())
    }
    pub fn name(v: impl Into<String>) -> Self {
        Self::Name(v.into())
    }
    pub fn node(v: impl Into<String>) -> Self {
        Self::Node(v.into())
    }
    pub fn desired_state(v: TaskDesiredState) -> Self {
        Self::DesiredState(v)
    }
    pub fn up_to_date(v: bool) -> Self {
        Self::UpToDate(v)
    }
    pub fn is_task(v: bool) -> Self {
        Self::IsTask(v)
    }
}

impl fmt::Display for TaskFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Id(v) => write!(f, "id={v}"),
            Self::Label(k, v) => write!(f, "label={k}={v}"),
            Self::Name(v) => write!(f, "name={v}"),
            Self::Node(v) => write!(f, "node={v}"),
            Self::DesiredState(v) => write!(f, "desired-state={v}"),
            Self::UpToDate(b) => write!(f, "up-to-date={b}"),
            Self::IsTask(b) => write!(f, "is-task={b}"),
        }
    }
}
