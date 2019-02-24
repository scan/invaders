use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, PngFormat, Projection, SpriteRender, SpriteSheet,
    SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata,
};

use crate::config::ArenaConfig;

pub const ARENA_WIDTH: f32 = 256.0;
pub const ARENA_HEIGHT: f32 = 224.0;

pub const DEFENDER_WIDTH: f32 = 16.0;
pub const DEFENDER_HEIGHT: f32 = 9.0;

pub struct Defender {
    pub width: f32,
    pub height: f32,
}

impl Defender {
    fn new() -> Defender {
        Defender {
            width: DEFENDER_WIDTH,
            height: DEFENDER_HEIGHT
        }
    }
}

impl Component for Defender {
    type Storage = DenseVecStorage<Self>;
}

pub struct Invaders;

impl SimpleState for Invaders {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let sprite_sheet_handle = load_sprite_sheet(world);

        initialise_camera(world);
        initialise_defender(world, sprite_sheet_handle);
    }
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0, ARENA_WIDTH, 0.0, ARENA_HEIGHT
        )))
        .with(transform)
        .build();
}

fn initialise_defender(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    let mut transform = Transform::default();

    let x = (ARENA_WIDTH / 2.0) - (DEFENDER_WIDTH / 2.0);
    transform.set_xyz(x, 10.0, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(Defender::new())
        .with(transform)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "resources/textures/spritesheet.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "resources/spritesheet.ron",
        SpriteSheetFormat,
        texture_handle,
        (),
        &sprite_sheet_store,
    )
}