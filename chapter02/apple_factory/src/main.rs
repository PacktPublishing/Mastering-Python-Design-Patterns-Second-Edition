// const MINI14: String = String::from("1.4GHz Mac mini");
const MINI14: &str = "1.4GHz Mac mini";

trait Computer {
    fn get_info(&self) -> String;
}

struct AppleFactory {}

struct MacMini14 {
    memory: u32,
    hdd: u32,
    gpu: String,
}

impl AppleFactory {
    fn build_computer(model: &str) -> Option<Box<Computer>> {
        // https://stackoverflow.com/questions/25383488/how-to-match-a-string-against-string-literals-in-rust
        match model {
            "MINI14" => Some(Box::new(MacMini14 {
                memory: 4,
                hdd: 500,
                gpu: String::from("Intel HD Graphics 5000"),
                })),
            _ => None,
        }
    }
}

impl Computer for MacMini14 {
    fn get_info(&self) -> String {
        format!("Model: {}\nMemory: {}\nHard Disk: {}\nGraphics Card: {}",
            MINI14,
            &self.memory,
            &self.hdd,
            &self.gpu
        )
    }
}


fn main() {
    let mac_mini = AppleFactory::build_computer("MINI14").unwrap();
    println!("{}", mac_mini.get_info());
}
