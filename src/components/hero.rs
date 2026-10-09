use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    rsx! {
        section {
            id: "overview",
            class: "relative pt-32 pb-20 md:pt-40 md:pb-28 px-4 sm:px-6 lg:px-8 overflow-hidden glow-mesh",
            // Ambient Decorative Elements
            div { class: "absolute top-1/4 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[600px] h-[350px] bg-gradient-to-tr from-indigo-600/15 via-purple-600/10 to-cyan-400/10 blur-[120px] rounded-full pointer-events-none -z-10" }

            div { class: "max-w-6xl mx-auto text-center relative z-10",
                // Executive Lead Badge
                div { class: "inline-flex items-center gap-2 px-3.5 py-1.5 rounded-full bg-indigo-950/70 border border-indigo-500/30 text-indigo-300 text-xs font-mono uppercase tracking-wider mb-6 shadow-sm shadow-indigo-500/20 backdrop-blur-md",
                    span { class: "flex h-2 w-2 relative",
                        span { class: "animate-ping absolute inline-flex h-full w-full rounded-full bg-indigo-400 opacity-75" }
                        span { class: "relative inline-flex rounded-full h-2 w-2 bg-indigo-500" }
                    }
                    span { "Tech Lead & Systems Architect" }
                    span { class: "text-slate-500", "|" }
                    span { class: "text-slate-400", "Edge Embedded ⇄ Cloud Scale" }
                }

                // Headline
                h1 { class: "text-4xl sm:text-6xl lg:text-7xl font-extrabold tracking-tight text-white mb-6 leading-[1.1]",
                    "Architecting Resilient Platforms."
                    br { class: "hidden sm:inline" }
                    span { class: "text-transparent bg-clip-text bg-gradient-to-r from-indigo-400 via-purple-300 to-cyan-400",
                        " Leading High-Velocity Teams."
                    }
                }

                // Mission Description
                p { class: "text-lg sm:text-xl lg:text-2xl text-slate-300 max-w-4xl mx-auto font-normal leading-relaxed mb-10",
                    "I am a software architect and engineering lead with "
                    span { class: "text-white font-semibold", "7+ years of experience" }
                    " delivering mission-critical systems. Currently at "
                    span { class: "text-white font-semibold", "ZF Group" }
                    ", I lead the architecture of distributed fleet orchestration platforms spanning embedded Linux devices, C# .NET microservices, and AWS serverless infrastructure."
                }

                // CTA Actions
                div { class: "flex flex-wrap items-center justify-center gap-4 mb-16",
                    a {
                        class: "px-7 py-3.5 rounded-xl bg-gradient-to-r from-indigo-600 to-indigo-500 hover:from-indigo-500 hover:to-indigo-400 text-white font-semibold text-sm shadow-lg shadow-indigo-500/25 transition duration-200 transform hover:-translate-y-0.5 flex items-center gap-2",
                        href: "#architecture",
                        span { "Explore Architecture Case Studies" }
                        svg {
                            class: "w-4 h-4",
                            fill: "none",
                            stroke: "currentColor",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M19 9l-7 7-7-7",
                            }
                        }
                    }

                    a {
                        class: "px-7 py-3.5 rounded-xl bg-slate-900/80 hover:bg-slate-800 text-slate-200 hover:text-white font-semibold text-sm border border-slate-700/80 hover:border-slate-600 transition duration-200 flex items-center gap-2 backdrop-blur-sm",
                        href: "#leadership",
                        span { "Leadership Philosophy" }
                    }

                    a {
                        class: "px-7 py-3.5 rounded-xl text-slate-300 hover:text-white font-semibold text-sm hover:bg-slate-800/40 border border-transparent hover:border-slate-800 transition duration-200",
                        href: "#contact",
                        "Get in Touch"
                    }
                }

                // Executive Highlights Ribbon / Metrics
                div { class: "grid grid-cols-2 md:grid-cols-4 gap-4 max-w-5xl mx-auto pt-6 border-t border-slate-800/80",
                    div { class: "glass-card p-5 rounded-2xl text-left",
                        div { class: "text-3xl font-extrabold text-white tracking-tight mb-1 flex items-baseline gap-1",
                            "7+"
                            span { class: "text-indigo-400 text-lg font-bold", "Years" }
                        }
                        div { class: "text-xs font-semibold text-slate-300 uppercase tracking-wider mb-1",
                            "Production Systems"
                        }
                        p { class: "text-xs text-slate-400 leading-normal",
                            "From low-level embedded hardware to cloud distributed microservices."
                        }
                    }

                    div { class: "glass-card p-5 rounded-2xl text-left",
                        div { class: "text-3xl font-extrabold text-white tracking-tight mb-1 flex items-baseline gap-1",
                            "Fleet"
                            span { class: "text-cyan-400 text-lg font-bold", "Scale" }
                        }
                        div { class: "text-xs font-semibold text-slate-300 uppercase tracking-wider mb-1",
                            "Platform Lead (ZF)"
                        }
                        p { class: "text-xs text-slate-400 leading-normal",
                            "Connected vehicle orchestration with EventBridge, SQS, & DDD."
                        }
                    }

                    div { class: "glass-card p-5 rounded-2xl text-left",
                        div { class: "text-3xl font-extrabold text-white tracking-tight mb-1 flex items-baseline gap-1",
                            "60%"
                            span { class: "text-emerald-400 text-lg font-bold", "DX Gain" }
                        }
                        div { class: "text-xs font-semibold text-slate-300 uppercase tracking-wider mb-1",
                            "Virtualization DX"
                        }
                        p { class: "text-xs text-slate-400 leading-normal",
                            "Engineered QEMU simulators unblocking multi-team embedded dev."
                        }
                    }

                    div { class: "glass-card p-5 rounded-2xl text-left",
                        div { class: "text-3xl font-extrabold text-white tracking-tight mb-1 flex items-baseline gap-1",
                            "1"
                            span { class: "text-purple-400 text-lg font-bold", "Peer-Reviewed" }
                        }
                        div { class: "text-xs font-semibold text-slate-300 uppercase tracking-wider mb-1",
                            "Robotics Publication"
                        }
                        p { class: "text-xs text-slate-400 leading-normal",
                            "CCD & Frenet-Serret parameterisation in International Robotics Journal."
                        }
                    }
                }

                // Primary Tech Matrix Chips
                div { class: "mt-12 flex flex-wrap items-center justify-center gap-2",
                    span { class: "text-xs text-slate-400 font-mono mr-2", "Core Tech Focus:" }
                    span { class: "tech-tag border-indigo-500/40 text-indigo-300 bg-indigo-950/40", "C# / .NET 8" }
                    span { class: "tech-tag border-cyan-500/40 text-cyan-300 bg-cyan-950/40", "AWS Serverless & EventBridge" }
                    span { class: "tech-tag border-emerald-500/40 text-emerald-300 bg-emerald-950/40", "Embedded Linux & QEMU" }
                    span { class: "tech-tag border-purple-500/40 text-purple-300 bg-purple-950/40", "Domain-Driven Design (DDD)" }
                    span { class: "tech-tag", "React Native & Expo" }
                    span { class: "tech-tag", "Rust & WebAssembly" }
                    span { class: "tech-tag", "MongoDB & Distributed State" }
                }
            }
        }
    }
}
