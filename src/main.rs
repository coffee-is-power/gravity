mod force;
mod gravity;
mod object;
mod vector;

use force::Force;
use gravity::calculate_weight;
use object::Object;
use piston_window::{*, types::Color};
use vector::Vector2;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut earth = Object::new_with_pos(300., 420., 340.);
    let mut bullet = Object::new_with_pos(1., 420., 240.);
    bullet.apply_force(&Force {
        intensity: 5.,
        direction: Vector2(1., 0.)
    });
    while let Some(event) = window.next() {
        match event {
            Event::Loop(Loop::Update(_)) => {
                let bullet_weight = calculate_weight(&earth, &bullet);
                bullet.apply_force(&bullet_weight);
                earth.apply_force(&calculate_weight(&bullet, &earth));
                bullet.pos += bullet.velocity;
                earth.pos += earth.velocity;
            },
            _ => {}
        }
        window.draw_2d(&event, |mut context, graphics, _device| {
            clear([1.0; 4], graphics);
            context = context.scale(0.5, 0.5);
            render(&earth, [0., 1., 0., 1.], 50.,&context, graphics);
            render(&bullet, [0.2, 0.2, 0.2, 1.], 10., &context, graphics);
        });
    }
}
fn render<G: Graphics>(obj: &Object, color: Color, radius: f64, context: &Context, g: &mut G) {
    ellipse(color, [obj.pos.0-radius, obj.pos.1-radius, radius*2., radius*2.], context.transform, g)
}
