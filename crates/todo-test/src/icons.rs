use html_macro::html;
use html_rt::Markup;

pub fn square() -> Markup {
    html! {
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
            <rect width="18" height="18" x="3" y="3" rx="2"></rect>
        </svg>
    }
}

pub fn square_check() -> Markup {
    html! {
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
            <rect width="18" height="18" x="3" y="3" rx="2"></rect>
            <path d="m9 12 2 2 4-4"></path>
        </svg>
    }
}
