use nalgebra::Point2;
use specs::{ReadStorage, System, ReadExpect, Entities, Builder, Entity, LazyUpdate, Read};

use crate::{collision::Hitbox, physics::Position, bullet::{NormalBullet, SpiralSprayer}, Tick, rendering::Visual};

pub struct EnemySpawnerSystem{
    future_enemies: Vec<(Point2<f64>, EnemyType, u64)>,
    unspawned_enemies: usize,
}

pub enum EnemyType {
    StationarySpiral,
}

impl EnemySpawnerSystem {
    pub fn new() -> Self {
        let mut future_enemies = vec![
            (Point2::new(5., 5.), EnemyType::StationarySpiral, 60*1),
            (Point2::new(0., 0.), EnemyType::StationarySpiral, 60*20),
            (Point2::new(10., 10.), EnemyType::StationarySpiral, 60*10)
        ];
        future_enemies.sort_by_key(|(_,_,tick)| *tick);
        Self {
            future_enemies,
            unspawned_enemies: 0,
        }
    }
}

impl<'a> System<'a> for EnemySpawnerSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, LazyUpdate>,
        ReadExpect<'a, Tick>,
    );

    fn run(&mut self, (entities, world, tick): Self::SystemData) {
        for (index, (pos, enemy_type, spawn_tick)) in self.future_enemies[self.unspawned_enemies..].iter().enumerate() {
            if *spawn_tick == tick.0 {
                self.unspawned_enemies = index + 1;
                match enemy_type {
                    EnemyType::StationarySpiral => {
                        create_stationary_spiral(world.create_entity(&entities), *pos);
                    },
                }
            } else if *spawn_tick > tick.0 {
                break;
            }
        }
    }
}

pub fn create_stationary_spiral(builder: impl Builder, pos: Point2<f64>) -> Entity {
    builder
        .with(Visual::Sprite("spiral_turret".to_string(), 1.))
        .with(Position(pos))
        .with(SpiralSprayer)
        .build()
}