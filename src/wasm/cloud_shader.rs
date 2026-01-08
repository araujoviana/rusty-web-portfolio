use crate::wasm::init::{
    FULLSCREEN_QUAD, build_program, get_canvas, get_webgl2_context, setup_alpha_blending, uniform,
    upload_array_buffer,
};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{JsCast, prelude::*};
use web_sys::Element;
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

const CLOUD_VERTEX_SHADER: &str = include_str!("shaders/cloudvert.glsl");
pub const CLOUD_FRAGMENT_SHADER: &str = include_str!("shaders/cloudfrag.glsl");

thread_local! {
    static GLOBAL_MOUSE: Rc<RefCell<(f32, f32)>> = {
        let mouse = Rc::new(RefCell::new((0.0f32, 0.0f32)));
        let mouse_for_cb = mouse.clone();

        let on_move = Closure::<dyn FnMut(web_sys::MouseEvent)>::wrap(Box::new(move |e| {
            *mouse_for_cb.borrow_mut() = (e.client_x() as f32, e.client_y() as f32);
        }));

        web_sys::window()
            .unwrap()
            .add_event_listener_with_callback("mousemove", on_move.as_ref().unchecked_ref())
            .unwrap();
        on_move.forget();

        mouse
    };
}

fn global_mouse() -> Rc<RefCell<(f32, f32)>> {
    GLOBAL_MOUSE.with(|m| m.clone())
}

fn scroll_progress() -> f32 {
    let win = match web_sys::window() {
        Some(w) => w,
        None => return 0.0,
    };

    let scroll_y = win.scroll_y().unwrap_or(0.0) as f32;

    let doc = match win.document() {
        Some(d) => d,
        None => return 0.0,
    };
    let el = match doc.document_element() {
        Some(e) => e,
        None => return 0.0,
    };

    let inner_h = win
        .inner_height()
        .ok()
        .and_then(|v| v.as_f64())
        .unwrap_or(1.0) as f32;

    let scroll_max = (el.scroll_height() as f32 - inner_h).max(1.0);
    (scroll_y / scroll_max).clamp(0.0, 1.0)
}

#[derive(Clone, Copy)]
pub struct CloudOptions {
    pub render_scale: f32,
    pub dpr_cap: f32,
    pub time_offset: f32,
    pub seed: f32,
    pub use_scroll: bool,
    pub default_sun: Option<[f32; 3]>,
    pub mouse_mix: f32, // NEW: 0 = no mouse, 1 = interactive
}

impl Default for CloudOptions {
    fn default() -> Self {
        Self {
            render_scale: 0.55,
            dpr_cap: 1.25,
            time_offset: 0.0,
            seed: 0.0,
            use_scroll: false,
            default_sun: Some([0.8, 0.9, 0.6]),
            mouse_mix: 0.0,
        }
    }
}

// pub fn stop_cloud(canvas_id: &str) {
//     if let Ok(canvas) = get_canvas(canvas_id) {
//         let _ = canvas.set_attribute("data-cloud-stop", "1");
//     }
// }

