use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, PngFormat, Projection, SpriteRender, SpriteSheet,
    SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata,
};

pub struct Game;

pub const ARENA_HEIGHT: f32 = 400.0;
pub const ARENA_WIDTH: f32 = 400.0;

pub const CURSOR_WIDTH: i32 = 48;
pub const CURSOR_HEIGHT: i32 = 48;

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            ARENA_WIDTH,
            0.0,
            ARENA_HEIGHT,
        )))
        .with(transform)
        .build();
}

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        initialise_camera(world);
    }

}

#[derive(PartialEq, Eq)]
pub enum Faction {
    Red,
    Blue
}

pub struct Cursor {
    pub faction: Faction,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32
}

impl Cursor {
    fn new(faction: Faction) -> Cursor {
        Cursor {
            faction,
            x: 0,
            y: 0,
            width: CURSOR_WIDTH,
            height: CURSOR_HEIGHT
        }
    }
}

impl Component for Cursor {
    type Storage = DenseVecStorage<Self>;
}