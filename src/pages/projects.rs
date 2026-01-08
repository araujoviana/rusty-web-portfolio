use leptos::prelude::*;

use crate::utils::projects_data::{PROJECTS, Project};

fn svg_data_url(svg: &str) -> String {
    let mut out = String::with_capacity(svg.len() + 32);
    out.push_str("data:image/svg+xml;utf8,");
    for c in svg.chars() {
        match c {
            '#' => out.push_str("%23"),
            '&' => out.push_str("%26"),
            '<' => out.push_str("%3C"),
            '>' => out.push_str("%3E"),
            '"' => out.push_str("%22"),
            '\'' => out.push_str("%27"),
            ' ' => out.push_str("%20"),
            '\n' | '\r' | '\t' => {}
            _ => out.push(c),
        }
    }
    out
}

fn is_video(src: &str) -> bool {
    src.ends_with(".webm") || src.ends_with(".mp4")
}

#[component]
pub fn Projects() -> impl IntoView {
    view! {
            <section id="projects" class=" [content-visibility:auto] [contain-intrinsic-size:1px_900px] px-4 sm:px-6 lg:px-8 pt-16 sm:pt-20">
                <div class="mx-auto max-w-6xl">
                    <div class="mb-10 flex items-end justify-between gap-6">
                        <div class="space-y-2">
                            <h2 class="text-3xl sm:text-4xl font-semibold tracking-tight text-white/90">
                                "Projects"
                            </h2>
                            <p class="text-white/60 max-w-prose leading-relaxed">
    "Selected projects exploring systems, tooling, and performance-driven engineering."
                            </p>
                        </div>
                        <div class="hidden sm:block text-xs font-mono text-white/40">
                            "$ ls projects/"
                        </div>
                    </div>

                    <div class="space-y-6">
                        <For
                            // IMPORTANT: iterate Projects by value (Copy) so key gets &Project (NOT &&Project)
                            each=move || PROJECTS.iter().copied()
                            key=|p: &Project| p.id
                            children=move |p: Project| {
                                let has_media = !p.media_src.is_empty();
                                let has_lang = !p.language_label.is_empty();
                                let has_demo = !p.demo_url.is_empty();
                                let has_icon = p.language_svg.is_some();

                                view! {
                                    <article class="
                                relative overflow-hidden rounded-3xl
                                border border-white/15
                                bg-gradient-to-br from-white/14 via-white/8 to-white/4
                                backdrop-blur-xl
                                shadow-[0_20px_55px_-25px_rgba(0,0,0,0.55)]
                                px-6 py-6
                                ">
                                        <div class="relative z-10 grid grid-cols-1 md:grid-cols-5 gap-6 items-center">

                                            <div class="md:col-span-2">
                                                <div class="
                                            relative overflow-hidden rounded-2xl
                                            border border-white/10
                                            bg-black/40
                                            shadow-[inset_0_1px_0_rgba(255,255,255,0.08)]
                                            ">
                                                    <Show
                                                        when=move || has_media
                                                        fallback=move || {
                                                            view! {
                                                                <div class="h-full w-full flex items-center justify-center px-6 text-sm text-white/40">
                                                                    <span class="font-mono">{p.media_label}</span>
                                                                </div>
                                                            }
                                                        }
                                                    >
                                                        <div class="
                                                    relative overflow-hidden rounded-2xl
                                                    border border-white/10
                                                    bg-black/40
                                                    shadow-[inset_0_1px_0_rgba(255,255,255,0.08)]
                                                    ">
                                                            <Show
                                                                when=move || is_video(p.media_src)
                                                                fallback=move || {
                                                                    view! {
                                                                        <img
                                                                            class="w-full h-auto object-contain rounded-2xl"
                                                                            src=p.media_src
                                                                            alt=p.name
                                                                            loading="lazy"
                                                                        />
                                                                    }
                                                                }
                                                            >
                                                                <video
                                                                    class="w-full h-auto object-contain rounded-2xl"
                                                                    src=p.media_src
                                                                    autoplay
                                                                    loop
                                                                    muted
                                                                    playsinline
                                                                    preload="metadata"
                                                                />
                                                            </Show>
                                                        </div>
                                                    </Show>

                                                </div>
                                            </div>

                                            <div class="md:col-span-3 flex flex-col">
                                                <div class="flex items-start justify-between gap-4">
                                                    <div class="space-y-1">
                                                        <h3 class="text-xl sm:text-2xl font-semibold text-white/90 tracking-tight">
                                                            {p.name}
                                                        </h3>

                                                        <Show when=move || has_lang fallback=|| ()>
                                                            <span class="
                                                        inline-flex items-center gap-2
                                                        rounded-full px-3 py-1 text-xs
                                                        border border-white/10
                                                        bg-white/5
                                                        text-white/70
                                                        backdrop-blur
                                                        ">
                                                                <Show when=move || has_icon fallback=|| ()>
                                                                    <img
                                                                        class="w-4 h-4 shrink-0"
                                                                        src=svg_data_url(p.language_svg.unwrap())
                                                                        alt=""
                                                                        aria-hidden="true"
                                                                    />
                                                                </Show>
                                                                <span>{p.language_label}</span>
                                                            </span>
                                                        </Show>
                                                    </div>

                                                    <span class="text-xs font-mono text-white/35">
                                                        {format!("#{}", p.id)}
                                                    </span>
                                                </div>

                                                <p class="mt-4 text-white/65 leading-relaxed">
                                                    {p.description}
                                                </p>

                                                <div class="mt-5 flex flex-wrap items-center gap-3">
                                                    <a
                                                        href=p.repo_url
                                                        target="_blank"
                                                        rel="noopener noreferrer"
                                                        class="
                                                    inline-flex items-center justify-center gap-2
                                                    rounded-xl px-4 py-2 text-sm font-medium
                                                    bg-white/10 text-white/85
                                                    border border-white/15
                                                    backdrop-blur
                                                    transition duration-300
                                                    hover:bg-white/15 hover:border-white/25 hover:text-white
                                                    "
                                                    >
                                                        {p.cta_label}
                                                    </a>

                                                    <Show when=move || has_demo fallback=|| ()>
                                                        <a
                                                            href=p.demo_url
                                                            target="_blank"
                                                            rel="noopener noreferrer"
                                                            class="
                                                        inline-flex items-center justify-center gap-2
                                                        rounded-xl px-4 py-2 text-sm font-medium
                                                        text-white/70
                                                        border border-white/10
                                                        bg-white/5
                                                        backdrop-blur
                                                        transition duration-300
                                                        hover:bg-white/10 hover:border-white/20 hover:text-white
                                                        "
                                                        >
                                                            {p.demo_label}
                                                        </a>
                                                    </Show>
                                                </div>
                                            </div>
                                        </div>
                                    </article>
                                }
                            }
                        />
                    </div>
                </div>
            </section>
        }
}
