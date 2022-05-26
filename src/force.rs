use crate::vector::Vector2;
#[derive(Debug, Copy, Clone)]
pub struct Force {
    pub intensity: f64,
    pub direction: Vector2,
}
