use crate::utils::glass::{GLASS_NAV, NAV_LINK};
use leptos::prelude::*;

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <nav class="fixed top-4 left-1/2 -translate-x-1/2 z-50 w-fit max-w-[92vw]">
            <div class=format!(
                "{GLASS_NAV} flex items-center
         gap-2 sm:gap-6
         px-3 sm:px-6
         py-2
         whitespace-nowrap
         overflow-x-auto sm:overflow-visible
         [scrollbar-width:none] [&::-webkit-scrollbar]:hidden",
            )>
                <NavItem text="Home" href="#home" />
                <NavItem text="Skills" href="#skills" />
                <NavItem text="Projects" href="#projects" />
                <NavItem text="Contact" href="#contact" />
                <NavItem text="About" href="#about" />
            </div>
        </nav>
    }
}

#[component]
fn NavItem(text: &'static str, href: &'static str) -> impl IntoView {
    view! {
        <a href=href class=NAV_LINK>
            {text}
        </a>
    }
}
