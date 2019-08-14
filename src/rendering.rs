use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    prelude::*,
    core::transform::Transform,
    ecs::{ReadExpect, Resources, SystemData, World},
    renderer::{
        pass::DrawFlat2DDesc, types::DefaultBackend, Factory, Format, GraphBuilder, GraphCreator,
        Kind, RenderGroupDesc, SubpassBuilder, Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture
    },
    window::{ScreenDimensions, Window},
};

pub const ARENA_HEIGHT: f32 = 800.0;
pub const ARENA_WIDTH: f32 = 600.0;

pub fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left. 
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

pub fn load_texture<N>(name: N, world: &World) -> Handle<Texture>
where
    N: Into<String>,
{
    let loader = world.read_resource::<Loader>();
    loader.load(
        name,
        ImageFormat::default(),
        (),
        &world.read_resource::<AssetStorage<Texture>>(),
    )
}