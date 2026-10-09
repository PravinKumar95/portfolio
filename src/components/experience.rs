use dioxus::prelude::*;

struct ExperienceItem {
    role: &'static str,
    company: &'static str,
    period: &'static str,
    location_or_type: &'static str,
    badge: &'static str,
    badge_color: &'static str,
    summary: &'static str,
    deliverables: &'static [&'static str],
    tech: &'static [&'static str],
}

const EXPERIENCES: &[ExperienceItem] = &[
    ExperienceItem {
        role: "Tech Lead & Senior Software Engineer",
        company: "ZF Group",
        period: "Present",
        location_or_type: "Automotive & Mobility Systems",
        badge: "Current Leadership Role",
        badge_color: "bg-indigo-950/80 text-indigo-300 border-indigo-500/40",
        summary: "Leading the systems architecture and cross-team delivery for a mission-critical commercial fleet orchestration platform connecting in-vehicle edge devices with scalable AWS cloud services.",
        deliverables: &[
            "Architected event-driven C# .NET microservices topology on AWS (Lambda, EventBridge, SQS) using Domain-Driven Design (DDD).",
            "Conceived and built a custom QEMU virtualized simulator for macOS/Linux, eliminating physical ECU hardware dependencies and accelerating onboarding by 60%.",
            "Established strict OpenAPI contracts and event schemas, aligning 3 squads (embedded Linux, React Native, and AWS cloud backend).",
            "Implemented resilient offline caching and synchronization protocols for vehicle nodes operating in intermittent connectivity environments.",
            "Mentored mid and senior engineers on event-driven architecture, concurrency patterns, and automated CI pipelines.",
        ],
        tech: &[
            "C#",
            ".NET 8",
            "AWS Lambda",
            "Amazon EventBridge",
            "SQS",
            "Embedded Linux",
            "QEMU",
            "MongoDB",
            "React Native",
            "DDD",
        ],
    },
    ExperienceItem {
        role: "Senior Software Engineer & UI Guild Lead",
        company: "Telstra",
        period: "Previous Enterprise Engagement",
        location_or_type: "Telecommunications & Enterprise Platforms",
        badge: "Guild Leadership & Standards",
        badge_color: "bg-purple-950/80 text-purple-300 border-purple-500/40",
        summary: "Served as UI Guild Lead and core contributor, spearheading enterprise-wide design system development and rigorous WCAG accessibility standards across mission-critical customer applications.",
        deliverables: &[
            "Architected and published an accessible, tokenized React component library adopted by 6+ disparate product teams.",
            "Integrated automated axe-core accessibility regression gates into CI/CD workflows, eliminating compliance audit penalties.",
            "Led bi-weekly guild knowledge shares, architectural reviews, and technical onboarding for front-end engineering squads.",
            "Refactored legacy customer portal flows, significantly reducing bundle size and improving Core Web Vitals.",
        ],
        tech: &[
            "TypeScript",
            "React",
            "Storybook",
            "WCAG 2.1 AAA/AA",
            "Automated Testing",
            "CI/CD Gates",
        ],
    },
    ExperienceItem {
        role: "Full-Stack Systems Engineer",
        company: "High-Performance Software & Graphics",
        period: "Specialized Engineering",
        location_or_type: "Graphics, Cloud & Distributed Storage",
        badge: "Performance & Systems Optimization",
        badge_color: "bg-cyan-950/80 text-cyan-300 border-cyan-500/40",
        summary: "Engineered high-performance graphics visualizers and optimized cloud storage architectures handling intensive computational workloads.",
        deliverables: &[
            "Optimized WebGL rendering pipelines by 3x and implemented virtualized tree algorithms handling millions of dynamic nodes at 60 FPS.",
            "Engineered Python/FastAPI microservices leveraging asynchronous S3 pre-signed upload channels, cutting client cloud storage and egress fees by >50%.",
            "Pioneered early internal adoption of agentic AI workflows and Rust/WebAssembly tooling for deterministic performance benchmarks.",
        ],
        tech: &[
            "WebGL",
            "React",
            "Python",
            "FastAPI",
            "AWS S3",
            "Rust",
            "WebAssembly",
            "Algorithms",
        ],
    },
];

