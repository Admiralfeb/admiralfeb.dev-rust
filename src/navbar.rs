use leptos::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <a id="homeImage"><img src=""/></a>
        <a>Home</a>
        <a>Resume</a>
        <a>Projects</a>
    }
}
