use crate::home::Home;
use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=Home />
                    <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>
                </Routes>
            </main>
        </Router>
    }
}
