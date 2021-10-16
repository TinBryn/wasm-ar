use std::ops::Mul;

pub fn translation_matrix(tx: f32, ty: f32, tz: f32) -> Matrix4 {
    let mut result = [0.; 16];

    result[0] = 1.0;
    result[5] = 1.0;
    result[10] = 1.0;
    result[15] = 1.0;

    result[12] = tx;
    result[13] = ty;
    result[14] = tz;

    Matrix4 { elements: result }
}

pub fn scaling_matrix(sx: f32, sy: f32, sz: f32) -> Matrix4 {
    let mut result = [0.0; 16];

    result[0] = sx;
    result[5] = sy;
    result[10] = sz;
    result[15] = 1.0;

    Matrix4 { elements: result }
}

pub struct Matrix4 {
    pub elements: [f32; 16],
}

impl Mul for Matrix4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut elements = [0f32; 16];

        let a = &self.elements;
        let b = &rhs.elements;

        elements[0] = a[0] * b[0] + a[1] * b[4] + a[2] * b[8] + a[3] * b[12];
        elements[1] = a[0] * b[1] + a[1] * b[5] + a[2] * b[9] + a[3] * b[13];
        elements[2] = a[0] * b[2] + a[1] * b[6] + a[2] * b[10] + a[3] * b[14];
        elements[3] = a[0] * b[3] + a[1] * b[7] + a[2] * b[11] + a[3] * b[15];

        elements[4] = a[4] * b[0] + a[5] * b[4] + a[6] * b[8] + a[7] * b[12];
        elements[5] = a[4] * b[1] + a[5] * b[5] + a[6] * b[9] + a[7] * b[13];
        elements[6] = a[4] * b[2] + a[5] * b[6] + a[6] * b[10] + a[7] * b[14];
        elements[7] = a[4] * b[3] + a[5] * b[7] + a[6] * b[11] + a[7] * b[15];

        elements[8] = a[8] * b[0] + a[9] * b[4] + a[10] * b[8] + a[11] * b[12];
        elements[9] = a[8] * b[1] + a[9] * b[5] + a[10] * b[9] + a[11] * b[13];
        elements[10] = a[8] * b[2] + a[9] * b[6] + a[10] * b[10] + a[11] * b[14];
        elements[11] = a[8] * b[3] + a[9] * b[7] + a[10] * b[11] + a[11] * b[15];

        elements[12] = a[12] * b[0] + a[13] * b[4] + a[14] * b[8] + a[15] * b[12];
        elements[13] = a[12] * b[1] + a[13] * b[5] + a[14] * b[9] + a[15] * b[13];
        elements[14] = a[12] * b[2] + a[13] * b[6] + a[14] * b[10] + a[15] * b[14];
        elements[15] = a[12] * b[3] + a[13] * b[7] + a[14] * b[11] + a[15] * b[15];

        Matrix4 {
            elements,
        }
    }
}
