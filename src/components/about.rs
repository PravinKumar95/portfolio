use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    let skills = vec![
        "C#", ".NET", "Node.js", "TypeScript", "React Native", "Expo", 
        "AWS (Lambda, SQS, EventBridge)", "Terraform", "MongoDB", 
        "Embedded Linux", "QEMU", "Domain-Driven Design (DDD)"
    ];
    
    view! {
        <section id="about" class="py-20 px-4 bg-gray-800">
            <div class="max-w-4xl mx-auto">
                <h2 class="text-3xl md:text-4xl font-bold mb-8 text-center border-b-2 border-purple-500 pb-2 w-fit mx-auto">
                    "About Me"
                </h2>
                <div class="grid md:grid-cols-2 gap-12">
                    <div>
                        <p class="text-gray-300 leading-relaxed mb-4">
                            "I'm a senior software engineer with 7+ years of experience building production systems. Currently at ZF Group, I lead the architecture of a fleet orchestration platform spanning embedded Linux apps and C# microservices on AWS."
                        </p>
                        <p class="text-gray-300 leading-relaxed mb-4">
                            "My expertise lies in creating seamless integrations between hardware and cloud, whether it's building QEMU-based simulators for embedded development or implementing robust event-driven microservices."
                        </p>
                        <p class="text-gray-300 leading-relaxed italic text-sm">
                            "I actively leverage agentic workflows and AI-driven tooling to accelerate delivery without compromising on quality."
                        </p>
                    </div>
                    <div>
                        <h3 class="text-xl font-semibold mb-4 text-purple-400">"Core Stack & Expertise"</h3>
                        <div class="flex flex-wrap gap-2">
                            {skills.into_iter().map(|skill| view! {
                                <span class="bg-gray-700 text-purple-300 px-3 py-1 rounded-md text-sm border border-gray-600">
                                    {skill}
                                </span>
                            }).collect::<Vec<_>>()}
                        </div>
                        
                        <div class="mt-8">
                            <h3 class="text-xl font-semibold mb-2 text-purple-400">"Publication"</h3>
                            <p class="text-sm text-gray-400">
                                "A CCD-BASED INVERSE KINEMATIC APPROACH USING FRENET-SERRET PARAMETERISATION FOR SHR MANIPULATORS"
                            </p>
                            <p class="text-xs text-gray-500 mt-1">"International Journal of Robotics and Automation · 2021"</p>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
