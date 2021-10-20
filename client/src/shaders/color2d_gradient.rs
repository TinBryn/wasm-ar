pub const COLOR_2D_GRADIENT_VERT: &str = include_str!("../../shaders/color_2d_gradient_vert.glsl");
pub const COLOR_2D_GRADIENT_FRAG: &str = include_str!("../../shaders/color_2d_gradient_frag.glsl");

use std::mem::size_of;

use gfx_maths::Mat4;
use gfx_maths::Vec3;
use wasm_bindgen::JsCast;
use web_sys::WebGlBuffer;
use web_sys::WebGlProgram;
use web_sys::WebGl2RenderingContext;
use web_sys::WebGl2RenderingContext as GL;
use web_sys::WebGlUniformLocation;

use crate::app_state;

pub struct Color2D {
    program: WebGlProgram,
    vertex_buffer: WebGlBuffer,
    u_color: WebGlUniformLocation,
    u_opacity: WebGlUniformLocation,
    u_transform: WebGlUniformLocation,
    array_len: usize,
}

#[allow(dead_code)]
impl Color2D {
    pub fn new(gl: &WebGl2RenderingContext) -> Self {
        let program =
            ::shaders::compile::link_program(gl, COLOR_2D_GRADIENT_VERT, COLOR_2D_GRADIENT_FRAG)
                .unwrap();

        let verticies: [f32; 12] = [
            0.0, 1.0, //
            0.0, 0.0, //
            1.0, 1.0, //
            1.0, 1.0, //
            0.0, 0.0, //
            1.0, 0.0, //
        ];

        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<js_sys::WebAssembly::Memory>()
            .unwrap()
            .buffer();

        let vertices_location = verticies.as_ptr() as u32 / size_of::<f32>() as u32;
        let vert_array = js_sys::Float32Array::new(&memory_buffer).subarray(
            vertices_location,
            vertices_location + verticies.len() as u32,
        );

        let vertex_buffer = gl.create_buffer().ok_or("failed to create buffer").unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);

        let u_color = gl.get_uniform_location(&program, "uColor").unwrap();
        let u_opacity = gl.get_uniform_location(&program, "uOpacity").unwrap();
        let u_transform = gl.get_uniform_location(&program, "uTransform").unwrap();

        Self {
            program,
            vertex_buffer,
            u_color,
            u_opacity,
            u_transform,
            array_len: verticies.len(),
        }
    }

    pub fn render(
        &self,
        gl: &WebGl2RenderingContext,
        control: app_state::Control,
        canvas: app_state::Canvas,
    ) {
        gl.use_program(Some(&self.program));

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.vertex_buffer));
        gl.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(0);

        gl.uniform4f(Some(&self.u_color), 0.0, 0.5, 0.5, 1.0);

        let translation = Mat4::translate(Vec3::new(
            2.0 * control.left / canvas.width - 1.0,
            2.0 * control.bottom / canvas.height - 1.0,
            0.0,
        ));

        let scale = Mat4::scale(Vec3::new(
            2.0 * (control.right - control.left) / canvas.width,
            2.0 * (control.top - control.bottom) / canvas.height,
            1.0,
        ));

        let transform = translation * scale;

        gl.uniform_matrix4fv_with_f32_array(Some(&self.u_transform), false, &transform.values);

        gl.uniform1f(Some(&self.u_opacity), 1.0);

        gl.draw_arrays(GL::TRIANGLES, 0, (self.array_len / 2) as i32);
    }
}
