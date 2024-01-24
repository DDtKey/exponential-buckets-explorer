use leptos::{component, view, IntoView};

#[component]
pub(crate) fn NotFoundPage() -> impl IntoView {
    view! {
        <div class="container text-center mt-5">
            <h1 class="display-1 text-muted">404</h1>
            <p class="lead">Oops! Page Not Found</p>
            <p class="text-muted">Sorry, but the page you are looking for might be in another universe.</p>
            <a href="/" class="btn btn-primary btn-lg">Go Home</a>
        </div>
    }
}