/// Base initializer: same wiring for any fragment shader.
/// Uniforms are required: u_time, u_resolution.
/// Optional: u_mouse, u_sun_dir, u_scroll, u_seed.
pub fn init_cloud(
    canvas_id: &str,
    frag_src: &'static str,
    opts: CloudOptions,
) -> Result<(), JsValue> {
    let canvas = get_canvas(canvas_id)?;
    let gl = get_webgl2_context(&canvas, true)?;
    setup_alpha_blending(&gl);

    let program = build_program(&gl, CLOUD_VERTEX_SHADER, frag_src)?;
    gl.use_program(Some(&program));

    // Keep GPU objects alive by capturing them into the RAF closure.
    let quad_buffer: WebGlBuffer =
        upload_array_buffer(&gl, &FULLSCREEN_QUAD, WebGl2RenderingContext::STATIC_DRAW)?;
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&quad_buffer));

    let position_attr = gl.get_attrib_location(&program, "position") as u32;
    gl.vertex_attrib_pointer_with_i32(position_attr, 2, WebGl2RenderingContext::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(position_attr);

    // Required uniforms
    let time_loc = uniform(&gl, &program, "u_time")?;
    let res_loc = uniform(&gl, &program, "u_resolution")?;

    // Optional uniforms
    let mouse_loc = gl.get_uniform_location(&program, "u_mouse");
    let sun_loc = gl.get_uniform_location(&program, "u_sun_dir");
    let scroll_loc = gl.get_uniform_location(&program, "u_scroll");
    let seed_loc = gl.get_uniform_location(&program, "u_seed");
    let mouse_mix_loc = gl.get_uniform_location(&program, "u_mouse_mix");

    let mouse = global_mouse();

    // Move everything we need into the RAF closure
    let gl = Rc::new(gl);
    let canvas = Rc::new(canvas);
    let program = Rc::new(program);
    let quad_buffer = Rc::new(quad_buffer);
    let mouse_mix_loc = Rc::new(mouse_mix_loc);

    let time_loc = Rc::new(time_loc);
    let res_loc = Rc::new(res_loc);

    let mouse_loc = Rc::new(mouse_loc);
    let sun_loc = Rc::new(sun_loc);
    let scroll_loc = Rc::new(scroll_loc);
    let seed_loc = Rc::new(seed_loc);

    let f: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move |time: f64| {
        if !canvas.is_connected() {
            f.borrow_mut().take();
            return;
        }
        if canvas.get_attribute("data-cloud-stop").as_deref() == Some("1") {
            let _ = canvas.remove_attribute("data-cloud-stop");
            f.borrow_mut().take();
            return;
        }
        let t = (time / 1000.0) as f32 + opts.time_offset;

        let win = web_sys::window().unwrap();
        let mut dpr = win.device_pixel_ratio() as f32;
        dpr = dpr.min(opts.dpr_cap);

        let css_w = canvas.client_width().max(1) as f32;
        let css_h = canvas.client_height().max(1) as f32;

        let pixel_w = (css_w * dpr * opts.render_scale).round() as u32;
        let pixel_h = (css_h * dpr * opts.render_scale).round() as u32;

        if canvas.width() != pixel_w {
            canvas.set_width(pixel_w);
        }
        if canvas.height() != pixel_h {
            canvas.set_height(pixel_h);
        }

        gl.viewport(0, 0, pixel_w as i32, pixel_h as i32);

        gl.use_program(Some(&program));
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&quad_buffer));
        gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        gl.uniform1f(Some(&time_loc), t);
        gl.uniform2f(Some(&res_loc), pixel_w as f32, pixel_h as f32);

        // Optional: seed
        if let Some(loc) = seed_loc.as_ref().as_ref() {
            gl.uniform1f(Some(loc), opts.seed);
        }

        if let Some(loc) = mouse_mix_loc.as_ref().as_ref() {
            gl.uniform1f(Some(loc), opts.mouse_mix);
        }

        // Optional: sun
        if let (Some(loc), Some(sun)) = (sun_loc.as_ref().as_ref(), opts.default_sun) {
            gl.uniform3f(Some(loc), sun[0], sun[1], sun[2]);
        }

        // Optional: mouse (convert window coords -> canvas-local -> pixel space)
        if let Some(loc) = mouse_loc.as_ref().as_ref() {
            let (mx, my) = *mouse.borrow();
            let rect = canvas
                .as_ref()
                .dyn_ref::<Element>()
                .unwrap()
                .get_bounding_client_rect();
            let local_x = (mx as f64 - rect.left()) as f32;
            let local_y = (my as f64 - rect.top()) as f32;

            gl.uniform2f(
                Some(loc),
                local_x * dpr * opts.render_scale,
                local_y * dpr * opts.render_scale,
            );
        }

        // Optional: scroll
        if opts.use_scroll {
            if let Some(loc) = scroll_loc.as_ref().as_ref() {
                gl.uniform1f(Some(loc), scroll_progress());
            }
        }

        gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 6);

        win.request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .unwrap();
    }) as Box<dyn FnMut(f64)>));

    web_sys::window()
        .unwrap()
        .request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())?;

    Ok(())
}

pub fn init_hero(canvas_id: &str) -> Result<(), JsValue> {
    init_cloud(
        canvas_id,
        CLOUD_FRAGMENT_SHADER,
        CloudOptions {
            render_scale: 0.75,
            mouse_mix: 1.0,
            ..Default::default()
        },
    )
}
