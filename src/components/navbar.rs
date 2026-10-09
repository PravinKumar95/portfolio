use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    let mut menu_open = use_signal(|| false);

    rsx! {
        header { class: "fixed top-0 left-0 right-0 z-50 glass-panel border-b border-slate-800/80 transition-all duration-300",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 h-18 flex items-center justify-between",
                // Brand / Monogram
                a {
                    class: "flex items-center gap-3 group",
                    href: "#",
                    div { class: "w-10 h-10 rounded-xl bg-gradient-to-tr from-indigo-600 via-indigo-500 to-cyan-400 p-[1px] shadow-lg shadow-indigo-500/20 group-hover:shadow-indigo-500/40 transition duration-300",
                        div { class: "w-full h-full bg-slate-950 rounded-[11px] flex items-center justify-center",
                            span { class: "text-sm font-black tracking-wider text-transparent bg-clip-text bg-gradient-to-r from-indigo-300 to-cyan-300",
                                "PK"
                            }
                        }
                    }
                    div { class: "flex flex-col",
                        span { class: "text-base font-bold text-white tracking-tight group-hover:text-indigo-300 transition duration-200",
                            "Pravin Kumar"
                        }
                        span { class: "text-xs font-mono text-slate-400 flex items-center gap-1.5",
                            span { class: "inline-block w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse" }
                            "Tech Lead & Systems Architect"
                        }
                    }
                }

                // Desktop Nav Links
                nav { class: "hidden md:flex items-center gap-1 lg:gap-2",
                    a {
                        class: "px-3 py-2 text-sm font-medium text-slate-300 hover:text-white hover:bg-slate-800/60 rounded-lg transition duration-200",
                        href: "#overview",
                        "Overview"
                    }
                    a {
                        class: "px-3 py-2 text-sm font-medium text-slate-300 hover:text-white hover:bg-slate-800/60 rounded-lg transition duration-200",
                        href: "#leadership",
                        "Leadership"
                    }
                    a {
                        class: "px-3 py-2 text-sm font-medium text-slate-300 hover:text-white hover:bg-slate-800/60 rounded-lg transition duration-200",
                        href: "#architecture",
                        "Architecture & Impact"
                    }
                    a {
                        class: "px-3 py-2 text-sm font-medium text-slate-300 hover:text-white hover:bg-slate-800/60 rounded-lg transition duration-200",
                        href: "#competencies",
                        "Tech Matrix"
                    }
                    a {
                        class: "px-3 py-2 text-sm font-medium text-slate-300 hover:text-white hover:bg-slate-800/60 rounded-lg transition duration-200",
                        href: "#experience",
                        "Experience"
                    }
                    a {
                        class: "px-3 py-2 text-sm font-medium text-slate-300 hover:text-white hover:bg-slate-800/60 rounded-lg transition duration-200",
                        href: "#research",
                        "Research"
                    }
                }

                // Action Button & Mobile Toggle
                div { class: "flex items-center gap-3",
                    a {
                        class: "hidden sm:inline-flex items-center justify-center text-xs font-semibold px-4 py-2 rounded-lg bg-indigo-600 hover:bg-indigo-500 text-white shadow-sm shadow-indigo-500/25 transition duration-200",
                        href: "#contact",
                        "Let's Connect"
                    }
                    button {
                        class: "md:hidden p-2 rounded-lg text-slate-400 hover:text-white hover:bg-slate-800/80 focus:outline-none",
                        aria_label: "Toggle navigation menu",
                        onclick: move |_| menu_open.toggle(),
                        svg {
                            class: "w-6 h-6",
                            fill: "none",
                            stroke: "currentColor",
                            view_box: "0 0 24 24",
                            if menu_open() {
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M6 18L18 6M6 6l12 12",
                                }
                            } else {
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M4 6h16M4 12h16M4 18h16",
                                }
                            }
                        }
                    }
                }
            }

            // Mobile Drawer
            if menu_open() {
                div { class: "md:hidden px-4 pt-2 pb-4 bg-slate-900/95 border-b border-slate-800 backdrop-blur-xl space-y-1",
                    a {
                        class: "block px-3 py-2 text-sm font-medium text-slate-200 hover:bg-slate-800 rounded-lg",
                        href: "#overview",
                        onclick: move |_| menu_open.set(false),
                        "Overview"
                    }
                    a {
                        class: "block px-3 py-2 text-sm font-medium text-slate-200 hover:bg-slate-800 rounded-lg",
                        href: "#leadership",
                        onclick: move |_| menu_open.set(false),
                        "Leadership"
                    }
                    a {
                        class: "block px-3 py-2 text-sm font-medium text-slate-200 hover:bg-slate-800 rounded-lg",
                        href: "#architecture",
                        onclick: move |_| menu_open.set(false),
                        "Architecture & Impact"
                    }
                    a {
                        class: "block px-3 py-2 text-sm font-medium text-slate-200 hover:bg-slate-800 rounded-lg",
                        href: "#competencies",
                        onclick: move |_| menu_open.set(false),
                        "Tech Matrix"
                    }
                    a {
                        class: "block px-3 py-2 text-sm font-medium text-slate-200 hover:bg-slate-800 rounded-lg",
                        href: "#experience",
                        onclick: move |_| menu_open.set(false),
                        "Experience"
                    }
                    a {
                        class: "block px-3 py-2 text-sm font-medium text-slate-200 hover:bg-slate-800 rounded-lg",
                        href: "#research",
                        onclick: move |_| menu_open.set(false),
                        "Research"
                    }
                    div { class: "pt-2 border-t border-slate-800",
                        a {
                            class: "block w-full text-center px-4 py-2.5 rounded-lg bg-indigo-600 text-white font-semibold text-sm",
                            href: "#contact",
                            onclick: move |_| menu_open.set(false),
                            "Let's Connect"
                        }
                    }
                }
            }
        }
    }
}
