use leptos::prelude::*;
use crate::components::hero::Hero;
use crate::components::about::About;
use crate::components::projects::Projects;
use crate::components::contact::Contact;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main class="bg-gray-900 text-white min-h-screen font-sans selection:bg-purple-500 selection:text-white">
            <Hero />
            <About />
            <Projects />
            <Contact />
        </main>
    }
}
