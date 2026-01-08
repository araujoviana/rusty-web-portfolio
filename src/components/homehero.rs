use crate::components::Cloud;
use crate::utils::glass::GLASS_HERO;
use leptos::prelude::*;

const TITLE: &str = "Hi! I'm Matheus";
const SUBTITLE: &str =
    "I build reliable systems, cloud infrastructure, and low-level tools for real-world use.";

#[component]
pub fn HomeHero() -> impl IntoView {
    view! {
        <div class=format!("{GLASS_HERO} max-w-6xl mx-auto px-4 sm:px-6")>
            <section class="
            relative
            mx-auto
            max-w-5xl
            px-8
            py-28
            ">
                <div class="
                grid
                grid-cols-1
                md:grid-cols-[1.1fr_0.9fr]
                gap-16
                items-center
                ">
                    <div class="space-y-8">
                        <h1 class="
                        text-5xl
                        font-semibold
                        tracking-tight
                        leading-tight
                        ">{TITLE}</h1>

                        <p class="
                        max-w-md
                        text-lg
                        text-white/80
                        leading-relaxed
                        ">{SUBTITLE}</p>

                        <div class="flex gap-4 pt-2">
                            <a
                                href="#projects"
                                class="
                                inline-flex items-center justify-center
                                px-5 py-2.5
                                rounded-xl
                                text-white/80
                                font-medium
                                border border-white/15
                                bg-white/5
                                backdrop-blur
                                transition duration-300
                                hover:bg-white/10
                                hover:border-white/25
                                hover:text-white
                                "
                            >
                                "Projects"
                            </a>

                            <a
                                href="#contact"
                                class="
                                inline-flex items-center justify-center
                                px-5 py-2.5
                                rounded-xl
                                text-white/80
                                font-medium
                                border border-white/15
                                bg-white/5
                                backdrop-blur
                                transition duration-300
                                hover:bg-white/10
                                hover:border-white/25
                                hover:text-white
                                "
                            >
                                "Contact"
                            </a>
                        </div>

                    </div>

                    <div class="
                    relative
                    flex
                    justify-center
                    md:justify-end
                    ">
                        <Cloud />
                    </div>
                </div>
            </section>
        </div>
    }
}
