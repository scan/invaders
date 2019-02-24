extern crate amethyst;
extern crate serde;

mod config;
mod invaders;
mod systems;

use amethyst::prelude::*;
use amethyst::core::transform::TransformBundle;
use amethyst::input::InputBundle;
use amethyst::renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage};

use crate::config::ArenaConfig;
use crate::invaders::Invaders;

fn main() -> amethyst::Result<()> {
    use amethyst::utils::application_root_dir;

    amethyst::start_logger(Default::default());

    let path = format!("{}/resources/display_config.ron", application_root_dir());
    let config = DisplayConfig::load(&path);

    let arena_config_path = format!("{}/resources/config.ron", application_root_dir());
    let arena_config = ArenaConfig::load(&arena_config_path);

    let binding_path = format!("{}/resources/bindings_config.ron", application_root_dir());
    let input_bundle = InputBundle::<String, String>::new().with_bindings_from_file(binding_path)?;

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
            )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::DefenderSystem, "defender_system", &["input_system"]);

    let mut game = Application::new("./", Invaders, game_data)?;

    game.run();

    Ok(())
}
