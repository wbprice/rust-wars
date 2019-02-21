use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler

pub struct CursorSystem;

impl<'s> System<'s> for CursorSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Game>,
        Read<'s, InputHandler<String, String>> 
    );

    
}
