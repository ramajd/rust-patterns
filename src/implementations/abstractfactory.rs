trait CPU {
    fn process(&self);
}

trait MMU {
    fn manage(&self);
}

trait AbstractFactory<C: CPU, M: MMU> {
    fn new_cpu(&self) -> C;
    fn new_mmu(&self) -> M;
}

// =====================================

struct EmberCPU {}

impl CPU for EmberCPU {
    fn process(&self) {
        println!("Ember performs the process");
    }
}

struct EnginolaCPU {}

impl CPU for EnginolaCPU {
    fn process(&self) {
        println!("Enginola performs the process");
    }
}

struct EmberMMU {}

impl MMU for EmberMMU {
    fn manage(&self) {
        println!("Ember manages the memory");
    }
}

struct EnginolaMMU {}

impl MMU for EnginolaMMU {
    fn manage(&self) {
        println!("Enginola manages the memory");
    }
}

struct EmberToolkit;

impl AbstractFactory<EmberCPU, EmberMMU> for EmberToolkit {
    fn new_cpu(&self) -> EmberCPU {
        EmberCPU {}
    }

    fn new_mmu(&self) -> EmberMMU {
        EmberMMU {}
    }
}

struct EnginolaToolkit;

impl AbstractFactory<EnginolaCPU, EnginolaMMU> for EnginolaToolkit {
    fn new_cpu(&self) -> EnginolaCPU {
        EnginolaCPU {}
    }

    fn new_mmu(&self) -> EnginolaMMU {
        EnginolaMMU {}
    }
}

pub fn run_abstract_factory_pattern() {
    let ember = EmberToolkit;
    let cpu = ember.new_cpu();
    cpu.process();
    let mmu = ember.new_mmu();
    mmu.manage();

    let enginola = EnginolaToolkit;
    let cpu = enginola.new_cpu();
    cpu.process();
    let mmu = enginola.new_mmu();
    mmu.manage();
}
