use crate::{force::Force, object::Object};

const GRAVITY_CONSTANT: f64 = 6.6743015151515;

pub fn calculate_weight(obj: &Object, other_obj: &Object) -> Force {
    let intensity =
        GRAVITY_CONSTANT * ((obj.mass * other_obj.mass) / obj.distance(other_obj).powf(2.0));
    let direction = (obj.pos - other_obj.pos).normalize();
    Force {
        intensity,
        direction,
    }
}
