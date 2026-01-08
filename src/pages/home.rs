use crate::components::HomeHero;
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div id="home" class="px-4 sm:px-6 lg:px-8 mt-10 sm:mt-10 px-4 sm:px-6 pt-12 sm:pt-12">
            <HomeHero />
        </div>
    }
}
