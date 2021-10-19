use gfx_maths::{Mat4, Vec3};

pub fn translation_matrix(tx: f32, ty: f32, tz: f32) -> Mat4 {
    Mat4::translate(Vec3::new(tx, ty, tz))
}

pub fn scaling_matrix(sx: f32, sy: f32, sz: f32) -> Mat4 {
    Mat4::scale(Vec3::new(sx, sy, sz))
}
