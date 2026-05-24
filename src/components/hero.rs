use leptos::prelude::*;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <section class="flex flex-col items-center justify-center min-h-[70vh] text-center px-4 bg-gradient-to-b from-gray-900 to-gray-800">
            <h1 class="text-5xl md:text-7xl font-extrabold mb-4 bg-clip-text text-transparent bg-gradient-to-r from-purple-400 to-pink-600">
                "Pravin Kumar"
            </h1>
            <h2 class="text-2xl md:text-3xl font-bold mb-4 text-purple-300">
                "Senior Software Engineer"
            </h2>
            <p class="text-xl md:text-2xl text-gray-300 max-w-3xl mb-8">
                "Building production systems at the intersection of embedded software, cloud infrastructure, and connected vehicle platforms."
            </p>
            <div class="flex gap-4">
                <a href="#projects" class="bg-purple-600 hover:bg-purple-700 text-white font-bold py-3 px-8 rounded-full transition duration-300 transform hover:scale-105">
                    "View My Work"
                </a>
                <a href="#contact" class="border border-purple-500 text-purple-400 hover:bg-purple-500 hover:text-white font-bold py-3 px-8 rounded-full transition duration-300">
                    "Get in Touch"
                </a>
            </div>
        </section>
    }
}
