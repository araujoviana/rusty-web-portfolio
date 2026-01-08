use std::cell::RefCell;
use std::rc::Rc;

use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};

// TWEAKS
const CLOUD_COUNT_AREA_DIVISOR: f32 = 250_000.0;
const PADDING_MULTIPLIER: f32 = 0.35;
const CLOUD_SIZE_BASE: f32 = 50.0;
const CLOUD_SIZE_VARIATION: f32 = 180.0;
const WIND_SPEED: f32 = 12.0;

#[derive(Clone, PartialEq)]
struct BgCloud {
    id: String,
    x_px: f32,
    y_px: f32,
    size: f32,
    depth: f32,
    seed: f32,
}

fn too_close(x: f32, y: f32, size: f32, others: &[BgCloud]) -> bool {
    let cx = x + size * 0.5;
    let cy = y + size * 0.5;
    let r = size * 0.45;

    for o in others {
        let ocx = o.x_px + o.size * 0.5;
        let ocy = o.y_px + o.size * 0.5;
        let or = o.size * 0.45;

        let dx = cx - ocx;
        let dy = cy - ocy;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist < (r + or) * 0.95 {
            return true;
        }
    }
    false
}

#[component]
pub fn CloudBg() -> impl IntoView {
    // viewport size (for initial spawn area + resize)
    let vw = RwSignal::new(0.0f32);
    let vh = RwSignal::new(0.0f32);

    let scroll_y = RwSignal::new(0.0f32);

    let time_s = RwSignal::new(0.0f32);

    let enabled = move || vw.get() >= 768.0;

    Effect::new(move || {
        let window = web_sys::window().unwrap();
        let perf = window.performance().unwrap();

        let last_ms = RwSignal::new(perf.now() as f32);

        let f = Rc::new(RefCell::new(None::<Closure<dyn FnMut(f64)>>));
        let g = f.clone();

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move |ms: f64| {
            let ms = ms as f32;
            let prev = last_ms.get();
            last_ms.set(ms);

            let dt = ((ms - prev) / 1000.0).clamp(0.0, 0.05);
            time_s.set(time_s.get() + dt);

            let _ = web_sys::window()
                .unwrap()
                .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref());
        }) as Box<dyn FnMut(f64)>));

        let _ =
            window.request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref());
    });

    Effect::new(move || {
        let window = web_sys::window().unwrap();

        let update_size = Closure::wrap(Box::new(move || {
            let window = web_sys::window().unwrap();

            if let Ok(w) = window.inner_width() {
                if let Some(w) = w.as_f64() {
                    vw.set(w as f32);
                }
            }
            if let Ok(h) = window.inner_height() {
                if let Some(h) = h.as_f64() {
                    vh.set(h as f32);
                }
            }
        }) as Box<dyn FnMut()>);

        update_size
            .as_ref()
            .unchecked_ref::<js_sys::Function>()
            .call0(&JsValue::NULL)
            .ok();

        window
            .add_event_listener_with_callback("resize", update_size.as_ref().unchecked_ref())
            .ok();

        scroll_y.set(window.scroll_y().unwrap_or(0.0) as f32);

        let on_scroll = Closure::wrap(Box::new(move || {
            let window = web_sys::window().unwrap();
            scroll_y.set(window.scroll_y().unwrap_or(0.0) as f32);
        }) as Box<dyn FnMut()>);

        window
            .add_event_listener_with_callback("scroll", on_scroll.as_ref().unchecked_ref())
            .ok();

        // CloudBg lives for the whole app lifetime; keep callbacks alive
        update_size.forget();
        on_scroll.forget();
    });

    // spawn clouds once per viewport size
    let clouds = Memo::new(move |_| {
        let width = vw.get();
        let height = vh.get();

        if width < 200.0 || height < 200.0 {
            return Vec::new();
        }

        // allow some spawn outside the viewport so they can "peek in"
        let padding = height * PADDING_MULTIPLIER;
        let min_x = -padding;
        let min_y = -padding;
        let max_x = width + padding;
        let max_y = height + padding;

        // density tweak (smaller divisor = more clouds)
        let count = (((width * height) / CLOUD_COUNT_AREA_DIVISOR).round() as usize).clamp(5, 8);

        let mut v = Vec::with_capacity(count);
        let mut attempts = 0;

        while v.len() < count && attempts < count * 120 {
            attempts += 1;

            let depth = (js_sys::Math::random() as f32).powf(2.0);
            let size = CLOUD_SIZE_BASE + js_sys::Math::random() as f32 * CLOUD_SIZE_VARIATION;

            let x_range = (max_x - min_x - size).max(0.0);
            let y_range = (max_y - min_y - size).max(0.0);

            let x = min_x + js_sys::Math::random() as f32 * x_range;
            let y = min_y + js_sys::Math::random() as f32 * y_range;

            if too_close(x, y, size, &v) {
                continue;
            }

            v.push(BgCloud {
                id: format!("bg-cloud-{}", v.len()),
                x_px: x,
                y_px: y,
                size,
                depth,
                seed: js_sys::Math::random() as f32 * 1000.0,
            });
        }

        v
    });

    view! {
        // Above background (z-0), below content (z-20)
        <Show when=enabled fallback=|| ()>
            <div class="fixed inset-0 z-10 pointer-events-none">
                <For
                    each=move || clouds.get()
                    key=|c| c.id.clone()
                    children=move |cloud| {
                        let id = cloud.id.clone();
                        let id_init = id.clone();
                        let x = cloud.x_px;
                        let y = cloud.y_px;
                        let size = cloud.size;
                        let depth = cloud.depth;
                        let seed = cloud.seed;
                        Effect::new(move || {
                            let _ = crate::wasm::cloud_shader::init_cloud(
                                &id_init,
                                crate::wasm::cloud_shader::CLOUD_FRAGMENT_SHADER,
                                crate::wasm::cloud_shader::CloudOptions {
                                    render_scale: 0.45,
                                    seed,
                                    time_offset: seed * 10.0,
                                    default_sun: Some([0.7, 0.8, 0.9]),
                                    ..Default::default()
                                },
                            );
                        });

                        // init each canvas once

                        view! {
                            <canvas
                                id=id
                                class="absolute will-change-transform"

                                style=move || {
                                    let t = time_s.get();
                                    let width = vw.get();
                                    let padding = vh.get() * PADDING_MULTIPLIER;
                                    let depth_factor = 0.35 + depth * 0.65;
                                    let drift = t * WIND_SPEED * depth_factor;
                                    let span = width + padding * 2.0 + size;
                                    let mut u = (x + drift + padding + size) % span;
                                    if u < 0.0 {
                                        u += span;
                                    }
                                    let x_render = u - padding - size;
                                    let op = 0.35 + depth * 0.45;
                                    let strength = 1.0_f32;
                                    let y_render = y - scroll_y.get() * strength;
                                    format!(
                                        "transform: translate3d({:.1}px,{:.1}px,0);
                                    width:{:.1}px; height:{:.1}px; opacity:{:.3};",
                                        x_render,
                                        y_render,
                                        size,
                                        size,
                                        op,
                                    )
                                }
                            />
                        }
                    }
                />
            </div>
        </Show>
    }
}
