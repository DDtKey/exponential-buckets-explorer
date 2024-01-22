use crate::pages::BucketsExplorerPage;
use leptos::*;
use leptos_router::*;

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
