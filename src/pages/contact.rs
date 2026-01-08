use crate::utils::glass::*;
use leptos::prelude::*;

// REVIEW this could be useless?
#[cfg(target_arch = "wasm32")]
mod clipboard {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(inline_js = r#"
        export function copy_text(text) {
            try {
                if (navigator && navigator.clipboard && navigator.clipboard.writeText) {
                    navigator.clipboard.writeText(text);
                    return true;
                }
            } catch (e) {}
            return false;
        }
    "#)]
    extern "C" {
        pub fn copy_text(text: &str) -> bool;
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod clipboard {
    pub fn copy_text(_: &str) -> bool {
        false
    }
}

#[component]
pub fn Contact() -> impl IntoView {
    let email: &'static str = "matheusaraujo1@proton.me";
    let github: &'static str = "https://github.com/araujoviana";
    let linkedin: &'static str = "https://www.linkedin.com/in/matheus-g-viana/";

    let mailto = format!("mailto:{email}");

    let (copied, set_copied) = signal(false);

    let on_copy = move |_| {
        if clipboard::copy_text(email) {
            set_copied.set(true);
        }
    };

    view! {
        <section
            id="contact"
            class="[content-visibility:auto] [contain-intrinsic-size:1px_900px] px-4 sm:px-6 lg:px-8 pt-16 sm:pt-20 pb-20"
        >
            <div class="mx-auto max-w-6xl">
                <div class="mb-10 flex items-end justify-between gap-6">
                    <div class="space-y-2">
                        <h2 class="text-3xl sm:text-4xl font-semibold tracking-tight text-white/90">
                            "Contact"
                        </h2>
                        <p class="text-white/60 max-w-prose leading-relaxed">
                            "Ways to get in touch."
                        </p>
                    </div>
                    <div class="hidden sm:block text-xs font-mono text-white/40">"$ links"</div>
                </div>

                <div class=format!("{GLASS_CARD} p-6 sm:p-8")>
                    <div class="relative z-10 flex flex-col gap-6">

                        <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
                            <div class="flex items-center gap-4">
                                <div class="shrink-0">
                                    <svg
                                        class="w-10 h-10 text-white/75"
                                        viewBox="0 0 24 24"
                                        fill="none"
                                        xmlns="http://www.w3.org/2000/svg"
                                        aria-hidden="true"
                                    >
                                        <path
                                            d="M4 6h16v12H4V6Z"
                                            stroke="currentColor"
                                            stroke-width="1.5"
                                            stroke-linejoin="round"
                                        />
                                        <path
                                            d="M4 7l8 6 8-6"
                                            stroke="currentColor"
                                            stroke-width="1.5"
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                        />
                                    </svg>
                                </div>

                                <div class="min-w-0">
                                    <div class="text-sm text-white/65">
                                        <span class="font-mono text-white/55">"email: "</span>
                                        <span class="select-all">{email}</span>
                                    </div>
                                </div>
                            </div>

                            <div class="flex flex-wrap items-center gap-3">
                                <a class=BTN_PRIMARY href=mailto>
                                    "Email"
                                </a>
                                <button type="button" class=BTN_GHOST on:click=on_copy>
                                    <Show
                                        when=move || copied.get()
                                        fallback=|| view! { "Copy email" }
                                    >
                                        "Copied ✓"
                                    </Show>
                                </button>
                            </div>
                        </div>

                        <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                            <a
                                class=SOCIAL_TILE
                                href=github
                                target="_blank"
                                rel="noopener noreferrer"
                            >
                                <div class="flex items-center justify-between gap-3">
                                    <div class="min-w-0">
                                        <div class="text-sm font-semibold text-white/90">
                                            "GitHub"
                                        </div>
                                        <div class="mt-1 text-xs text-white/50 font-mono truncate">
                                            {github}
                                        </div>
                                    </div>
                                    <span class="text-white/45 group-hover:text-white/70 transition">
                                        "↗"
                                    </span>
                                </div>
                            </a>

                            <a
                                class=SOCIAL_TILE
                                href=linkedin
                                target="_blank"
                                rel="noopener noreferrer"
                            >
                                <div class="flex items-center justify-between gap-3">
                                    <div class="min-w-0">
                                        <div class="text-sm font-semibold text-white/90">
                                            "LinkedIn"
                                        </div>
                                        <div class="mt-1 text-xs text-white/50 font-mono truncate">
                                            {linkedin}
                                        </div>
                                    </div>
                                    <span class="text-white/45 group-hover:text-white/70 transition">
                                        "↗"
                                    </span>
                                </div>
                            </a>
                        </div>

                    </div>
                </div>
            </div>
        </section>
    }
}
