use dioxus::prelude::*;

struct SkillGroup {
    title: &'static str,
    tagline: &'static str,
    accent_color: &'static str,
    skills: &'static [(&'static str, &'static str)],
}

const GROUPS: &[SkillGroup] = &[
    SkillGroup {
        title: "Distributed Systems & Cloud Architecture",
        tagline: "Event-driven scale, microservices & cloud infrastructure",
        accent_color: "text-indigo-400 border-indigo-500/30 bg-indigo-950/40",
        skills: &[
            (
                "C# / .NET 8",
                "Primary language for high-performance enterprise microservices",
            ),
            (
                "AWS Lambda & Serverless",
                "Event-driven computing with zero idle overhead",
            ),
            (
                "Amazon EventBridge",
                "Central event bus decoupling fleet domain components",
            ),
            (
                "Amazon SQS / SNS",
                "Durable async queueing and dead-letter fault isolation",
            ),
            (
                "Domain-Driven Design (DDD)",
                "Strategic bounded contexts and aggregate invariants",
            ),
            (
                "MongoDB & Distributed DBs",
                "High-throughput document telemetry and indexing",
            ),
            (
                "Terraform & IaC",
                "Declarative, repeatable infrastructure provisioning",
            ),
            (
                "REST & Asynchronous Messaging",
                "OpenAPI contracts and schema validation",
            ),
        ],
    },
    SkillGroup {
        title: "Embedded Systems & Hardware Virtualization",
        tagline: "Bridging physical devices, kernels & host development",
        accent_color: "text-emerald-400 border-emerald-500/30 bg-emerald-950/40",
        skills: &[
            (
                "Embedded Linux",
                "Target OS for automotive in-cabin and fleet compute units",
            ),
            (
                "QEMU Virtualization",
                "Virtual hardware environments mirroring physical target boards",
            ),
            (
                "C / C++",
                "Systems programming, device interfacing, and simulation mocks",
            ),
            (
                "POSIX & Linux Internals",
                "Process isolation, daemon architecture, IPC mechanisms",
            ),
            (
                "Hardware-in-the-Loop (HIL) DX",
                "Decoupling software test suites from scarce bench rigs",
            ),
            (
                "Cross-Compilation Pipelines",
                "Automated toolchains for ARM and x86 target architectures",
            ),
        ],
    },
    SkillGroup {
        title: "Client Architecture & Modern Frontend",
        tagline: "Multi-platform applications, reactive UI & graphics",
        accent_color: "text-cyan-400 border-cyan-500/30 bg-cyan-950/40",
        skills: &[
            (
                "React Native & Expo",
                "In-vehicle fleet operator interfaces and companion apps",
            ),
            (
                "TypeScript / JavaScript",
                "End-to-end typed web applications and tooling",
            ),
            (
                "React & Component Systems",
                "Enterprise design system architecture and tokenization",
            ),
            (
                "Rust & WebAssembly (Dioxus)",
                "Zero-JS runtime, blazingly fast deterministic web tech",
            ),
            (
                "WebGL & High-Perf Graphics",
                "GPU-accelerated rendering and large virtualized datasets",
            ),
            (
                "WCAG 2.1 AA/AAA Accessibility",
                "Standardized accessibility guidelines and automated CI testing",
            ),
        ],
    },
    SkillGroup {
        title: "Technical Leadership & Delivery Culture",
        tagline: "Multiplying team output, governance & modern velocity",
        accent_color: "text-purple-400 border-purple-500/30 bg-purple-950/40",
        skills: &[
            (
                "Architecture Decision Records (ADRs)",
                "Transparent, documented architectural consensus",
            ),
            (
                "Engineering Mentorship",
                "Coaching junior to senior engineers through pairing & reviews",
            ),
            (
                "Guild & Cross-Squad Leadership",
                "Standardizing UI, accessibility, and backend practices",
            ),
            (
                "CI/CD Pipeline Automation",
                "GitHub Actions, automated test suites, linting, packaging",
            ),
            (
                "Agentic AI Workflows",
                "Amplifying engineering velocity with AI pair programming",
            ),
            (
                "Agile & Technical Strategy",
                "Translating executive product goals into technical execution",
            ),
        ],
    },
];

#[component]
pub fn Skills() -> Element {
    rsx! {
        section {
            id: "competencies",
            class: "py-24 px-4 sm:px-6 lg:px-8 bg-slate-900/30 relative border-t border-slate-800/80",
            div { class: "max-w-7xl mx-auto",
                // Section Header
                div { class: "text-center max-w-3xl mx-auto mb-16",
                    div { class: "inline-flex items-center gap-2 px-3 py-1 rounded-full bg-slate-800 text-slate-300 text-xs font-mono mb-3 border border-slate-700",
                        span { class: "w-1.5 h-1.5 rounded-full bg-purple-400" }
                        "TECHNICAL MATRIX & EXPERTISE"
                    }
                    h2 { class: "text-3xl sm:text-5xl font-extrabold text-white tracking-tight mb-4",
                        "Full-Spectrum Systems Competence"
                    }
                    p { class: "text-base sm:text-lg text-slate-300",
                        "A cohesive technical breadth from low-level Linux and hypervisors to cloud-native event brokers and team leadership."
                    }
                }

                // 4 Quadrants Grid
                div { class: "grid md:grid-cols-2 gap-8",
                    for group in GROUPS.iter() {
                        div { class: "glass-card p-7 rounded-2xl border border-slate-800 flex flex-col justify-between",
                            div {
                                div { class: "flex items-center justify-between gap-2 mb-2",
                                    h3 { class: "text-xl font-bold text-white tracking-tight",
                                        "{group.title}"
                                    }
                                    span { class: "text-[11px] font-mono px-2.5 py-0.5 rounded-full border shrink-0 {group.accent_color}",
                                        "Domain"
                                    }
                                }
                                p { class: "text-xs sm:text-sm text-slate-400 font-mono mb-6",
                                    "{group.tagline}"
                                }

                                div { class: "space-y-3",
                                    for &(name, detail) in group.skills.iter() {
                                        div { class: "p-3 rounded-xl bg-slate-900/70 border border-slate-800/80 flex flex-col sm:flex-row sm:items-center justify-between gap-1 group hover:border-slate-700 transition",
                                            span { class: "text-sm font-semibold text-slate-200 group-hover:text-white transition",
                                                "{name}"
                                            }
                                            span { class: "text-xs text-slate-400 font-normal",
                                                "{detail}"
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
}
