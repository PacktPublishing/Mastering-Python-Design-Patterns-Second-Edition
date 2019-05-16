struct Computer {
    serial: String,
    memory: Option<u32>,
    hdd: Option<u32>,
    gpu: Option<String>,
}

impl Computer {
    fn get_info(&self) -> String {
        let info = format!("Memory: {}GB\nHard Disk: {}GB\nGraphics Card: {}",
            self.memory.as_ref().unwrap(),
            self.hdd.as_ref().unwrap(),
            self.gpu.as_ref().unwrap()
        );
        info
    }
}

struct ComputerBuilder {
    computer: Computer,
}

impl ComputerBuilder {
    fn new() -> ComputerBuilder {
        ComputerBuilder {
            computer: Computer {
                serial: String::from("AG23385103"),
                memory: None,
                hdd: None,
                gpu: None,
            }
        }
    }

    fn configure_memory(&mut self, size: u32) {
        self.computer.memory = Some(size);
    }

    fn configure_hdd(&mut self, size: u32) {
        self.computer.hdd = Some(size);
    }
    fn configure_gpu(&mut self, gpu: String) {
        self.computer.gpu = Some(gpu);
    }

    fn get_info(&self) -> String {
        let info = self.computer.get_info().clone();
        info
    }
}

struct HardwareEngineer {
    builder: Option<ComputerBuilder>,
}

impl HardwareEngineer {
    fn construct_computer(&mut self, memory: u32, hdd: u32, gpu: String) {
        self.builder = Some(ComputerBuilder::new());
        
        self.builder.as_mut().unwrap().configure_memory(memory);
        self.builder.as_mut().unwrap().configure_hdd(hdd);
        self.builder.as_mut().unwrap().configure_gpu(gpu);
    }

    fn get_computer_info(&self) -> String {
        self.builder.as_ref().unwrap().get_info()
    }
}

fn main() {
    let mut engineer = HardwareEngineer { builder: None };
    engineer.construct_computer(
        500,
        8,
        String::from("GeForce GTX 650 Ti")
    );

    let computer = engineer.get_computer_info();
    println!("{}", computer);
}
