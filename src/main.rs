mod app;
mod home;
// mod navbar;
// mod projects;
// mod resume;

use app::App;
use leptos::*;

fn main() {
    leptos::mount_to_body(|| view! {<App/>})
}
