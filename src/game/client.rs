use std::collections::HashMap;
use std::io::Read;
use std::net::TcpStream;
use std::sync::mpsc::channel;
use std::time::Instant;

use ggez::{
    event::{self, EventHandler},
    graphics, Context, GameResult,
};

use specs::{world::Builder, Dispatcher, DispatcherBuilder, Entity, World};

use crate::na::{zero, Point2, Vector2};

use super::server::{Event, Frame};
use crate::bullet::{create_bullet, BulletComponent, BulletPatternSystem, NewBullets};
use crate::collision::{CollisionSystem, Hitbox};
use crate::physics::{Acceleration, PhysicsSystem, Position, Velocity};
use crate::player::PlayerControlSystem;
use crate::rendering::{Renderer, Visual};
use crate::Tick;
use crate::FRAME;

pub struct Client<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
    players: HashMap<u32, Entity>,
    player_id: u32,
    renderer: Renderer,
    server_connection: TcpStream,
}

impl<'a, 'b> Client<'a, 'b> {
    pub fn new(ctx: &mut Context, address: &str) -> Self {
        let mut world = World::new();
        world.register::<Position>();
        world.register::<Velocity>();
        world.register::<Hitbox>();
        world.register::<Acceleration>();
        world.register::<BulletComponent>();
        world.register::<Visual>();

        let mut dispatcher = DispatcherBuilder::new()
            //.with(PlayerControlSystem::new(recv), "player_control_system", &[])
            //.with(BulletPatternSystem, "bullet_pattern_system", &[])
            .with(
                PhysicsSystem,
                "physics_system",
                &[/*"player_control_system"*/],
            )
            //.with(CollisionSystem, "collision_system", &["physics_system"])
            .build();

        world.add_resource(Tick(0));
        world.add_resource(NewBullets(Vec::new()));

        let renderer = Renderer::new(ctx);

        let mut server_connection = TcpStream::connect(address).unwrap();
        let mut player_id_bytes = [0; 4];
        server_connection.read_exact(&mut player_id_bytes).unwrap();
        let player_id = u32::from_be_bytes(player_id_bytes);

        let player = world
            .create_entity()
            .with(Visual::Sprite("player".to_string()))
            .with(Position(Point2::new(400., 400.)))
            .with(Velocity(zero()))
            .with(Acceleration(zero()))
            .with(Hitbox::Point(Point2::new(0., 0.)))
            // .with(Visual::Sprite(player_texture))
            .build();
        // world.add_resource(PlayerHandle(player));
        let mut players = HashMap::new();
        players.insert(player_id, player);

        Client {
            world,
            dispatcher,
            players,
            player_id,
            renderer,
            server_connection,
        }
    }

    fn fetch_frame(&mut self) {
        let mut len_bytes = [0; 8];
        self.server_connection.read_exact(&mut len_bytes).unwrap();
        let len = u64::from_be_bytes(len_bytes);

        let mut frame_bytes = Vec::with_capacity(len as usize);
        (&mut self.server_connection)
            .take(len)
            .read_to_end(&mut frame_bytes)
            .unwrap();
        let frame: Frame = bincode::deserialize(&frame_bytes).unwrap();
        for event in frame.0 {
            match event {
                Event::CreateBullet { pos, vel, acc } => {
                    create_bullet(
                        self.world.create_entity(),
                        Point2::new(pos.0, pos.1),
                        Vector2::new(vel.0, vel.1),
                        Vector2::new(acc.0, acc.1),
                        5.,
                    );
                }

                Event::PlayerPos { id, pos } => {
                    match self.players.get(&id) {
                        Some(entity) => {
                            let storage = &mut self.world.write_storage::<Position>();
                            let mut position = storage.get_mut(*entity).unwrap();
                            (&mut position).0.x = pos.0;
                            (&mut position).0.y = pos.1;
                        }
                        None => {
                            let player = self
                                .world
                                .create_entity()
                                .with(Visual::Sprite("player".to_string()))
                                .with(Position(Point2::new(400., 400.)))
                                .with(Velocity(zero()))
                                .with(Acceleration(zero()))
                                .with(Hitbox::Point(Point2::new(0., 0.)))
                                // .with(Visual::Sprite(player_texture))
                                .build();
                            self.players.insert(id, player);
                        }
                    };
                }
            };
        }
    }
}

impl<'a, 'b> EventHandler for Client<'a, 'b> {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.dispatcher.dispatch(&self.world.res);
        self.world.maintain();
        self.fetch_frame();
        self.world.write_resource::<Tick>().0 += 1;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        let renderer = &self.renderer;
        self.world.exec(|s| {
            renderer.render(ctx, s);
        });
        graphics::present(ctx)
    }
}
