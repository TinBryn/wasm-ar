pub const CUBE_VERT: &str = include_str!("../../shaders/cube.vert");
pub const CUBE_FRAG: &str = include_str!("../../shaders/cube.frag");

use std::mem::size_of;

use gfx_maths::Mat4;
use gfx_maths::Quaternion;
use gfx_maths::Vec3;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;
use web_sys::WebGl2RenderingContext as GL;
use web_sys::WebGlBuffer;
use web_sys::WebGlProgram;
use web_sys::WebGlUniformLocation;

use crate::app_state;
use crate::utils;

pub struct Cube {
    program: WebGlProgram,
    vertex_buffer: WebGlBuffer,
    u_transform: WebGlUniformLocation,
    u_projection: WebGlUniformLocation,
    array_len: usize,
}

#[allow(dead_code)]
impl Cube {
    pub fn new(gl: &WebGl2RenderingContext) -> Self {
        let program = ::shaders::compile::link_program(gl, CUBE_VERT, CUBE_FRAG).unwrap();

        let corners = [
            [1.0, 1.0, 1.0],
            [1.0, 1.0, -1.0],
            [1.0, -1.0, 1.0],
            [1.0, -1.0, -1.0],
            [-1.0, 1.0, 1.0],
            [-1.0, 1.0, -1.0],
            [-1.0, -1.0, 1.0],
            [-1.0, -1.0, -1.0],
        ];

        let normals = [
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [1.0, 0.0, 1.0],
            [-1.0, 0.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, 0.0, -1.0],
        ];

        let faces = [
            ([0, 1, 2], [2, 1, 3], 0), // +X [0, 1, 2, 3]
            ([0, 4, 1], [1, 4, 5], 1), // +Y [0, 1, 4, 5]
            ([0, 2, 4], [4, 2, 6], 2), // +Z [0, 2, 4, 6]
            ([4, 6, 5], [5, 6, 7], 3), // -X [4, 5, 6, 7]
            ([2, 3, 6], [6, 3, 7], 4), // -Y [2, 3, 6, 7]
            ([1, 5, 3], [3, 5, 7], 5), // -Z [1, 3, 5, 7]
        ];

        let verticies: Vec<f32> = IntoIterator::into_iter(faces)
            .flat_map(|(first, second, normal)| {
                let first_face = first
                    .iter()
                    .copied()
                    .map(|index| (corners[index], normals[normal]));
                let second_face = second
                    .iter()
                    .copied()
                    .map(|index| (corners[index], normals[normal]));

                first_face.chain(second_face).collect::<Vec<_>>()
            })
            .flat_map(|(pos, normal)| vec![pos, normal])
            .flatten()
            .collect();

        // utils::log(&format!("{:?}", &verticies));

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

        let u_transform = gl.get_uniform_location(&program, "uTransform").unwrap();
        let u_projection = gl.get_uniform_location(&program, "uProjection").unwrap();

        utils::log(&format!("num verticies = {}", verticies.len() / 6));

        Self {
            program,
            vertex_buffer,
            u_projection,
            u_transform,
            array_len: verticies.len(),
        }
    }

    pub fn render(
        &self,
        gl: &WebGl2RenderingContext,
        canvas: app_state::Canvas,
        angles: app_state::Angles,
    ) {
        gl.use_program(Some(&self.program));

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.vertex_buffer));

        // setup position attribute
        {
            gl.vertex_attrib_pointer_with_i32(
                0,
                3,
                GL::FLOAT,
                false,
                6 * size_of::<f32>() as i32,
                0,
            );
            gl.enable_vertex_attrib_array(0);
        }

        // setup normal attribute
        {
            gl.vertex_attrib_pointer_with_i32(
                1,
                3,
                GL::FLOAT,
                false,
                6 * size_of::<f32>() as i32,
                3 * size_of::<f32>() as i32,
            );
            gl.enable_vertex_attrib_array(1);
        }

        let rotation_x = Quaternion::axis_angle(Vec3::new(1.0, 0.0, 0.0), angles.x);
        let rotation_y = Quaternion::axis_angle(Vec3::new(0.0, 1.0, 0.0), angles.y);
        let rotation_z = Quaternion::axis_angle(Vec3::new(0.0, 0.0, 1.0), angles.z);
        let rotation = rotation_x * rotation_y * rotation_z;
        // let rotation = Quaternion::axis_angle(Vec3::new(1.0, 1.0, 1.0).normalized(), 0.4);
        let rotation = Mat4::rotate(rotation);
        let translation = Mat4::translate(Vec3::new(0.0, 0.0, 3.0));
        // let scale = Mat4::scale(Vec3::new(0.5, 0.5, 0.5));
        let scale = Mat4::scale(Vec3::new(1.0, 1.0, 1.0));

        let transform = translation * scale * rotation;

        // crate::utils::log(&format!("transform = {:?}", transform));

        gl.uniform_matrix4fv_with_f32_array(Some(&self.u_transform), false, &transform.values);

        let projection = Mat4::perspective_opengl(
            std::f32::consts::PI / 1.5,
            0.1,
            10.0,
            canvas.width / canvas.height,
        );

        // let projection = Mat4::identity();

        // utils::log(&format!("projection = {:?}", projection));

        gl.uniform_matrix4fv_with_f32_array(Some(&self.u_projection), false, &projection.values);

        gl.draw_arrays(GL::TRIANGLES, 0, (self.array_len / 6) as i32);
    }
}
