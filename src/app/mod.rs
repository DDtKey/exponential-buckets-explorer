use crate::pages::{BucketsExplorerPage, NotFoundPage};
use leptos::*;
use leptos_router::*;

#[component]
pub(crate) fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="/" view=BucketsExplorerPage />
                <Route path="/*any" view=NotFoundPage />
            </Routes>
        </Router>
    }
}
