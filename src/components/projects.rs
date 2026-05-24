use leptos::prelude::*;

struct Project {
    title: &'static str,
    description: &'static str,
    tech: Vec<&'static str>,
}

#[component]
pub fn Projects() -> impl IntoView {
    let projects = vec![
        Project {
            title: "Fleet Orchestration Platform",
            description: "Leading architecture for a multi-platform system involving React Native embedded apps and C# microservices on AWS.",
            tech: vec!["C#", "AWS", "React Native", "MongoDB"],
        },
        Project {
            title: "Embedded Linux Simulator",
            description: "Built a QEMU-based macOS simulator for embedded driver apps, enabling hardware-free development and testing.",
            tech: vec!["QEMU", "Embedded Linux", "C++", "macOS"],
        },
        Project {
            title: "Messaging Microservice",
            description: "Implemented a complex access control microservice for fleet communication using EventBridge and SQS.",
            tech: vec!["C#", "AWS Lambda", "EventBridge", "SQS"],
        },
        Project {
            title: "Accessible UI Library",
            description: "Developed a suite of WCAG-compliant React components used across multiple enterprise teams at Telstra.",
            tech: vec!["React", "TypeScript", "Accessibility", "Storybook"],
        },
        Project {
            title: "High-Performance 3D Renderer",
            description: "Optimized WebGL renderer performance 3x and built a virtualized tree component for millions of nodes.",
            tech: vec!["WebGL", "React", "TypeScript", "Algorithms"],
        },
        Project {
            title: "Cloud File Storage App",
            description: "Built a Python/FastAPI backend with S3 integration, reducing client storage costs by over 50%.",
            tech: vec!["Python", "FastAPI", "AWS S3", "React"],
        },
    ];

    view! {
        <section id="projects" class="py-20 px-4 bg-gray-900">
            <div class="max-w-6xl mx-auto">
                <h2 class="text-3xl md:text-4xl font-bold mb-12 text-center text-white border-b-2 border-purple-500 pb-2 w-fit mx-auto">
                    "Professional Experience & Projects"
                </h2>
                <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
                    {projects.into_iter().map(|p| view! {
                        <div class="bg-gray-800 rounded-xl overflow-hidden shadow-lg border border-gray-700 hover:border-purple-500 transition duration-300 flex flex-col">
                            <div class="p-6 flex-grow">
                                <h3 class="text-xl font-bold mb-2 text-white">{p.title}</h3>
                                <p class="text-gray-400 mb-4 text-sm leading-relaxed">{p.description}</p>
                            </div>
                            <div class="px-6 pb-6 mt-auto">
                                <div class="flex flex-wrap gap-2">
                                    {p.tech.into_iter().map(|t| view! {
                                        <span class="text-xs bg-purple-900/30 text-purple-300 px-2 py-1 rounded">
                                            {t}
                                        </span>
                                    }).collect::<Vec<_>>()}
                                </div>
                            </div>
                        </div>
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </section>
    }
}
