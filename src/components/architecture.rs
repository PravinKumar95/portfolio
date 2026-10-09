use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum CategoryFilter {
    All,
    Cloud,
    Embedded,
    Frontend,
}

struct CaseStudy {
    category: CategoryFilter,
    badge: &'static str,
    title: &'static str,
    role: &'static str,
    org: &'static str,
    challenge: &'static str,
    architecture_solution: &'static str,
    leadership_impact: &'static str,
    tech: &'static [&'static str],
}

const CASE_STUDIES: &[CaseStudy] = &[
    CaseStudy {
        category: CategoryFilter::Cloud,
        badge: "Flagship Architecture",
        title: "Fleet Orchestration & Vehicle Telemetry Platform",
        role: "Tech Lead & System Architect",
        org: "ZF Group",
        challenge: "Orchestrating real-time commercial vehicle fleets with sporadic cellular connectivity, strict data delivery requirements, and high telemetry concurrency across heterogeneous embedded devices.",
        architecture_solution: "Architected a hybrid event-driven microservices topology using Domain-Driven Design (DDD). Decoupled edge communications using Amazon EventBridge rule filtering, SQS dead-letter buffers, and asynchronous C# .NET Lambdas backed by MongoDB. Implemented offline-first caching on edge client nodes.",
        leadership_impact: "Aligned 3 cross-functional development teams (embedded, mobile, and cloud backend) around contract-first OpenAPI and event schemas, achieving 99.99% message delivery reliability during peak fleet operations.",
        tech: &[
            "C#",
            ".NET 8",
            "AWS Lambda",
            "Amazon EventBridge",
            "SQS",
            "MongoDB",
            "React Native",
            "Domain-Driven Design",
        ],
    },
    CaseStudy {
        category: CategoryFilter::Embedded,
        badge: "DX & Tooling Innovation",
        title: "Hardware-Free Embedded Linux Simulation Ecosystem",
        role: "Lead Systems Engineer & DX Architect",
        org: "Embedded Systems Platform",
        challenge: "Physical automotive hardware modules were scarce and costly, blocking new developer onboarding and preventing automated end-to-end integration tests in CI pipelines.",
        architecture_solution: "Designed and engineered a custom QEMU-based virtualized simulator for macOS and Linux. Emulated the target vehicle hardware architecture, peripheral busses, and kernel interfaces with high fidelity, allowing driver software to execute identically to bench hardware.",
        leadership_impact: "Reduced engineer onboarding time by 60%, eradicated the hardware-bench development bottleneck for 15+ engineers, and enabled headless automated regression test suites in CI.",
        tech: &[
            "QEMU",
            "Embedded Linux",
            "C++",
            "macOS / POSIX",
            "CI/CD Pipelines",
            "Hardware Emulation",
        ],
    },
    CaseStudy {
        category: CategoryFilter::Cloud,
        badge: "Enterprise Security & Scale",
        title: "High-Throughput Fleet Messaging & Role-Based Access Control",
        role: "Backend Architecture Lead",
        org: "ZF Group Cloud Platform",
        challenge: "Executing secure, compliant, fine-grained access control across disparate vehicle sub-fleets with mission-critical dispatch latency constraints.",
        architecture_solution: "Engineered an event-driven pub/sub messaging architecture on AWS EventBridge and SQS queues, combined with policy-based authorization in C# .NET microservices and least-privilege IAM scopes. Ensured auditable security logs for automotive standards.",
        leadership_impact: "Delivered sub-second message dispatch latencies, zero packet drops under traffic spikes, and passed rigorous automotive security and data governance compliance reviews.",
        tech: &[
            "C#",
            "AWS Serverless",
            "EventBridge",
            "SQS",
            "Terraform",
            "Security & RBAC",
        ],
    },
    CaseStudy {
        category: CategoryFilter::Frontend,
        badge: "High-Performance Graphics",
        title: "High-Performance 3D Renderer & Digital Twin Engine",
        role: "Frontend / Graphics Tech Lead",
        org: "Graphics & Visualization Platform",
        challenge: "Rendering large-scale digital twins and hierarchical datasets containing millions of nodes without inducing browser stutter or exceeding client memory limits.",
        architecture_solution: "Built a customized GPU-accelerated WebGL rendering pipeline paired with an algorithmic virtualized tree data structure. Employed spatial frustum culling, batch drawing, and progressive memory chunking to minimize CPU-to-GPU overhead.",
        leadership_impact: "Achieved a 3x (300%) performance increase in rendering throughput, maintaining steady 60 FPS under massive node densities and authoring cross-team rendering best practices.",
        tech: &[
            "WebGL",
            "React",
            "TypeScript",
            "Virtualized Trees",
            "GPU Shaders",
            "Algorithms",
        ],
    },
    CaseStudy {
        category: CategoryFilter::Frontend,
        badge: "Enterprise Design System",
        title: "Enterprise Accessible Component System & Guild",
        role: "UI Guild Lead & Core Architect",
        org: "Telstra",
        challenge: "Multiple independent engineering squads had developed divergent UI patterns, accumulating technical debt and failing stringent WCAG accessibility compliance audits.",
        architecture_solution: "Spearheaded an enterprise design system of accessible, headless TypeScript/React primitives. Established automated axe-core accessibility regression gates in CI and interactive Storybook documentation with full tokenization.",
        leadership_impact: "Adopted by 6+ engineering teams across the enterprise, accelerating feature delivery cycles by 35% and achieving 100% WCAG AA/AAA regulatory compliance scorecards.",
        tech: &[
            "TypeScript",
            "React",
            "Storybook",
            "WCAG AAA/AA",
            "Automated CI Gates",
            "Design Systems",
        ],
    },
    CaseStudy {
        category: CategoryFilter::Cloud,
        badge: "Cloud Optimization",
        title: "Serverless Cloud Media & Storage Pipeline",
        role: "Full-Stack Lead Engineer",
        org: "Cloud Platform",
        challenge: "Client applications experienced slow multi-part uploads while backend servers incurred exorbitant compute and data egress costs handling raw media payloads.",
        architecture_solution: "Architected a Python/FastAPI microservice utilizing asynchronous AWS S3 pre-signed upload URLs, direct client-to-bucket multi-part streaming, automated lifecycle tiering, and responsive React management dashboards.",
        leadership_impact: "Slashed monthly cloud storage and egress expenses by over 50%, while accelerating multi-megabyte upload speeds by 2.4x for end customers.",
        tech: &[
            "Python",
            "FastAPI",
            "AWS S3",
            "Pre-signed URLs",
            "React",
            "Cost Optimization",
        ],
    },
];

