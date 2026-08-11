//! HTML template runtime — exact copy of maud's runtime API.
//!
//! Used by the `html!` macro (our `<tag>` syntax variant of maud).

use std::{
    borrow::Cow,
    boxed::Box,
    collections::HashMap,
    fmt::{self, Arguments, Display, Write},
    string::String,
    sync::Arc,
    sync::OnceLock,
};
use tokio::sync::{broadcast, mpsc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Patch {
    pub slot: String,
    pub html: String,
}

#[derive(Clone)]
pub struct ReactiveSession {
    pub id: String,
    sender: mpsc::Sender<Patch>,
}

fn sessions() -> &'static std::sync::Mutex<HashMap<String, mpsc::Receiver<Patch>>> {
    static SESSIONS: OnceLock<std::sync::Mutex<HashMap<String, mpsc::Receiver<Patch>>>> =
        OnceLock::new();
    SESSIONS.get_or_init(|| std::sync::Mutex::new(HashMap::new()))
}

pub fn reactive_session() -> ReactiveSession {
    let id = Uuid::new_v4().to_string();
    let (tx, receiver) = mpsc::channel(32);
    sessions()
        .lock()
        .expect("reactive session registry poisoned")
        .insert(id.clone(), receiver);
    ReactiveSession { id, sender: tx }
}

impl ReactiveSession {
    pub async fn send(&self, slot: impl Into<String>, html: String) -> bool {
        self.sender
            .send(Patch {
                slot: slot.into(),
                html,
            })
            .await
            .is_ok()
    }
}

pub fn take_session(session: &str) -> Option<mpsc::Receiver<Patch>> {
    sessions().lock().ok()?.remove(session)
}

fn table_changes() -> &'static broadcast::Sender<Arc<[String]>> {
    static CHANGES: OnceLock<broadcast::Sender<Arc<[String]>>> = OnceLock::new();
    CHANGES.get_or_init(|| broadcast::channel(256).0)
}

pub fn publish_table_changes(tables: Vec<String>) {
    if !tables.is_empty() {
        let _ = table_changes().send(tables.into());
    }
}

pub fn subscribe_table_changes() -> broadcast::Receiver<Arc<[String]>> {
    table_changes().subscribe()
}

mod escape;

// ── Escaper ───────────────────────────────────────────────────────────────────

/// An adapter that escapes HTML special characters.
///
/// The following characters are escaped:
///
/// * `&` is escaped as `&amp;`
/// * `<` is escaped as `&lt;`
/// * `>` is escaped as `&gt;`
/// * `"` is escaped as `&quot;`
///
/// All other characters are passed through unchanged.
pub struct Escaper<'a>(&'a mut String);

impl<'a> Escaper<'a> {
    /// Creates an `Escaper` from a `String`.
    pub fn new(buffer: &'a mut String) -> Escaper<'a> {
        Escaper(buffer)
    }
}

impl fmt::Write for Escaper<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        escape::escape_to_string(s, self.0);
        Ok(())
    }
}

// ── Render trait ──────────────────────────────────────────────────────────────

/// Represents a type that can be rendered as HTML.
///
/// To implement this for your own type, override either the `.render()`
/// or `.render_to()` methods; since each is defined in terms of the
/// other, you only need to implement one of them.
///
/// # Example
///
/// ```rust
/// use html_rt::{html_rt as _, Markup, Render};
///
/// pub struct Stylesheet(&'static str);
///
/// impl Render for Stylesheet {
///     fn render(&self) -> Markup {
///         html_macro::html! {
///             <link rel="stylesheet" type="text/css" href={self.0} />
///         }
///     }
/// }
/// ```
pub trait Render {
    /// Renders `self` as a block of `Markup`.
    fn render(&self) -> Markup {
        let mut buffer = String::new();
        self.render_to(&mut buffer);
        PreEscaped(buffer)
    }

    /// Appends a representation of `self` to the given buffer.
    ///
    /// Its default implementation just calls `.render()`, but you may
    /// override it with something more efficient.
    ///
    /// Note that no further escaping is performed on data written to
    /// the buffer. If you override this method, you must make sure that
    /// any data written is properly escaped, whether by hand or using
    /// the [`Escaper`] wrapper struct.
    fn render_to(&self, buffer: &mut String) {
        buffer.push_str(&self.render().into_string());
    }
}

impl Render for str {
    fn render_to(&self, w: &mut String) {
        escape::escape_to_string(self, w);
    }
}

impl Render for String {
    fn render_to(&self, w: &mut String) {
        str::render_to(self, w);
    }
}

impl Render for Cow<'_, str> {
    fn render_to(&self, w: &mut String) {
        str::render_to(self, w);
    }
}

impl Render for Arguments<'_> {
    fn render_to(&self, w: &mut String) {
        let _ = Escaper::new(w).write_fmt(*self);
    }
}

impl<T: Render + ?Sized> Render for &T {
    fn render_to(&self, w: &mut String) {
        T::render_to(self, w);
    }
}

impl<T: Render + ?Sized> Render for &mut T {
    fn render_to(&self, w: &mut String) {
        T::render_to(self, w);
    }
}

