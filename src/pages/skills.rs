use crate::utils::glass::{GLASS_SKILL_ENTRY, GLASS_SKILLS};
use crate::utils::skills_data::{GROUPS, SkillItem};
use leptos::prelude::*;

#[component]
pub fn Skills() -> impl IntoView {
    view! {
        <section id="skills" class="px-4 sm:px-6 lg:px-8 pt-16 sm:pt-20">
            <div class="mx-auto max-w-6xl">
                <div class="mb-10 flex items-end justify-between gap-6">
                    <div class="space-y-2">
                        <h2 class="text-3xl sm:text-4xl font-semibold tracking-tight text-white/90">
                            "Skills"
                        </h2>
                        <p class="text-white/60 max-w-prose leading-relaxed">
                            "Tools, frameworks, and systems knowledge I’m comfortable owning end-to-end."
                        </p>
                    </div>
                    <div class="hidden sm:block text-xs font-mono text-white/40">
                        "$ skills --summary"
                    </div>
                </div>

                <div class="grid gap-6 lg:grid-cols-3">
                    <For
                        // Return the slice directly => items are &SkillGroup
                        each=move || GROUPS
                        key=|g| g.title
                        children=move |g| {
                            view! { <SkillCard title=g.title subtitle=g.subtitle items=g.items /> }
                        }
                    />
                </div>
            </div>
        </section>
    }
}

#[component]
fn SkillCard(
    title: &'static str,
    subtitle: &'static str,
    items: &'static [SkillItem],
) -> impl IntoView {
    view! {
        <div class=GLASS_SKILLS>
            <div class="relative z-10">
                <div class="mb-5">
                    <h3 class="text-lg font-semibold text-white/90">{title}</h3>
                    <p class="mt-1 text-sm text-white/55">{subtitle}</p>
                </div>

                <div class="flex flex-wrap gap-2">
                    <For
                        each=move || items
                        key=|&(name, _)| name
                        children=move |&(name, level)| {
                            view! {
                                <span class=GLASS_SKILL_ENTRY>
                                    <span class="font-medium">{name}</span>
                                    <span class="text-white/40">"•"</span>
                                    <span class="font-mono text-[11px] text-white/50">{level}</span>
                                </span>
                            }
                        }
                    />
                </div>
            </div>
        </div>
    }
}
