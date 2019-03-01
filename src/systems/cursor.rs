use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

use crate::game::{Cursor, ARENA_HEIGHT, ARENA_WIDTH, CURSOR_WIDTH, CURSOR_HEIGHT};

pub struct CursorSystem;

impl<'s> System<'s> for CursorSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Cursor>,
        Read<'s, InputHandler<String, String>>
    );

    fn run(&mut self, (mut transforms, cursors, input): Self::SystemData) {
        for (cursor, transform) in (&cursors, &mut transforms).join() {

            let vertical_movement = input.axis_value("vertical");
            let horizontal_movement = input.axis_value("horizontal");

            if let Some(mv_amount) = vertical_movement {
                if vertical_movement > 0 {
                    cursor.y += 1;
                } else {
                    cursor.y -= 1;
                }
            }

            if let Some(mv_amount) = horizontal_movement {
                if horizontal_movement > 0 {
                    cursor.x += 1;
                } else {
                    cursor.x -= 1;
                }
            }

            set_position(&cursor, &transform);
        }
    }

}

fn set_position(cursor: &Cursor, transform: &mut Transform) {
    let (scale_x, scale_y) = {
        let scale = transform.scale();
        (scale.x, scale.y)
    };

    transform.set_x((cursor.x * 32.0) * scale_x);
    transform.set_y((cursor.y * 32.0) * scale_y);
}