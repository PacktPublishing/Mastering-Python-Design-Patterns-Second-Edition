use std::io;


// Traits
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

// Frog game
struct Frog {
    name: String,
}

struct Bug {
    name: String,
    action: String,
}

struct FrogWorld {
    world_name: String,
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

// Wizard game
struct Wizard {
    name: String,
}

struct Ork {
    name: String,
    action: String,
}

struct WizardWorld {
    world_name: String,
    player_name: String,
}

impl Hero for Wizard {
    fn interact_with(&self, obstacle: &Box<dyn Obstacle>) {
        let obs = obstacle.name();
        let act = obstacle.action();

        println!("{} the Wizard encounters {} and {}!",
                 self.name,
                 obs,
                 act);
    }
}

impl Obstacle for Ork {
    fn name(&self) -> &String {
        &self.name
    }

    fn action(&self) -> &String {
        &self.action
    }
}

impl World for WizardWorld {
    fn make_character(&self) -> Box<dyn Hero> {
        let player_name = self.player_name.clone();
        Box::new(Wizard { name: player_name })
    }

    fn make_obstacle(&self) -> Box<dyn Obstacle> {
        Box::new(Ork {
            name: String::from("an evil ork"),
            action: String::from("kills it"),
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

fn generate_world(name: String, age: u32) -> Box<dyn World> {
    if age < 18 {
        Box::new(FrogWorld { 
            world_name: String::from("Frog World"),
            player_name: name,
        })
    } else {
        Box::new(WizardWorld { 
            world_name: String::from("Wizard World"),
            player_name: name,
        })
    }
}

fn main() {
    let mut name = String::new();
    let mut age = String::new();

    println!("Hello. What's your name? ");
    io::stdin().read_line(&mut name)
        .expect("Failed to read line");

    println!("Welcome {}. How old are you? ", &name);

    io::stdin().read_line(&mut age)
        .expect("Failed to read line");

    let age: u32 = age.trim().parse()
        .expect("Please type a number!");

    let world = generate_world(name, age);

    let environment = GameEnvironment {
        hero: world.make_character(),
	obstacle: world.make_obstacle(),
    };

    environment.play();
}