#[component]
pub fn Architecture() -> Element {
    let mut active_filter = use_signal(|| CategoryFilter::All);

    rsx! {
        section {
            id: "architecture",
            class: "py-24 px-4 sm:px-6 lg:px-8 relative border-t border-slate-800/80",
            div { class: "max-w-7xl mx-auto",
                // Section Header
                div { class: "text-center max-w-3xl mx-auto mb-12",
                    div { class: "inline-flex items-center gap-2 px-3 py-1 rounded-full bg-slate-800 text-slate-300 text-xs font-mono mb-3 border border-slate-700",
                        span { class: "w-1.5 h-1.5 rounded-full bg-cyan-400" }
                        "SYSTEMS ARCHITECTURE & CASE STUDIES"
                    }
                    h2 { class: "text-3xl sm:text-5xl font-extrabold text-white tracking-tight mb-4",
                        "Engineering Impact & Production Systems"
                    }
                    p { class: "text-base sm:text-lg text-slate-300",
                        "Real-world platforms engineered with resilience, clear domain boundaries, and high business leverage."
                    }
                }

                // Interactive Category Filter Chips
                div { class: "flex flex-wrap items-center justify-center gap-2 mb-14",
                    button {
                        class: if active_filter() == CategoryFilter::All {
                            "px-4 py-2 rounded-xl text-xs sm:text-sm font-semibold bg-indigo-600 text-white shadow-md shadow-indigo-500/25 transition duration-200"
                        } else {
                            "px-4 py-2 rounded-xl text-xs sm:text-sm font-medium bg-slate-800/80 hover:bg-slate-700 text-slate-300 hover:text-white border border-slate-700/60 transition duration-200"
                        },
                        onclick: move |_| active_filter.set(CategoryFilter::All),
                        "All Case Studies (6)"
                    }

                    button {
                        class: if active_filter() == CategoryFilter::Cloud {
                            "px-4 py-2 rounded-xl text-xs sm:text-sm font-semibold bg-indigo-600 text-white shadow-md shadow-indigo-500/25 transition duration-200"
                        } else {
                            "px-4 py-2 rounded-xl text-xs sm:text-sm font-medium bg-slate-800/80 hover:bg-slate-700 text-slate-300 hover:text-white border border-slate-700/60 transition duration-200"
                        },
                        onclick: move |_| active_filter.set(CategoryFilter::Cloud),
                        "Cloud & Event-Driven"
                    }

                    button {
                        class: if active_filter() == CategoryFilter::Embedded {
                            "px-4 py-2 rounded-xl text-xs sm:text-sm font-semibold bg-indigo-600 text-white shadow-md shadow-indigo-500/25 transition duration-200"
                        } else {
                            "px-4 py-2 rounded-xl text-xs sm:text-sm font-medium bg-slate-800/80 hover:bg-slate-700 text-slate-300 hover:text-white border border-slate-700/60 transition duration-200"
                        },
                        onclick: move |_| active_filter.set(CategoryFilter::Embedded),
                        "Embedded & Virtualization"
                    }

                    button {
                        class: if active_filter() == CategoryFilter::Frontend {
                            "px-4 py-2 rounded-xl text-xs sm:text-sm font-semibold bg-indigo-600 text-white shadow-md shadow-indigo-500/25 transition duration-200"
                        } else {
                            "px-4 py-2 rounded-xl text-xs sm:text-sm font-medium bg-slate-800/80 hover:bg-slate-700 text-slate-300 hover:text-white border border-slate-700/60 transition duration-200"
                        },
                        onclick: move |_| active_filter.set(CategoryFilter::Frontend),
                        "High-Performance Frontend & DX"
                    }
                }

                // Case Studies Cards Grid
                div { class: "grid md:grid-cols-2 lg:grid-cols-3 gap-7",
                    for cs in CASE_STUDIES.iter().filter(|c| active_filter() == CategoryFilter::All || c.category == active_filter()) {
                        div { class: "glass-card rounded-2xl overflow-hidden border border-slate-800 p-6 flex flex-col justify-between group hover:border-indigo-500/50",
                            div {
                                // Header Badges
                                div { class: "flex items-center justify-between gap-2 mb-4",
                                    span { class: "text-[11px] font-mono uppercase tracking-wider px-2.5 py-1 rounded-md bg-indigo-950/70 text-indigo-300 border border-indigo-500/30",
                                        "{cs.badge}"
                                    }
                                    span { class: "text-xs font-semibold text-slate-400",
                                        "{cs.org}"
                                    }
                                }

                                // Title & Role
                                h3 { class: "text-xl font-bold text-white mb-1 group-hover:text-indigo-300 transition duration-200 leading-snug",
                                    "{cs.title}"
                                }
                                div { class: "text-xs font-mono text-cyan-400 mb-4",
                                    "Role: {cs.role}"
                                }

                                // The Challenge
                                div { class: "mb-3.5",
                                    div { class: "text-[11px] font-semibold uppercase tracking-wider text-slate-400 mb-1",
                                        "Architectural Challenge"
                                    }
                                    p { class: "text-xs sm:text-sm text-slate-300 leading-relaxed",
                                        "{cs.challenge}"
                                    }
                                }

                                // The Architectural Solution
                                div { class: "mb-3.5",
                                    div { class: "text-[11px] font-semibold uppercase tracking-wider text-indigo-400 mb-1",
                                        "System Design & Decisions"
                                    }
                                    p { class: "text-xs sm:text-sm text-slate-300 leading-relaxed",
                                        "{cs.architecture_solution}"
                                    }
                                }

                                // Leadership Impact
                                div { class: "p-3.5 rounded-xl bg-slate-900/80 border border-slate-800 mb-5",
                                    div { class: "text-[11px] font-semibold uppercase tracking-wider text-emerald-400 mb-1 flex items-center gap-1.5",
                                        svg {
                                            class: "w-3.5 h-3.5",
                                            fill: "none",
                                            stroke: "currentColor",
                                            view_box: "0 0 24 24",
                                            path {
                                                stroke_linecap: "round",
                                                stroke_linejoin: "round",
                                                stroke_width: "2.5",
                                                d: "M13 7h8m0 0v8m0-8l-8 8-4-4-6 6",
                                            }
                                        }
                                        "Leadership & Business Impact"
                                    }
                                    p { class: "text-xs text-slate-200 font-medium leading-relaxed",
                                        "{cs.leadership_impact}"
                                    }
                                }
                            }

                            // Tech Chips footer
                            div { class: "pt-4 border-t border-slate-800/80 mt-auto",
                                div { class: "flex flex-wrap gap-1.5",
                                    for t in cs.tech.iter() {
                                        span { class: "text-[11px] font-mono px-2 py-0.5 rounded bg-slate-800 text-slate-300 border border-slate-700/60",
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
