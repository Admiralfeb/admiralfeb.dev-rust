use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div>
            <h1 class="text-center">"Welcome to admiralfeb.dev - rust-leptos"</h1>
            <div class="container">
                <div class="jumbotron">
                    <h2 class="text-center">"I'm Austyn and this is my portfolio."</h2>
                    <div class="container">
                        <img width=300 src="" />
                        <div>
                            <p>"For my resume/ccv click " <a href="resume">"here"</a></p>
                            <p>"For my other projects click " <a href="projects">"here"</a></p>
                            <p>"Finally, to contact me, email me at " <a href="mailto:austyn@admiralfeb.dev" target="_blank">"Austyn@admiralfeb.dev"</a></p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
