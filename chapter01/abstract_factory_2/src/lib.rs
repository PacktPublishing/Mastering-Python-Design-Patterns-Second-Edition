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
    type Hero;
    type TestTest;

    fn new(name: String) -> Self;
    fn make_character(&self) -> Hero;
    fn make_obstacle(&self) -> TestTest;
}

pub enum WorldSelection {
    FrogWorld,
    WizardWorld,
}

pub enum TheWorld {
    FrogWorld(FrogWorld),
    WizardWorld(WizardWorld),
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
    type Hero = Frog;
    type TestTest = Bug;

    fn new(name: String) -> FrogWorld {
        let world_name = String::from("Frog World");
        println!("\n\n\t------ {} ------", world_name);

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
    fn new(name: String) -> Wizard {
        Wizard {
            name: name,
        }
    }

    fn interact_with<T: Obstacle>(&self, obstacle: &T) {
        let obs = obstacle.name();
        let act = obstacle.action();

        println!("{} the Wizard battles against {} and {}!",
                 self.name,
                 obs,
                 act);
    }
}
    
impl Obstacle for Ork {
    fn new() -> Ork {
        Ork {
            name: String::from("an evil ork"),
            action: String::from("kills it"),
        }
    }
    fn name(&self) -> &String {
        &self.name
    }

    fn action(&self) -> &String {
        &self.action
    }
}

impl World for WizardWorld {
    type Hero = Wizard;
    type TestTest = Ork;

    fn new(name: String) -> WizardWorld {
        let world_name = String::from("Wizard World");
        println!("\n\n\t------ {} ------", world_name);

        WizardWorld {
            world_name: world_name,
            player_name: name,
        }
    }

    fn make_character(&self) -> Wizard {
        let player_name = self.player_name.clone();
        Wizard::new(player_name)
    }

    fn make_obstacle(&self) -> Ork {
        Ork::new()
    }
}

// Game environment

pub struct GameEnvironment<U, V> {
    hero: U,
    obstacle: V,
}

impl<U, V> GameEnvironment<U, V> {
    pub fn new<W: World>(factory: W) -> GameEnvironment<U, V> {
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
pub fn generate_world(world_name: WorldSelection, player_name: String) -> TheWorld {
    match world_name {
        WorldSelection::FrogWorld => TheWorld::FrogWorld(FrogWorld::new(String::from(player_name))),
        WorldSelection::WizardWorld => TheWorld::WizardWorld(WizardWorld::new(String::from(player_name))),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
