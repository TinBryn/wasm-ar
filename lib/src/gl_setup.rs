use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::*;

pub fn initialize_webgl_context() -> Result<WebGlRenderingContext, JsValue> {

    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("arDisplay").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into()?;
    let gl: WebGlRenderingContext = canvas.get_context("webgl")?.unwrap().dyn_into()?;

    Ok(gl)
}
