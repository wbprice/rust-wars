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

pub const CURSOR_WIDTH: f32 = 48.0;
pub const CURSOR_HEIGHT: f32 = 48.0;

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

fn initialize_cursor(world: &mut World) {
    let mut transform = Transform::default();

    let y = ARENA_HEIGHT / 2.0;
    transform.set_xyz(CURSOR_WIDTH * 0.5, y, 0.0);

    world.create_entity()
        .with(Cursor::new(Faction::Blue))
        .with(transform)
        .build();
}

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.register::<Cursor>();

        initialize_cursor(world);
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
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32
}

impl Cursor {
    fn new(faction: Faction) -> Cursor {
        Cursor {
            faction,
            x: 0.0,
            y: 0.0,
            width: CURSOR_WIDTH,
            height: CURSOR_HEIGHT
        }
    }
}

impl Component for Cursor {
    type Storage = DenseVecStorage<Self>;
}