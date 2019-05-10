trait Hero {
    fn new(name: String) -> Heros;
    fn interact_with(&self, obstacle: Obstacles);
}

trait Obstacle {
    fn new() -> Obstacles;
    fn name(&self) -> &String;
    fn action(&self) -> &String;
}

trait World {
    fn new(name: String) -> Worlds;
    fn make_character(&self) -> Heros;
    fn make_obstacle(&self) -> Obstacles;
}

pub enum WorldSelection {
    FrogWorld,
    WizardWorld,
}

pub enum Worlds {
    FrogWorld(FrogWorld),
    WizardWorld(WizardWorld),
}

pub enum Heros {
    Frog(Frog),
    Wizard(Wizard),
}

pub enum Obstacles {
    Bug(Bug),
    Ork(Ork),
}

// Frog game

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
    fn new(name: String) -> Heros {
        Heros::Frog(Frog {
            name: name,
        })
    }

    fn interact_with(&self, obstacle: Obstacles) {
        let obs = obstacle.name();
        let act = obstacle.action();

        println!("{} the Frog encounters {} and {}!",
                 self.name,
                 obs,
                 act);
    }
}

impl Obstacle for Bug {
    fn new() -> Obstacles {
        Obstacles::Bug(Bug {
            name: String::from("a bug"),
            action: String::from("eats it"),
        })
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn action(&self) -> &String {
        &self.action
    }
}

impl World for FrogWorld {
    fn new(name: String) -> Worlds {
        let world_name = String::from("Frog World");
        println!("\n\n\t------ {} ------", world_name);

        Worlds::FrogWorld(FrogWorld {
            world_name: world_name,
            player_name: name,
        })
    }
    
    fn make_character(&self) -> Heros {
        let player_name = self.player_name.clone();
        Frog::new(player_name)
    }

    fn make_obstacle(&self) -> Obstacles {
        Bug::new()
    }
}


// Wizard game

struct Wizard {
    name: String,
}

struct Ork {
    name: String,
    action: String,
}

pub struct WizardWorld {
    world_name: String,
    player_name: String,
}

impl Hero for Wizard {
    fn new(name: String) -> Heros {
        Heros::Wizard(Wizard {
            name: name,
        })
    }

    fn interact_with(&self, obstacle: Obstacles) {
        let obs = obstacle.name();
        let act = obstacle.action();

        println!("{} the Wizard battles against {} and {}!",
                 self.name,
                 obs,
                 act);
    }
}
    
impl Obstacle for Ork {
    fn new() -> Obstacles {
        Obstacles::Ork(Ork {
            name: String::from("an evil ork"),
            action: String::from("kills it"),
        })
    }
    fn name(&self) -> &String {
        &self.name
    }

    fn action(&self) -> &String {
        &self.action
    }
}

impl World for WizardWorld {
    fn new(name: String) -> Worlds {
        let world_name = String::from("Wizard World");
        println!("\n\n\t------ {} ------", world_name);

        Worlds::WizardWorld(WizardWorld {
            world_name: world_name,
            player_name: name,
        })
    }

    fn make_character(&self) -> Heros {
        let player_name = self.player_name.clone();
        Wizard::new(player_name)
    }

    fn make_obstacle(&self) -> Obstacles {
        Ork::new()
    }
}

// Game environment

pub struct GameEnvironment {
    hero: String,
    obstacle: String,
}

impl GameEnvironment {
    pub fn new(factory: Worlds) -> GameEnvironment {
        let character = factory.make_character();
        let obstacle = factory.make_obstacle();

        GameEnvironment {
            hero: character,
            obstacle: obstacle,
        }
    }

    pub fn play(&self) {
        self.hero.interact_with(&self.obstacle);
    }
}


// This function is created because structs having a trait cannot expose their methods to outside.
pub fn generate_world(world_name: WorldSelection, player_name: String) -> Worlds {
    match world_name {
        WorldSelection::FrogWorld => FrogWorld::new(String::from(player_name)),
        WorldSelection::WizardWorld => WizardWorld::new(String::from(player_name)),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
