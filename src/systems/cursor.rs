use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

use crate::game::{Cursor, Faction, ARENA_HEIGHT, ARENA_WIDTH};

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
                Faction::Blue => input.axis_value("vertical"),
                Faction::Red => input.axis_value("horizontal")
            };

            if let Some(mv_amount) = movement {
                let scaled_amount = 1.2 * mv_amount as f32;
                transform.translate_y(scaled_amount);
            }
        }
    }
    
}
