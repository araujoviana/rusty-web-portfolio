use leptos::html;
use leptos::prelude::*;

use std::time::Duration;

use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

use crate::utils::glass::GLASS_TERMINAL;

const ABOUT_P1: &str = "I’m Matheus, a cloud and systems-focused intern who likes software that feels fast, reliable, and straight to the point. Most of my work lives around cloud infrastructure, Linux, automation, and the tooling that keeps systems running.";

const ABOUT_P2: &str = "I learn by building. I’m good at debugging, picking up new tools quickly, and reading docs/specs when others won’t. I also lead when a team is stuck, and I’m comfortable talking with clients and teammates in both English and Portuguese.";

const TERMINAL_SCRIPT: &[&str] = &[
    "$ whoami",
    "Matheus",
    "",
    "$ role",
    "Solutions Architect Intern (Huawei Cloud)",
    "",
    "$ focus",
    "performance / reliability / simple systems",
    "",
    "$ strengths",
    "debugging / reading docs / guiding teams",
    "",
    "$ stack",
    "Linux / Kubernetes / cloud infrastructure / Python / Rust",
    "",
    "$ languages",
    "Portuguese (native) / English (advanced)",
];

#[component]
pub fn About() -> impl IntoView {
    let terminal_ref = NodeRef::<html::Div>::new();
    let is_visible = RwSignal::new(false);

    Effect::new(move |_| {
        if is_visible.get() {
            return;
        }

        let Some(el) = terminal_ref.get() else { return };

        let cb =
            Closure::<dyn FnMut(js_sys::Array, web_sys::IntersectionObserver)>::wrap(Box::new(
                move |entries: js_sys::Array, _observer: web_sys::IntersectionObserver| {
                    // iterate entries; if any is intersecting -> trigger
                    for i in 0..entries.length() {
                        let entry = entries
                            .get(i)
                            .unchecked_into::<web_sys::IntersectionObserverEntry>();
                        if entry.is_intersecting() {
                            is_visible.set(true);
                            break;
                        }
                    }
                },
            ));

        let observer = web_sys::IntersectionObserver::new(cb.as_ref().unchecked_ref())
            .expect("IntersectionObserver should be available");

        observer.observe(&el);

        // leak both so they stay alive
        cb.forget();
        std::mem::forget(observer);
    });

    view! {
        <section id="about" class="px-4 sm:px-6 lg:px-8 pt-16 sm:pt-20 [content-visibility:auto] [contain-intrinsic-size:1px_900px]">
            <div class="mx-auto max-w-6xl">
                <div class="grid gap-10 lg:grid-cols-2 items-start">

                    // LEFT: About text
                    <div class="space-y-4">
                        <h2 class="text-3xl sm:text-4xl font-semibold tracking-tight text-white/90">
                            "About"
                        </h2>

                        <p class="text-white/70 leading-relaxed max-w-prose">
                        {ABOUT_P1}
                        </p>

                        <p class="text-white/60 leading-relaxed max-w-prose">
                        {ABOUT_P2}
                        </p>
                    </div>

                    // RIGHT: Terminal
                    <div class="lg:pt-2 [content-visibility:auto] [contain-intrinsic-size:1px_900px]">
                        <div node_ref=terminal_ref class=format!("{GLASS_TERMINAL} p-5 sm:p-7")>
                            // Faux window header
                            <div class="relative z-10 flex items-center justify-between pb-4">
                                <div class="flex items-center gap-2">
                                    <span class="h-2.5 w-2.5 rounded-full bg-red-400/70"></span>
                                    <span class="h-2.5 w-2.5 rounded-full bg-yellow-300/70"></span>
                                    <span class="h-2.5 w-2.5 rounded-full bg-green-400/70"></span>
                                </div>
                                <div class="text-xs text-white/40 font-mono">"matheus@sky: ~"</div>
                            </div>

                            <TypingTerminal active=is_visible.read_only() />

                            // Subtle bottom fade
                            <div class="pointer-events-none absolute inset-x-0 bottom-0 h-10 bg-gradient-to-t from-black/30 to-transparent"></div>
                        </div>
                    </div>

                </div>
            </div>
        </section>
    }
}

#[component]
fn TypingTerminal(active: ReadSignal<bool>) -> impl IntoView {
    let line_idx = RwSignal::new(0usize);
    let char_idx = RwSignal::new(0usize);

    Effect::new(move |_| {
        if !active.get() {
            return;
        }

        let li = line_idx.get();
        let ci = char_idx.get();

        if li >= TERMINAL_SCRIPT.len() {
            return;
        }

        let line = TERMINAL_SCRIPT[li];

        let ms = if line.starts_with('$') { 280 } else { 38 };
        let delay = Duration::from_millis(ms);

        set_timeout(
            move || {
                if ci < line.len() {
                    char_idx.set(ci + 1);
                } else {
                    char_idx.set(0);
                    line_idx.set(li + 1);
                }
            },
            delay,
        );
    });

    view! {
        <pre class="
        relative z-10
        font-mono
        text-[13px] sm:text-sm
        leading-relaxed
        text-white/75
        whitespace-pre-wrap
        ">
            {move || {
                let mut out = String::new();
                let li = line_idx.get();
                let ci = char_idx.get();
                for (i, line) in TERMINAL_SCRIPT.iter().enumerate() {
                    if i < li {
                        out.push_str(line);
                        out.push('\n');
                    } else if i == li {
                        out.push_str(&line[..ci.min(line.len())]);
                        break;
                    }
                }
                out
            }} <span class="inline-block animate-pulse text-emerald-300">"▮"</span>
        </pre>
    }
}
