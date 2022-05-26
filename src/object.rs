use crate::{force::Force, vector::Vector2};
#[derive(Debug)]
pub struct Object {
    pub mass: f64,
    pub pos: Vector2,
    pub velocity: Vector2,
}
impl Object {
    pub fn new_with_pos(mass: f64, x: f64, y: f64) -> Self {
        Self {
            mass,
            pos: Vector2(x, y),
            velocity: Vector2(0., 0.),
        }
    }
    pub fn new(mass: f64) -> Self {
        Self::new_with_pos(mass, 0., 0.)
    }
    pub fn distance(&self, other: &Self) -> f64 {
        let min = Vector2(self.pos.0.min(other.pos.0), self.pos.1.min(other.pos.1));
        let max = Vector2(self.pos.0.max(other.pos.0), self.pos.1.max(other.pos.1));
        let diff = max - min;
        diff.len()
    }
    pub fn apply_force(&mut self, force: &Force) {
        //     F = m * a <=>
        // <=> a = F/m
        let accel_intensity = force.intensity / self.mass;
        let accel = Vector2(
            force.direction.0 * accel_intensity,
            force.direction.1 * accel_intensity,
        );
        self.velocity += accel;
    }
}
