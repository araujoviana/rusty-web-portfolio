use leptos::prelude::*;

use crate::wasm::cloud_shader::init_hero;

#[component]
pub fn Cloud() -> impl IntoView {
    Effect::new(|_| {
        init_hero("hero-canvas").expect("cannot init hero");
    });
    view! {
        <canvas
            id="hero-canvas"
            class="w-[320px] h-[320px] sm:w-[380px] sm:h-[380px] md:w-[420px] md:h-[420px]"
        ></canvas>
    }
}
