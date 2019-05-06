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

impl Frog {
    fn new(name: String) -> Frog {
        Frog {
            name: name,
        }
    }

    fn interact_with(&self, obstacle: &Bug) {
        let obs = obstacle.name();
        let act = obstacle.action();

        println!("{} the Frog encounters {} and {}!",
                 self.name,
                 obs,
                 act);
    }
}

impl Bug {
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

impl FrogWorld {
    pub fn new(name: String) -> FrogWorld {
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
