pub trait Prototype {
    fn clone_prototype(&self) -> Box<dyn Prototype>;
    fn get_name(&self) -> String;
    fn execute(&self);
}

#[derive(Debug, Clone)]
struct PrototypeAlpha {
    name: String,
}

impl PrototypeAlpha {
    fn new() -> Self {
        let name = "AlphaVersion";
        println!("create prototype: {}", name);
        Self {
            name: name.to_owned(),
        }
    }
}

impl Prototype for PrototypeAlpha {
    fn clone_prototype(&self) -> Box<dyn Prototype> {
        println!("- clone prototype: {}", self.name);
        Box::new(self.clone())
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn execute(&self) {
        println!("  - {}: does something", self.name);
    }
}

#[derive(Debug, Clone)]
struct PrototypeBeta {
    name: String,
}

impl PrototypeBeta {
    fn new() -> Self {
        let name = "BetaVersion";
        println!("create prototype: {}", name);
        Self {
            name: name.to_owned(),
        }
    }
}

impl Prototype for PrototypeBeta {
    fn clone_prototype(&self) -> Box<dyn Prototype> {
        println!("- clone prototype: {}", self.name);
        Box::new(self.clone())
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn execute(&self) {
        println!("  - {}: does something", self.name)
    }
}

struct PrototypeManager {
    prototypes: Vec<Box<dyn Prototype>>,
}

impl PrototypeManager {
    fn new() -> Self {
        Self { prototypes: vec![] }
    }

    fn add_prototype(&mut self, prototype: Box<dyn Prototype>) {
        self.prototypes.push(prototype);
    }

    fn create_prototype(&self, name: &str) -> Option<Box<dyn Prototype>> {
        for p in self.prototypes.iter() {
            if &p.get_name() == name {
                return Some(p.clone_prototype());
            }
        }
        None
    }
}

pub fn run_prototype_logic() {
    let mut manager = PrototypeManager::new();
    manager.add_prototype(Box::new(PrototypeAlpha::new()));
    manager.add_prototype(Box::new(PrototypeBeta::new()));

    let prototype_names = vec!["AlphaVersion", "BetaVersion", "AlphaVersion", "BetaVersion"];
    for name in prototype_names {
        match manager.create_prototype(name) {
            Some(p) => {
                p.execute();
            }
            None => println!("Error: prototype not found for {}", name),
        }
    }
}
