use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    ecs::{world::Builder, World},
    renderer::{
        Camera, Flipped, PngFormat, Projection, SpriteRender, SpriteSheet, SpriteSheetFormat,
        SpriteSheetHandle, Texture, TextureMetadata,
    },
};

pub const CAMERA_HEIGHT: f32 = 640.0;
pub const CAMERA_WIDTH: f32 = 480.0;

pub fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.,
            CAMERA_HEIGHT,
            0.,
            CAMERA_WIDTH,
        )))
        .with(transform)
        .build();
}

fn load_sprite_sheet(name: &str, world: &mut World) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            format!("assets/textures/{}.png", name),
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        format!("assets/textures/{}_spritesheet.ron", name), // Here we load the associated ron file
        SpriteSheetFormat,
        texture_handle, // We pass it the handle of the texture we want it to use
        (),
        &sprite_sheet_store,
    )
}

// use crate::rendering::Visual::*;
// use specs::{Component, DenseVecStorage, Join, ReadStorage};

// use crate::physics::Position;
// #[derive(Component)]
// pub enum Visual {
//     Circle([f32; 4], f64),
//     Sprite(<G2d<'static> as Graphics>::Texture),
// }

// pub fn render(
//     c: Context,
//     g: &mut G2d,
//     (positions, visuals): (ReadStorage<Position>, ReadStorage<Visual>),
// ) {
//     let [width, height] = c.viewport.unwrap().draw_size;
//     let width = width as f64;
//     let height = height as f64;
//     (&positions, &visuals).join().for_each(|(pos, vis)| {
//         let x = pos.0.x as f64;
//         let y = pos.0.y as f64;
//         match vis {
//             Circle(color, radius) => {
//                 //Figure out where to fetch the resolution of the window so that I don't have to hardcode 480 for the height here.
//                 ellipse(
//                     *color,
//                     [x - radius, height - (y - radius), 2. * radius, 2. * radius],
//                     c.transform,
//                     g,
//                 );
//             }
//             Sprite(img) => {
//                 let img_height = img.get_height() as f64;
//                 let img_width = img.get_width() as f64;
//                 let mut transform = c.transform;
//                 transform[0][2] += 2./width*(x + img_width/2.) ;
//                 transform[1][2] -= 2./height*((height - y) + img_height/2.);
//                 image(
//                     img,
//                     transform,
//                     g,
//                 );
//             },
//         }
//     });
// }