impl<T: Render + ?Sized> Render for Box<T> {
    fn render_to(&self, w: &mut String) {
        T::render_to(self, w);
    }
}

impl<T: Render + ?Sized> Render for Arc<T> {
    fn render_to(&self, w: &mut String) {
        T::render_to(self, w);
    }
}

macro_rules! impl_render_with_display {
    ($($ty:ty)*) => {
        $(
            impl Render for $ty {
                fn render_to(&self, w: &mut String) {
                    format_args!("{self}", self = self).render_to(w);
                }
            }
        )*
    };
}

impl_render_with_display! {
    char f32 f64
}

macro_rules! impl_render_with_itoa {
    ($($ty:ty)*) => {
        $(
            impl Render for $ty {
                fn render_to(&self, w: &mut String) {
                    w.push_str(itoa::Buffer::new().format(*self));
                }
            }
        )*
    };
}

impl_render_with_itoa! {
    i8 i16 i32 i64 i128 isize
    u8 u16 u32 u64 u128 usize
}

// ── display() ─────────────────────────────────────────────────────────────────

/// Renders a value using its [`Display`] impl (with HTML escaping).
///
/// # Example
///
/// ```rust
/// use std::net::Ipv4Addr;
/// use html_rt::display;
///
/// let ip = Ipv4Addr::new(127, 0, 0, 1);
/// // html! { {display(ip)} }  →  "127.0.0.1"
/// ```
pub fn display(value: impl Display) -> impl Render {
    struct DisplayWrapper<T>(T);

    impl<T: Display> Render for DisplayWrapper<T> {
        fn render_to(&self, w: &mut String) {
            format_args!("{0}", self.0).render_to(w);
        }
    }

    DisplayWrapper(value)
}

// ── PreEscaped / Markup ───────────────────────────────────────────────────────

/// A wrapper that renders the inner value **without** escaping.
#[derive(Debug, Clone, Copy)]
pub struct PreEscaped<T>(pub T);

impl<T: AsRef<str>> Render for PreEscaped<T> {
    fn render_to(&self, w: &mut String) {
        w.push_str(self.0.as_ref());
    }
}

/// A block of markup is a string that does not need to be escaped.
///
/// The `html!` macro expands to an expression of this type.
pub type Markup = PreEscaped<String>;

impl<T: Into<String>> PreEscaped<T> {
    /// Converts the inner value to a `String`.
    pub fn into_string(self) -> String {
        self.0.into()
    }
}

impl<T: Into<String>> From<PreEscaped<T>> for String {
    fn from(value: PreEscaped<T>) -> String {
        value.into_string()
    }
}

impl<T: Default> Default for PreEscaped<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

// ── DOCTYPE ───────────────────────────────────────────────────────────────────

/// The literal string `<!DOCTYPE html>`.
///
/// In `html!`, use it as `{DOCTYPE}` (via a splice expr).
pub const DOCTYPE: PreEscaped<&'static str> = PreEscaped("<!DOCTYPE html>");

// ── axum: IntoResponse for Markup ─────────────────────────────────────────────

#[cfg(feature = "axum")]
mod axum_support {
    use crate::PreEscaped;
    use axum::http::{header, HeaderValue};
    use axum::response::{IntoResponse, Response};
    use std::string::String;

    impl IntoResponse for PreEscaped<String> {
        fn into_response(self) -> Response {
            let headers = [(
                header::CONTENT_TYPE,
                HeaderValue::from_static("text/html; charset=utf-8"),
            )];
            (headers, self.0).into_response()
        }
    }
}

// ── macro_private — used by html! expansion ───────────────────────────────────

#[doc(hidden)]
pub mod macro_private {
    use crate::{display, Render};
    use std::{fmt::Display, string::String};

    #[doc(hidden)]
    #[macro_export]
    macro_rules! render_to {
        ($x:expr, $buffer:expr) => {{
            use $crate::macro_private::*;
            match ChooseRenderOrDisplay($x) {
                x => (&&x).implements_render_or_display().render_to(x.0, $buffer),
            }
        }};
    }

    pub use render_to;

    pub struct ChooseRenderOrDisplay<T>(pub T);

    pub struct ViaRenderTag;
    pub struct ViaDisplayTag;

    pub trait ViaRender {
        fn implements_render_or_display(&self) -> ViaRenderTag {
            ViaRenderTag
        }
    }
    pub trait ViaDisplay {
        fn implements_render_or_display(&self) -> ViaDisplayTag {
            ViaDisplayTag
        }
    }

    impl<T: Render> ViaRender for &ChooseRenderOrDisplay<T> {}
    impl<T: Display> ViaDisplay for ChooseRenderOrDisplay<T> {}

    impl ViaRenderTag {
        pub fn render_to<T: Render + ?Sized>(self, value: &T, buffer: &mut String) {
            value.render_to(buffer);
        }
    }

    impl ViaDisplayTag {
        pub fn render_to<T: Display + ?Sized>(self, value: &T, buffer: &mut String) {
            display(value).render_to(buffer);
        }
    }
}

#[cfg(test)]
mod _html_macro_integration_tests {
    // integration tests will be in html-macro crate
}
