use abstract_factory::{FrogWorld, GameEnvironment};

fn main() {
    let environment = GameEnvironment::new(FrogWorld::new(String::from("test_user")));
    environment.play();
}
