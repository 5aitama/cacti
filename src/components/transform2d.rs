
use cgmath::{
    Vector2,
    Vector3,
    Matrix4,
};

pub struct Transform2D {
    pub pos: Vector2<f32>,
}

impl Transform2D {
    pub fn new(pos: Vector2<f32>) -> Self {
        Self {
            pos,
        }
    }

    pub fn get_matrix(&self) -> Matrix4<f32> {
        Matrix4::<f32>::from_translation(Vector3::new(self.pos.x, self.pos.y, 0.0))
    }
}