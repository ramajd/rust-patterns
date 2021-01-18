trait TimeInterface {
    fn tell(&self);
}

struct TimeImpl {
    hr: u8,
    min: u8,
}

impl TimeImpl {
    fn new(hr: u8, min: u8) -> Self {
        Self { hr, min }
    }
}

impl TimeInterface for TimeImpl {
    fn tell(&self) {
        println!("current time is: {:02}:{:02}", self.hr, self.min);
    }
}

struct CivilianTimeImpl {
    hr: u8,
    min: u8,
    is_eve: bool,
}

impl CivilianTimeImpl {
    fn new(hr: u8, min: u8, is_eve: bool) -> Self {
        Self { hr, min, is_eve }
    }
}

impl TimeInterface for CivilianTimeImpl {
    fn tell(&self) {
        let eve_str = if self.is_eve { "pm" } else { "am" };
        println!(
            "current time is: {:02}:{:02} {}",
            self.hr, self.min, eve_str
        );
    }
}

struct Time {
    imp: Box<dyn TimeInterface>,
}

impl Time {
    fn new(imp: Box<dyn TimeInterface>) -> Self {
        Self { imp }
    }

    fn tell(&self) {
        self.imp.tell();
    }
}

pub fn run_bridge_logic() {
    let time = Time::new(Box::new(TimeImpl::new(23, 4)));
    let civil_time = Time::new(Box::new(CivilianTimeImpl::new(11, 4, true)));

    time.tell();
    civil_time.tell();
}
