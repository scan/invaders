use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

use crate::invaders::{Defender, ARENA_WIDTH, DEFENDER_WIDTH};

pub struct DefenderSystem;

impl<'s> System<'s> for DefenderSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Defender>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut transforms, defenders, input): Self::SystemData) {
        for (_defender, transform) in (&defenders, &mut transforms).join() {
            let movement = input.axis_value("defender");

            if let Some(mv_amount) = movement {
                let scaled_amount = 1.2 * mv_amount as f32;
                let defender_x = transform.translation().x;
                transform.set_x(
                    (defender_x + scaled_amount)
                        .min(ARENA_WIDTH - DEFENDER_WIDTH * 0.5)
                        .max(DEFENDER_WIDTH * 0.5),
                );
            }
        }
    }
}