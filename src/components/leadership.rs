use dioxus::prelude::*;

struct LeadershipPillar {
    icon_d: &'static str,
    title: &'static str,
    tagline: &'static str,
    description: &'static str,
    practices: &'static [&'static str],
    badge: &'static str,
    badge_color: &'static str,
}

const PILLARS: &[LeadershipPillar] = &[
    LeadershipPillar {
        icon_d: "M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10",
        title: "Domain-Driven & Distributed Architecture",
        tagline: "Translating messy real-world complexity into decoupled, resilient boundaries.",
        description: "At ZF Group, I decompose multi-tier vehicle platforms into well-defined bounded contexts. By employing Domain-Driven Design (DDD) and asynchronous event-driven buses (Amazon EventBridge, SQS), we isolate high-frequency in-vehicle telemetry from core business services, ensuring extreme fault isolation.",
        practices: &[
            "Architectural Decision Records (ADRs) for transparent consensus",
            "Event-driven pub/sub avoiding fragile point-to-point couplings",
            "Graceful degradation & offline-first vehicle caching",
        ],
        badge: "System Design",
        badge_color: "bg-indigo-950/70 text-indigo-300 border-indigo-500/30",
    },
    LeadershipPillar {
        icon_d: "M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z",
        title: "Developer Velocity & Virtualization Tooling",
        tagline: "Unblocking teams by solving systemic engineering bottlenecks.",
        description: "When scarce physical hardware units threatened to delay multiple development teams, I designed and built a customized QEMU-based virtual simulator for macOS and Linux. This eliminated hardware bench wait-times, reduced local onboarding friction, and unlocked headless automated testing in CI.",
        practices: &[
            "Hardware-free developer onboarding in minutes, not weeks",
            "Automated end-to-end simulation pipelines in CI/CD",
            "Eliminating cross-team dependency blocking with virtual mocks",
        ],
        badge: "DevOps & DX",
        badge_color: "bg-emerald-950/70 text-emerald-300 border-emerald-500/30",
    },
    LeadershipPillar {
        icon_d: "M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z",
        title: "Team Mentorship, Standards & Guild Leadership",
        tagline: "Fostering engineering rigor, psychological safety, and quality culture.",
        description: "I believe great leadership creates autonomous, high-trust teams. At Telstra, I served as UI Guild Lead, driving WCAG accessibility compliance across 6+ squads. I lead architectural spikes, conduct empathetic yet rigorous code reviews, and mentor engineers from junior to staff levels.",
        practices: &[
            "Comprehensive code review standards focused on maintainability & security",
            "Accessible-first engineering (WCAG AAA/AA compliance)",
            "Hands-on pairing and 1-on-1 technical roadmap mentorship",
        ],
        badge: "Culture & People",
        badge_color: "bg-purple-950/70 text-purple-300 border-purple-500/30",
    },
    LeadershipPillar {
        icon_d: "M13 10V3L4 14h7v7l9-11h-7z",
        title: "Pragmatic Modernization & Agentic Workflows",
        tagline: "Leveraging cutting-edge tools to amplify engineering output.",
        description: "Modern leadership requires assessing new technology objectively. I actively champion Rust and WebAssembly for mission-critical reliability where memory safety and zero runtime overhead are required, and integrate agentic AI workflows to accelerate architectural prototyping and developer productivity.",
        practices: &[
            "Targeted Rust & WebAssembly adoption for deterministic performance",
            "Agentic AI tooling to automate boilerplate and accelerate discovery",
            "Continuous telemetry, observability, and SLO-driven performance audits",
        ],
        badge: "Innovation",
        badge_color: "bg-cyan-950/70 text-cyan-300 border-cyan-500/30",
    },
];

