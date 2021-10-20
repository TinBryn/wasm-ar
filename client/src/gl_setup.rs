use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::*;

#[allow(dead_code)]
pub fn initialize_webgl_context() -> Result<WebGl2RenderingContext, JsValue> {

    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("arDisplay").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into()?;
    let gl: WebGl2RenderingContext = canvas.get_context("webgl2")?.unwrap().dyn_into()?;

    gl.enable(WebGl2RenderingContext::DEPTH_TEST);
    gl.enable(WebGl2RenderingContext::CULL_FACE);

    Ok(gl)
}
