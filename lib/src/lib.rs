mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[macro_use]
extern crate lazy_static;

mod app_state;
mod gl_setup;
mod math;
mod shaders;

use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

#[wasm_bindgen]
pub struct Client {
    gl: WebGlRenderingContext,
    program: shaders::color2d_gradient::Color2D,
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        utils::set_panic_hook();
        let gl = gl_setup::initialize_webgl_context().unwrap();

        gl.enable(GL::BLEND);
        gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);

        gl.clear_color(0.6, 0.4, 0.0, 1.0);
        gl.clear_depth(1.0);

        let program = shaders::color2d_gradient::Color2D::new(&gl);

        Self { gl, program }
    }

    pub fn update(
        &mut self,
        time: f32,
        canvas_width: f32,
        canvas_height: f32,
    ) -> Result<(), JsValue> {
        app_state::update_dynamic_data(time, canvas_width, canvas_height);
        Ok(())
    }

    pub fn render(&self) {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        let curr_state = app_state::get_curr_state();

        self.program.render(
            &self.gl,
            curr_state.control_bottom,
            curr_state.control_top,
            curr_state.control_left,
            curr_state.control_right,
            curr_state.canvas_width,
            curr_state.canvas_height,
        );
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}
