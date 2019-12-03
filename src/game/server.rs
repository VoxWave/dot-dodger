use std::io;
use std::io::Write;
use std::net::{SocketAddr, TcpListener, TcpStream, UdpSocket};
use std::sync::mpsc::channel;
use std::time::Instant;

use ggez::{
    event::{self, EventHandler},
    graphics, Context, GameResult,
};

use serde::{Deserialize, Serialize};

use specs::{
    world::Builder, Dispatcher, DispatcherBuilder, Entities, Entity, Join, ReadStorage, World,
    WriteExpect,
};

use crate::na::{zero, Point2, Vector2};

use crate::bullet::{BulletComponent, BulletPatternSystem, NewBullets};
use crate::collision::{CollisionSystem, Hitbox};
use crate::physics::{Acceleration, PhysicsSystem, Position, Velocity};
use crate::player::{AxisState, PlayerControlSystem, PlayerInputState};
use crate::rendering::{Renderer, Visual};
use crate::Tick;
use crate::FRAME;

pub struct Server<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
    last_tick: Instant,
    renderer: Renderer,
    tcp_listener: TcpListener,
    tcp_streams: Vec<TcpStream>,
    udp_socket: UdpSocket,
}

impl<'a, 'b> Server<'a, 'b> {
    pub fn new(ctx: &mut Context, port: &str) -> Self {
        let mut world = World::new();
        world.register::<Position>();
        world.register::<Velocity>();
        world.register::<Hitbox>();
        world.register::<Acceleration>();
        world.register::<BulletComponent>();
        world.register::<Visual>();

        //let (send, recv) = channel();

        let dispatcher = DispatcherBuilder::new()
            //.with(PlayerControlSystem::new(recv), "player_control_system", &[])
            .with(BulletPatternSystem, "bullet_pattern_system", &[])
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
        let address = format!("0.0.0.0:{}", port);
        let tcp_listener = TcpListener::bind(&address).unwrap();
        tcp_listener
            .set_nonblocking(true)
            .expect("Cannot set the TCP listener to non-blocking");
        let tcp_streams = Vec::with_capacity(10);
        let udp_socket = UdpSocket::bind(&address).unwrap();

        Server {
            world,
            dispatcher,
            last_tick: Instant::now(),
            renderer,
            tcp_listener,
            tcp_streams,
            udp_socket,
        }
    }

    fn update_inputs(&mut self) {}

    fn send_state(&mut self) {
        if !self.tcp_streams.is_empty() {
            let mut frame = Frame(Vec::new());

            self.add_player_positions(&mut frame);
            self.add_new_bullets(&mut frame);

            let serialized_frame = &mut bincode::serialize(&frame).unwrap();

            let mut message: Vec<u8> =
                Vec::from(&(serialized_frame.len() as u64).to_be_bytes()[..]);
            message.append(serialized_frame);

            for stream in &mut self.tcp_streams {
                stream.write(&*message).unwrap();
            }
        }
    }

    fn handle_new_players(&mut self) {
        let mut new_streams = Vec::new();
        for connection in self.tcp_listener.incoming() {
            match connection {
                Ok(stream) => new_streams.push(stream),
                Err(ref e) if e.kind() != io::ErrorKind::WouldBlock => {
                    panic!("IO error");
                }
                _ => break,
            }
        }
        if !new_streams.is_empty() {
            // Construct the initial frame.
            let mut frame = Frame(Vec::new());
            self.add_player_positions(&mut frame);
            self.add_all_bullets(&mut frame);
            // Form the frame message which consists of the lenght of the frame in bytes and the frame itself.
            let serialized_frame = &mut bincode::serialize(&frame).unwrap();
            let mut message: Vec<u8> =
                Vec::from(&(serialized_frame.len() as u64).to_be_bytes()[..]);
            message.append(serialized_frame);
            // Send the frame message to each connection.
            for stream in &mut new_streams {
                // Create an entity for the new player.
                let entity = self.create_entity();
                // Send the players id to the player.
                stream.write(&entity.id().to_be_bytes()).unwrap();
                stream.write(&*message).unwrap();
            }
            //Store the newly formed connections for later use.
            self.tcp_streams.append(&mut new_streams);
        };
    }

    fn add_player_positions(&mut self, frame: &mut Frame) {
        self.world.exec(
            |(ents, plrs, poss): (
                Entities,
                ReadStorage<PlayerInputState>,
                ReadStorage<Position>,
            )| {
                (&*ents, &plrs, &poss).join().for_each(|(ent, _, pos)| {
                    frame.0.push(Event::PlayerPos {
                        id: ent.id(),
                        pos: (pos.0.x, pos.0.y),
                    });
                });
            },
        );
    }

    fn add_all_bullets(&mut self, frame: &mut Frame) {
        self.world.exec(
            |(bllts, poss, vels, accs): (
                ReadStorage<BulletComponent>,
                ReadStorage<Position>,
                ReadStorage<Velocity>,
                ReadStorage<Acceleration>,
            )| {
                (&bllts, &poss, &vels, &accs)
                    .join()
                    .for_each(|(_, pos, vel, acc)| {
                        frame.0.push(Event::CreateBullet {
                            pos: (pos.0.x, pos.0.y),
                            vel: (vel.0.x, vel.0.y),
                            acc: (acc.0.x, acc.0.y),
                        });
                    });
            },
        );
    }

    fn add_new_bullets(&mut self, frame: &mut Frame) {
        self.world.exec(
            |(mut bllts, poss, vels, accs): (
                WriteExpect<NewBullets>,
                ReadStorage<Position>,
                ReadStorage<Velocity>,
                ReadStorage<Acceleration>,
            )| {
                for blt in &mut bllts.0.drain(..) {
                    let pos = poss.get(blt).unwrap().0;
                    let vel = vels.get(blt).unwrap().0;
                    let acc = accs.get(blt).unwrap().0;
                    frame.0.push(Event::CreateBullet {
                        pos: (pos.x, pos.y),
                        vel: (vel.x, vel.y),
                        acc: (acc.x, acc.y),
                    });
                }
            },
        );
    }

    fn create_entity(&mut self) -> Entity {
        self.world
            .create_entity()
            .with(PlayerInputState(AxisState::Neutral, AxisState::Neutral))
            .with(Visual::Sprite("player".to_string()))
            .with(Position(Point2::new(400., 400.)))
            .with(Velocity(Vector2::new(1., 1.)))
            .with(Acceleration(zero()))
            .with(Hitbox::Point(Point2::new(0., 0.)))
            .build()
    }
}

impl<'a, 'b> EventHandler for Server<'a, 'b> {
    //The game logic loop which also contains all the networking.
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.update_inputs();
        if self.last_tick.elapsed() >= FRAME {
            self.dispatcher.dispatch(&self.world.res);
            self.world.maintain();
            self.world.write_resource::<Tick>().0 += 1;
            self.last_tick = Instant::now();
            self.send_state();
        }
        self.handle_new_players();
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

#[derive(Serialize, Deserialize)]
pub enum Event {
    CreateBullet {
        pos: (f64, f64),
        vel: (f64, f64),
        acc: (f64, f64),
    },
    PlayerPos {
        id: u32,
        pos: (f64, f64),
    },
}

#[derive(Serialize, Deserialize)]
pub struct Frame(pub Vec<Event>);
