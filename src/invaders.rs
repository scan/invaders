use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, PngFormat, Projection, SpriteRender, SpriteSheet,
    SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata,
};

pub const ARENA_WIDTH: f32 = 100.0;
pub const ARENA_HEIGHT: f32 = 100.0;

pub const DEFENDER_WIDTH: f32 = 16.0;
pub const DEFENDER_HEIGHT: f32 = 8.0;

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

        world.register::<Defender>();

        initialise_camera(world);
        initialise_defender(world);
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

fn initialise_defender(world: &mut World) {
    let mut transform = Transform::default();

    let x = ARENA_WIDTH / 2.0;
    transform.set_xyz(x, ARENA_HEIGHT - 5.0, 0.0);

    world
        .create_entity()
        .with(Defender::new())
        .with(transform)
        .build();
}