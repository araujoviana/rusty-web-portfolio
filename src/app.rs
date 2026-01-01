use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <button
            on:click=move |_| { *set_count.write() += 1}
        >
        "Click me: " {move || count.get()}
        </button>
    }
}
