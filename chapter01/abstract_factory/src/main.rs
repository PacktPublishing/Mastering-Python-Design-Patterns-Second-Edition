use abstract_factory::{GameEnvironment, WorldSelection};

fn main() {
    let world = abstract_factory::generate_world(WorldSelection::FrogWorld, String::from("test_user"));
    let environment = GameEnvironment::new(world);
    environment.play();
}
