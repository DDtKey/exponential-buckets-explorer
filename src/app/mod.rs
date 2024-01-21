use leptos::*;
use leptos_router::*;
use crate::pages::BucketsExplorerPage;

#[component]
pub(crate) fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="" view=BucketsExplorerPage />
            </Routes>
        </Router>
    }
}