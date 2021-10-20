use web_sys::WebGlProgram;
use web_sys::WebGl2RenderingContext;
use web_sys::WebGl2RenderingContext as GL;
use web_sys::WebGlShader;
pub fn compile_shader(
    gl: &WebGl2RenderingContext,
    shader_kind: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = gl
        .create_shader(shader_kind)
        .ok_or_else(|| String::from("Cannot create shader"))?;

    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, GL::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| "can't get shader info log".to_string()))
    }
}

pub fn link_program(
    gl: &WebGl2RenderingContext,
    vert_source: &str,
    frag_source: &str,
) -> Result<WebGlProgram, String> {
    let program = gl.create_program().ok_or("Cannot create program")?;

    let vert_shader = compile_shader(gl, GL::VERTEX_SHADER, vert_source).unwrap();
    let frag_shader = compile_shader(gl, GL::FRAGMENT_SHADER, frag_source).unwrap();

    gl.attach_shader(&program, &vert_shader);
    gl.attach_shader(&program, &frag_shader);
    gl.link_program(&program);

    if gl
        .get_program_parameter(&program, GL::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(gl
            .get_program_info_log(&program)
            .unwrap_or_else(|| "can't get program info log".to_string()))
    }
}
