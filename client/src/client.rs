use crate::{app_state::AppState, gl_setup, shaders, utils};
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext as GL;
use web_sys::*;

#[wasm_bindgen]
pub struct Client {
    gl: WebGl2RenderingContext,
    program: shaders::cube::Cube,
    state: Mutex<Arc<AppState>>,
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        utils::set_panic_hook();
        let gl = gl_setup::initialize_webgl_context().unwrap();

        gl.enable(GL::BLEND);
        gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);

        gl.clear_color(0.7, 0.7, 0.7, 1.0);
        gl.clear_depth(1.0);

        let program = shaders::Cube::new(&gl);

        Self {
            gl,
            program,
            state: Mutex::new(Arc::new(AppState::new())),
        }
    }

    pub fn update(
        &mut self,
        time: f32,
        canvas_width: f32,
        canvas_height: f32,
    ) -> Result<(), JsValue> {
        let mut state = self.state.lock().unwrap();
        *state = Arc::new(state.updated(
            time,
            crate::app_state::Canvas {
                width: canvas_width,
                height: canvas_height,
            },
        ));
        self.gl
            .viewport(0, 0, canvas_width as i32, canvas_height as i32);
        Ok(())
    }

    pub fn render(&self) {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        let curr_state = self.get_curr_state();

        self.program
            .render(&self.gl, curr_state.canvas, curr_state.angles);
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        let mut state = self.state.lock().unwrap();
        *state = Arc::new(state.resized(crate::app_state::Canvas { width, height }));
    }
}

impl Client {
    fn get_curr_state(&self) -> Arc<AppState> {
        self.state.lock().unwrap().clone()
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}
