use crate::{
    components::{Background, CloudBg, Footer, NavBar},
    pages::{About, Contact, Home, Projects, Skills},
};

use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="min-h-screen flex flex-col text-slate-100 relative ">

            <Background />

            <CloudBg />

            // PAGE ROUTES
            <div class="relative z-20 flex min-h-screen flex-col">
                <NavBar />
                <main class="flex-1">
                    <Home />
                    <Skills />
                    <Projects />
                    <Contact />
                    <About />

                </main>
                <Footer />
            </div>
        </div>
    }
}