#[component]
pub fn Experience() -> Element {
    rsx! {
        section {
            id: "experience",
            class: "py-24 px-4 sm:px-6 lg:px-8 relative border-t border-slate-800/80",
            div { class: "max-w-5xl mx-auto",
                // Section Header
                div { class: "text-center max-w-3xl mx-auto mb-16",
                    div { class: "inline-flex items-center gap-2 px-3 py-1 rounded-full bg-slate-800 text-slate-300 text-xs font-mono mb-3 border border-slate-700",
                        span { class: "w-1.5 h-1.5 rounded-full bg-emerald-400" }
                        "CAREER JOURNEY & IMPACT"
                    }
                    h2 { class: "text-3xl sm:text-5xl font-extrabold text-white tracking-tight mb-4",
                        "Professional Leadership Timeline"
                    }
                    p { class: "text-base sm:text-lg text-slate-300",
                        "Demonstrated trajectory of architectural ownership, technical stewardship, and measurable engineering outcomes."
                    }
                }

                // Timeline Container
                div { class: "space-y-10 relative before:absolute before:inset-0 before:left-4 sm:before:left-8 before:w-0.5 before:bg-gradient-to-b before:from-indigo-500 before:via-purple-500 before:to-slate-800",
                    for item in EXPERIENCES.iter() {
                        div { class: "relative pl-12 sm:pl-20 group",
                            // Timeline Dot / Node
                            div { class: "absolute left-2 sm:left-6 -translate-x-1/2 top-1.5 w-5 h-5 rounded-full bg-slate-950 border-2 border-indigo-400 group-hover:border-cyan-300 transition duration-300 flex items-center justify-center shadow-md shadow-indigo-500/30",
                                div { class: "w-1.5 h-1.5 rounded-full bg-indigo-400 group-hover:bg-cyan-300 transition" }
                            }

                            // Content Card
                            div { class: "glass-card p-7 sm:p-8 rounded-2xl border border-slate-800",
                                div { class: "flex flex-col sm:flex-row sm:items-center justify-between gap-2 mb-3",
                                    div {
                                        h3 { class: "text-xl sm:text-2xl font-bold text-white group-hover:text-indigo-300 transition duration-200",
                                            "{item.role}"
                                        }
                                        div { class: "text-base font-semibold text-slate-300",
                                            "{item.company}"
                                            span { class: "text-slate-500 text-sm font-normal",
                                                " · {item.location_or_type}"
                                            }
                                        }
                                    }
                                    div { class: "flex flex-wrap items-center gap-2",
                                        span { class: "text-xs font-mono px-3 py-1 rounded-full border {item.badge_color}",
                                            "{item.badge}"
                                        }
                                        span { class: "text-xs font-mono text-slate-400 bg-slate-800/80 px-2.5 py-1 rounded-md border border-slate-700/60",
                                            "{item.period}"
                                        }
                                    }
                                }

                                p { class: "text-sm text-slate-300 leading-relaxed mb-6 font-medium",
                                    "{item.summary}"
                                }

                                // Key Leadership Deliverables
                                div { class: "mb-6",
                                    div { class: "text-xs font-semibold uppercase tracking-wider text-slate-400 mb-3",
                                        "Key Deliverables & Architectural Ownership:"
                                    }
                                    ul { class: "space-y-2",
                                        for point in item.deliverables.iter() {
                                            li { class: "flex items-start gap-3 text-xs sm:text-sm text-slate-300",
                                                span { class: "text-indigo-400 mt-1 shrink-0 font-bold", "▹" }
                                                span { "{point}" }
                                            }
                                        }
                                    }
                                }

                                // Tech Stack Chips
                                div { class: "flex flex-wrap gap-1.5 pt-4 border-t border-slate-800/80",
                                    for t in item.tech.iter() {
                                        span { class: "text-xs font-mono px-2.5 py-0.5 rounded-md bg-slate-800/90 text-slate-300 border border-slate-700/60",
                                            "{t}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