#[component]
pub fn Leadership() -> Element {
    rsx! {
        section {
            id: "leadership",
            class: "py-24 px-4 sm:px-6 lg:px-8 bg-slate-900/40 relative border-t border-slate-800/80",
            div { class: "max-w-7xl mx-auto",
                // Section Header
                div { class: "text-center max-w-3xl mx-auto mb-16",
                    div { class: "inline-flex items-center gap-2 px-3 py-1 rounded-full bg-slate-800 text-slate-300 text-xs font-mono mb-3 border border-slate-700",
                        span { class: "w-1.5 h-1.5 rounded-full bg-indigo-400" }
                        "LEADERSHIP PHILOSOPHY"
                    }
                    h2 { class: "text-3xl sm:text-5xl font-extrabold text-white tracking-tight mb-4",
                        "Leading Through Architecture, Autonomy & Velocity"
                    }
                    p { class: "text-base sm:text-lg text-slate-300",
                        "A Tech Lead's true output is measured not just in lines of code written, but in the clarity of system boundaries established, developer friction removed, and team capability elevated."
                    }
                }

                // 4 Leadership Pillars Grid
                div { class: "grid md:grid-cols-2 gap-8 mb-16",
                    for p in PILLARS.iter() {
                        div { class: "glass-card p-8 rounded-2xl flex flex-col justify-between border border-slate-800 relative group overflow-hidden",
                            div { class: "absolute top-0 right-0 w-32 h-32 bg-indigo-500/5 rounded-full blur-2xl -mr-10 -mt-10 pointer-events-none group-hover:bg-indigo-500/10 transition duration-500" }

                            div {
                                div { class: "flex items-center justify-between mb-5",
                                    div { class: "w-12 h-12 rounded-xl bg-slate-800/90 border border-slate-700/80 flex items-center justify-center text-indigo-400 shadow-inner",
                                        svg {
                                            class: "w-6 h-6",
                                            fill: "none",
                                            stroke: "currentColor",
                                            view_box: "0 0 24 24",
                                            path {
                                                stroke_linecap: "round",
                                                stroke_linejoin: "round",
                                                stroke_width: "1.5",
                                                d: "{p.icon_d}",
                                            }
                                        }
                                    }
                                    span { class: "text-xs font-mono px-3 py-1 rounded-full border {p.badge_color}",
                                        "{p.badge}"
                                    }
                                }

                                h3 { class: "text-xl sm:text-2xl font-bold text-white mb-2 group-hover:text-indigo-300 transition duration-200",
                                    "{p.title}"
                                }
                                p { class: "text-xs sm:text-sm font-medium text-indigo-400/90 mb-4 font-mono",
                                    "{p.tagline}"
                                }
                                p { class: "text-sm text-slate-300 leading-relaxed mb-6",
                                    "{p.description}"
                                }
                            }

                            div { class: "pt-5 border-t border-slate-800/80 mt-auto",
                                div { class: "text-xs font-semibold text-slate-400 uppercase tracking-wider mb-3",
                                    "Key Leadership Practices:"
                                }
                                ul { class: "space-y-2",
                                    for item in p.practices.iter() {
                                        li { class: "flex items-start gap-2.5 text-xs sm:text-sm text-slate-300",
                                            svg {
                                                class: "w-4 h-4 text-emerald-400 mt-0.5 shrink-0",
                                                fill: "none",
                                                stroke: "currentColor",
                                                view_box: "0 0 24 24",
                                                path {
                                                    stroke_linecap: "round",
                                                    stroke_linejoin: "round",
                                                    stroke_width: "2.5",
                                                    d: "M5 13l4 4L19 7",
                                                }
                                            }
                                            span { "{item}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Guiding Operating Principles Quote Card
                div { class: "glass-panel p-8 sm:p-10 rounded-2xl border border-indigo-500/20 max-w-5xl mx-auto relative overflow-hidden",
                    div { class: "absolute -right-10 -bottom-10 w-48 h-48 bg-indigo-600/10 rounded-full blur-3xl pointer-events-none" }
                    div { class: "grid md:grid-cols-3 gap-8 text-center md:text-left divide-y md:divide-y-0 md:divide-x divide-slate-800",
                        div { class: "pt-4 md:pt-0 md:pr-6",
                            div { class: "text-indigo-400 font-mono text-xs mb-2", "TENET 01" }
                            h4 { class: "text-white font-bold text-base mb-2", "Architecture Serves Product" }
                            p { class: "text-xs text-slate-400 leading-relaxed",
                                "Avoid over-engineering. Build systems simple enough for humans to understand, and modular enough to scale under production load."
                            }
                        }
                        div { class: "pt-6 md:pt-0 md:px-6",
                            div { class: "text-cyan-400 font-mono text-xs mb-2", "TENET 02" }
                            h4 { class: "text-white font-bold text-base mb-2", "Unblock the Flow of Work" }
                            p { class: "text-xs text-slate-400 leading-relaxed",
                                "The greatest multiplier on team velocity is eliminating friction: automated CI, local emulation, and clear interface boundaries."
                            }
                        }
                        div { class: "pt-6 md:pt-0 md:pl-6",
                            div { class: "text-purple-400 font-mono text-xs mb-2", "TENET 03" }
                            h4 { class: "text-white font-bold text-base mb-2", "Autonomy Requires Standards" }
                            p { class: "text-xs text-slate-400 leading-relaxed",
                                "Give engineers high autonomy by agreeing on crisp contracts, testing baselines, and shared architectural guidelines."
                            }
                        }
                    }
                }
            }
        }
    }
}
