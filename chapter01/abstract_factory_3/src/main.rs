trait Hero {
    fn interact_with(&self, obstacle: &Box<dyn Obstacle>);
}

trait Obstacle {
    fn name(&self) -> &String;
    fn action(&self) -> &String;
}

trait World {
    fn make_character(&self) -> Box<dyn Hero>;
    fn make_obstacle(&self) -> Box<dyn Obstacle>;
}

struct Frog {
    name: String,
}

struct Bug {
    name: String,
    action: String,
}

struct FrogWorld {
    player_name: String,
}

impl Hero for Frog {
    // https://stackoverflow.com/questions/45116984/the-trait-cannot-be-made-into-an-object
    // fn new(player_name: String) -> Box<dyn Hero> {
    //     Box::new(Frog { player_name: player_name })
    // }

    fn interact_with(&self, obstacle: &Box<dyn Obstacle>) {
        let obs = obstacle.name();
        let act = obstacle.action();

        println!("{} the Frog encounters {} and {}!",
                 self.name,
                 obs,
                 act);
    }
}

impl Obstacle for Bug {
    fn name(&self) -> &String {
        &self.name
    }

    fn action(&self) -> &String {
        &self.action
    }
}

impl World for FrogWorld {
    fn make_character(&self) -> Box<dyn Hero> {
        let player_name = self.player_name.clone();
        Box::new(Frog { name: player_name })
    }

    fn make_obstacle(&self) -> Box<dyn Obstacle> {
        Box::new(Bug {
            name: String::from("a bug"),
            action: String::from("eats it"),
        })
    }
}

struct GameEnvironment {
    hero: Box<dyn Hero>,
    obstacle: Box<dyn Obstacle>,
}

impl GameEnvironment {
    fn play(&self) {
        self.hero.interact_with(&self.obstacle);
    }
}


    
fn main() {
    let world = FrogWorld { player_name: String::from("testuser0") };
    let environment = GameEnvironment {
        hero: world.make_character(),
	obstacle: world.make_obstacle(),
    };

    environment.play();
}
