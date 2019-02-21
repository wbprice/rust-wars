extern crate amethyst;

use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, DrawFlat2D, Event, Pipeline,
                         RenderBundle, Stage, VirtualKeyCode};
use amethyst::utils::application_root_dir;

mod game;
use crate::game::Game;

fn main() -> amethyst::Result<()> {

    amethyst::start_logger(Default::default());

    let path = format!("{}/resources/display_config.ron", application_root_dir());
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build()
        .with_stage(
            Stage::with_backbuffer()
                .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
                .with_pass(DrawFlat2D::new()),
        );

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderBundle::new(pipe, Some(config))
                .with_sprite_sheet_processor()
        )?;

    let mut game = Application::new("./", Game, game_data)?;

    game.run();

    Ok(())
}