use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

use crate::game::{Cursor, Movement, ARENA_HEIGHT, ARENA_WIDTH, CURSOR_HEIGHT};

pub struct CursorSystem;

impl<'s> System<'s> for CursorSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Cursor>,
        Read<'s, InputHandler<String, String>> 
    ); 

    fn run(&mut self, (mut transforms, cursors, input): Self::SystemData) {
        for (cursor, transform) in (&cursors, &mut transforms).join() {

            let movement = match cursor.faction {
                Movement::Vertical => input.axis_value("vertical"),
                Movement::Horizontal => input.axis_value("horizontal")
            };

            if let Some(mv_amount) = movement {
                let scaled_amount = 1.2 * mv_amount as f32;
                let paddle_y = transform.translation().y;
                transform.set_y(
                    (paddle_y + scaled_amount)
                        .min(ARENA_HEIGHT - CURSOR_HEIGHT * 0.5)
                        .max(CURSOR_HEIGHT * 0.5),
                );
            }
        }
    }
    
}
