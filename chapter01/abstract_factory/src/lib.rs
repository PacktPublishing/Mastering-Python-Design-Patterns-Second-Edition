trait Hero {
    fn new(name: String) -> Self;
    fn interact_with<T: Obstacle>(&self, obstacle: &T);
}

trait Obstacle {
    fn new() -> Self;
    fn name(&self) -> &String;
    fn action(&self) -> &String;
}

trait World {
    fn new(name: String) -> Self;
    fn make_character(&self) -> Frog;
    fn make_obstacle(&self) -> Bug;
}

pub enum WorldSelection {
    FrogWorld,
}

struct Frog {
    name: String,
}

struct Bug {
    name: String,
    action: String,
}

pub struct FrogWorld {
    world_name: String,
    player_name: String,
}

impl Hero for Frog {
    fn new(name: String) -> Frog {
        Frog {
            name: name,
        }
    }

    fn interact_with<T: Obstacle>(&self, obstacle: &T) {
        let obs = obstacle.name();
        let act = obstacle.action();

        println!("{} the Frog encounters {} and {}!",
                 self.name,
                 obs,
                 act);
    }
}

impl Obstacle for Bug {
    fn new() -> Bug {
        Bug {
            name: String::from("a bug"),
            action: String::from("eats it"),
        }
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn action(&self) -> &String {
        &self.action
    }
}

impl World for FrogWorld {
    fn new(name: String) -> FrogWorld {
        let world_name = String::from("\n\n\t------ Frog World ------");
        println!("{}", world_name);

        FrogWorld {
            world_name: world_name,
            player_name: name,
        }
    }

    fn make_character(&self) -> Frog {
        let player_name = self.player_name.clone();
        Frog::new(player_name)
    }

    fn make_obstacle(&self) -> Bug {
        Bug::new()
    }
}


// Game environment
pub struct GameEnvironment {
    hero: Frog,
    obstacle: Bug,
}

impl GameEnvironment {
    pub fn new(factory: FrogWorld) -> GameEnvironment {
        GameEnvironment {
            hero: factory.make_character(),
            obstacle: factory.make_obstacle(),
        }
    }

    pub fn play(&self) {
        self.hero.interact_with(&self.obstacle);
    }
}


// This function is created because structs having a trait can expose their methods to outside.
pub fn generate_world(world_name: WorldSelection, player_name: String) -> FrogWorld {
    match world_name {
        WorldSelection::FrogWorld => FrogWorld::new(String::from(player_name)),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
