use dioxus::prelude::*;

#[component]
pub fn Research() -> Element {
    rsx! {
        section {
            id: "research",
            class: "py-24 px-4 sm:px-6 lg:px-8 bg-slate-900/40 relative border-t border-slate-800/80",
            div { class: "max-w-5xl mx-auto",
                // Section Header
                div { class: "text-center max-w-3xl mx-auto mb-16",
                    div { class: "inline-flex items-center gap-2 px-3 py-1 rounded-full bg-slate-800 text-slate-300 text-xs font-mono mb-3 border border-slate-700",
                        span { class: "w-1.5 h-1.5 rounded-full bg-indigo-400" }
                        "SCIENTIFIC RESEARCH & RIGOR"
                    }
                    h2 { class: "text-3xl sm:text-5xl font-extrabold text-white tracking-tight mb-4",
                        "Mathematical Foundations & Publication"
                    }
                    p { class: "text-base sm:text-lg text-slate-300",
                        "Engineering leadership rooted in first-principles algorithmic problem solving, peer-reviewed methodology, and mathematical precision."
                    }
                }

                // Publication Showcase Card
                div { class: "glass-panel p-8 sm:p-10 rounded-3xl border border-indigo-500/20 relative overflow-hidden group",
                    div { class: "absolute -right-16 -top-16 w-64 h-64 bg-indigo-500/10 rounded-full blur-3xl pointer-events-none group-hover:bg-indigo-500/15 transition duration-500" }

                    div { class: "flex flex-col lg:flex-row lg:items-start justify-between gap-6 mb-6",
                        div { class: "space-y-2",
                            div { class: "flex flex-wrap items-center gap-2",
                                span { class: "text-xs font-mono uppercase tracking-wider px-3 py-1 rounded-full bg-indigo-950/80 text-indigo-300 border border-indigo-500/30",
                                    "Peer-Reviewed Journal Publication"
                                }
                                span { class: "text-xs font-mono text-slate-400", "Year: 2021" }
                            }

                            h3 { class: "text-xl sm:text-2xl font-bold text-white tracking-tight leading-snug group-hover:text-indigo-300 transition duration-200",
                                "A CCD-Based Inverse Kinematic Approach Using Frenet-Serret Parameterisation for SHR Manipulators"
                            }

                            div { class: "text-sm font-semibold text-slate-300",
                                "International Journal of Robotics and Automation"
                            }
                        }

                        div { class: "shrink-0 flex items-center gap-3",
                            div { class: "px-4 py-2 rounded-xl bg-slate-800/80 border border-slate-700/80 text-xs font-mono text-cyan-300 flex items-center gap-2",
                                svg {
                                    class: "w-4 h-4 text-cyan-400",
                                    fill: "none",
                                    stroke: "currentColor",
                                    view_box: "0 0 24 24",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z",
                                    }
                                }
                                span { "Robotics & Kinematics" }
                            }
                        }
                    }

                    // Description and leadership translation
                    div { class: "grid md:grid-cols-2 gap-8 pt-6 border-t border-slate-800/80",
                        div {
                            div { class: "text-xs font-semibold uppercase tracking-wider text-slate-400 mb-2",
                                "Research Focus & Abstract"
                            }
                            p { class: "text-sm text-slate-300 leading-relaxed mb-4",
                                "Investigated inverse kinematics computation for spatial hyper-redundant (SHR) robotic manipulators. Combined Cyclic Coordinate Descent (CCD) algorithms with differential geometry via Frenet-Serret curves to achieve computationally efficient real-time trajectory convergence under spatial curvature constraints."
                            }
                            div { class: "flex flex-wrap gap-2",
                                span { class: "tech-tag", "Cyclic Coordinate Descent (CCD)" }
                                span { class: "tech-tag", "Frenet-Serret Frames" }
                                span { class: "tech-tag", "Hyper-Redundant Manipulators" }
                                span { class: "tech-tag", "Algorithmic Convergence" }
                            }
                        }

                        div {
                            div { class: "text-xs font-semibold uppercase tracking-wider text-indigo-400 mb-2",
                                "Why This Matters for Technical Leadership"
                            }
                            p { class: "text-sm text-slate-300 leading-relaxed",
                                "Navigating uncharted technical domains requires the ability to formulate hypotheses, prove algorithmic feasibility from first principles, and communicate findings with academic clarity. Whether optimizing microservice message loops or emulating kernel hardware interrupts, this foundation enables deep debugging down to the mathematical core."
                            }
                        }
                    }
                }
            }
        }
    }
}
