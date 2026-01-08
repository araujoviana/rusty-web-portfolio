use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGlBuffer;
use web_sys::WebGlUniformLocation;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext, WebGlProgram, WebGlShader};

// These are some helpers for rendering shaders using WebGL
// AKA inner wiring thats never touched

pub const FULLSCREEN_QUAD: [f32; 12] = [
    -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0,
];

/// Compile a GLSL shader and return the handle or an error
pub fn compile_shader(
    gl: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, JsValue> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| JsValue::from_str("Unable to create shader object"))?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);
    if gl
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(JsValue::from_str(
            &gl.get_shader_info_log(&shader).unwrap_or_default(),
        ))
    }
}

/// Link a vertex and fragment shader into a program
pub fn link_program(
    gl: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, JsValue> {
    let program = gl
        .create_program()
        .ok_or_else(|| JsValue::from_str("Unable to create shader program"))?;
    gl.attach_shader(&program, vert_shader);
    gl.attach_shader(&program, frag_shader);
    gl.link_program(&program);
    if gl
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(JsValue::from_str(
            &gl.get_program_info_log(&program).unwrap_or_default(),
        ))
    }
}

pub fn get_canvas(canvas_id: &str) -> Result<HtmlCanvasElement, JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("no window"))?;
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("no document"))?;

    let canvas = document
        .get_element_by_id(canvas_id)
        .ok_or_else(|| JsValue::from_str("canvas not found"))?
        .dyn_into::<HtmlCanvasElement>()?;

    Ok(canvas)
}

pub fn get_webgl2_context(
    canvas: &HtmlCanvasElement,
    alpha: bool,
) -> Result<WebGl2RenderingContext, JsValue> {
    let attrs = js_sys::Object::new();
    js_sys::Reflect::set(&attrs, &"alpha".into(), &alpha.into())?;
    js_sys::Reflect::set(&attrs, &"premultipliedAlpha".into(), &false.into())?;

    let gl = canvas
        .get_context_with_context_options("webgl2", &attrs.into())?
        .ok_or_else(|| JsValue::from_str("no WebGL2 context"))?
        .dyn_into::<WebGl2RenderingContext>()?;

    Ok(gl)
}

pub fn build_program(
    gl: &WebGl2RenderingContext,
    vertex_src: &str,
    fragment_src: &str,
) -> Result<WebGlProgram, JsValue> {
    let vs = compile_shader(gl, WebGl2RenderingContext::VERTEX_SHADER, vertex_src)?;
    let fs = compile_shader(gl, WebGl2RenderingContext::FRAGMENT_SHADER, fragment_src)?;
    let program = link_program(gl, &vs, &fs)?;

    Ok(program)
}

pub fn setup_alpha_blending(gl: &WebGl2RenderingContext) {
    gl.enable(WebGl2RenderingContext::BLEND);
    gl.blend_func(
        WebGl2RenderingContext::SRC_ALPHA,
        WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA,
    );
    gl.clear_color(0.0, 0.0, 0.0, 0.0);
}

pub fn uniform(
    gl: &WebGl2RenderingContext,
    program: &WebGlProgram,
    name: &str,
) -> Result<WebGlUniformLocation, JsValue> {
    gl.get_uniform_location(program, name)
        .ok_or_else(|| JsValue::from_str(&format!("missing uniform {}", name)))
}

pub fn upload_array_buffer(
    gl: &WebGl2RenderingContext,
    data: &[f32],
    usage: u32,
) -> Result<WebGlBuffer, JsValue> {
    let buffer = gl
        .create_buffer()
        .ok_or_else(|| JsValue::from_str("Failed to create buffer"))?;

    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

    unsafe {
        let array = js_sys::Float32Array::view(data);
        gl.buffer_data_with_array_buffer_view(WebGl2RenderingContext::ARRAY_BUFFER, &array, usage);
    }

    Ok(buffer)
}
