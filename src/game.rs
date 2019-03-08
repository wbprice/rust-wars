use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, PngFormat, Projection, SpriteRender, SpriteSheet,
    SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata
};

pub struct Game;

pub const ARENA_WIDTH: f32 = 640.0;
pub const ARENA_HEIGHT: f32 = 480.0;

pub const CURSOR_WIDTH: f32 = 64.0;
pub const CURSOR_HEIGHT: f32 = 64.0;

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(2.0);
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

fn initialize_cursor(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    let mut transform = Transform::default();

    let y = ARENA_HEIGHT / 2.0;
    transform.set_xyz(8.0, y, 1.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 4
    };

    world.create_entity()
        .with(Cursor::new(Faction::Blue))
        .with(sprite_render.clone())
        .with(transform)
        .build();
}

fn initialize_tank(world: &mut World, sprite_sheet: SpriteSheetHandle, x: i8, y: i8) {
    let mut transform = Transform::default();

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0
    };

    let tank = Tank::new(Faction::Blue, x, y);
    transform.set_xyz(x as f32 * 48.0, x as f32 * 48.0, 0.0);

    world.create_entity()
        .with(tank)
        .with(sprite_render.clone())
        .with(transform)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/rust-wars-spritesheet.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/rust-wars-spritesheet.ron",
        SpriteSheetFormat,
        texture_handle,
        (),
        &sprite_sheet_store
    )
}

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let sprite_sheet_handle = load_sprite_sheet(world);

        world.register::<Tank>();

        initialize_cursor(world, sprite_sheet_handle.clone());
        initialize_tank(world, sprite_sheet_handle, 2, 2);
        initialise_camera(world);
    }

}

#[derive(PartialEq, Eq)]
pub enum Faction {
    Blue,
    Red
}

pub struct Cursor {
    pub faction: Faction,
    pub width: f32,
    pub height: f32
}

impl Cursor {
    fn new(faction: Faction) -> Cursor {
        Cursor {
            faction,
            width: CURSOR_WIDTH,
            height: CURSOR_HEIGHT
        }
    }
}

impl Component for Cursor {
    type Storage = DenseVecStorage<Self>;
}

pub struct Tank {
    pub faction: Faction,
    pub x: i8,
    pub y: i8
}

impl Tank {
    fn new (faction: Faction, x: i8, y: i8) -> Tank {
        Tank {
            faction,
            x,
            y
        }
    }
}

impl Component for Tank {
    type Storage = DenseVecStorage<Self>;
}