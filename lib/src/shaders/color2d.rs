use std::mem::size_of;
use wasm_bindgen::JsCast;
use web_sys::WebGlBuffer;
use web_sys::WebGlProgram;
use web_sys::WebGlRenderingContext;
use web_sys::WebGlRenderingContext as GL;
use web_sys::WebGlUniformLocation;

pub const COLOR_2D_VERT: &str = include_str!("../../shaders/color_2d_vert.glsl");
pub const COLOR_2D_FRAG: &str = include_str!("../../shaders/color_2d_frag.glsl");

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
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let program = super::create::link_program(gl, COLOR_2D_VERT, COLOR_2D_FRAG).unwrap();

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

    #[allow(clippy::too_many_arguments)]
    pub fn render(
        &self,
        gl: &WebGlRenderingContext,
        bottom: f32,
        top: f32,
        left: f32,
        right: f32,
        canvas_width: f32,
        canvas_height: f32,
    ) {
        gl.use_program(Some(&self.program));

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.vertex_buffer));
        gl.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(0);

        gl.uniform4f(Some(&self.u_color), 0.0, 0.5, 0.5, 1.0);

        let translation = crate::math::translation_matrix(
            2.0 * left / canvas_width - 1.0,
            2.0 * bottom / canvas_height - 1.0,
            0.0,
        );

        let scale = crate::math::scaling_matrix(
            2.0 * (right - left) / canvas_width,
            2.0 * (top - bottom) / canvas_height,
            1.0,
        );

        let transform = translation * scale;

        gl.uniform_matrix4fv_with_f32_array(
            Some(&self.u_transform),
            false,
            &transform.values,
        );

        gl.uniform1f(Some(&self.u_opacity), 1.0);

        gl.draw_arrays(GL::TRIANGLES, 0, (self.array_len / 2) as i32);
    }
}
