pub struct EnemySpawnerSystem{
    future_enemies: :
};

impl<'a> System<'a> for EnemySpawnerSystem {
    type SystemData = (
        ReadStorage<'a, Hitbox>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BulletComponent>,
        ReadExpect<'a, Tick>,
    );

    fn run(&mut self, (hitboxes, positions, bullets, tick): Self::SystemData) {

    }
}