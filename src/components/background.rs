use leptos::prelude::*;
use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::{closure::Closure, JsCast};

#[component]
pub fn Background() -> impl IntoView {
    let daytime = RwSignal::new(0.0f64);

    let update = {
        let daytime = daytime.clone();
        move || {
            let win = web_sys::window().unwrap();
            let doc = win.document().unwrap();
            let el = doc.document_element().unwrap();

            let scroll_y = win.scroll_y().unwrap_or(0.0);
            let scrollable = (el.scroll_height() as f64 - el.client_height() as f64).max(1.0);
            let p = (scroll_y / scrollable).clamp(0.0, 1.0);
            daytime.set(p);
        }
    };

    // run once on mount
    Effect::new({
        let update = update.clone();
        move || update()
    });

    // throttle scroll -> RAF (ONE callback, reused)
    let ticking = Rc::new(Cell::new(false));
    let raf_cb = {
        let ticking = ticking.clone();
        let update = update.clone();
        Closure::<dyn FnMut(f64)>::wrap(Box::new(move |_ts: f64| {
            update();
            ticking.set(false);
        }))
    };

    let _cleanup = window_event_listener(leptos::ev::scroll, {
        let ticking = ticking.clone();
        let raf_cb = raf_cb.as_ref().unchecked_ref::<js_sys::Function>().clone();
        move |_| {
            if ticking.get() {
                return;
            }
            ticking.set(true);
            let _ = web_sys::window().unwrap().request_animation_frame(&raf_cb);
        }
    });

    // keep RAF closure alive for app lifetime
    raf_cb.forget();

    let bg_style = move || {
        let p = daytime.get();
        let h = 205.0 + (230.0 - 205.0) * p;
        let s = 90.0 + (40.0 - 90.0) * p;
        let l = 65.0 + (8.0 - 65.0) * p;
        format!("background-color: hsl({h} {s}% {l}%);")
    };

    view! {
        <div class="pointer-events-none fixed inset-0 -z-0" style=bg_style>
            <div class="absolute inset-0 bg-gradient-to-b from-white/10 via-transparent to-black/50"></div>
        </div>
    }
}
