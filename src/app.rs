use crate::components::architecture::Architecture;
use crate::components::contact::Contact;
use crate::components::experience::Experience;
use crate::components::hero::Hero;
use crate::components::leadership::Leadership;
use crate::components::navbar::Navbar;
use crate::components::research::Research;
use crate::components::skills::Skills;
use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/public/style.css") }
        div { class: "bg-slate-950 text-slate-100 min-h-screen font-sans selection:bg-indigo-500 selection:text-white flex flex-col",
            Navbar {}
            main { class: "flex-grow",
                Hero {}
                Leadership {}
                Architecture {}
                Skills {}
                Experience {}
                Research {}
            }
            Contact {}
        }
    }
}
