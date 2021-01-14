use core::panic;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref SINGLETON_REF: Mutex<Option<SingletonConfig>> = Mutex::new(None);
}

#[derive(Debug)]
pub struct SingletonConfig {
    pub str_param: String,
    pub int_param: usize,
}

impl SingletonConfig {
    pub fn init(param1: String, param2: usize) {
        let mut config_ref = SINGLETON_REF.lock().unwrap();
        if config_ref.is_some() {
            panic!("SINGLETON is already initiated");
        }
        let config = SingletonConfig {
            str_param: param1,
            int_param: param2,
        };
        *config_ref = Some(config);
    }

    pub fn get() -> &'static Mutex<Option<SingletonConfig>> {
        if SINGLETON_REF.lock().unwrap().is_none() {
            panic!("SINGLETON is not initialized yet!");
        }
        &SINGLETON_REF
    }
}

pub fn run_singleton_pattern() {
    SingletonConfig::init(String::from("Some string field"), 123);

    match SingletonConfig::get().lock().unwrap().take() {
        Some(cfg) => println!("read singleton config: {:?}", cfg),
        None => panic!("config not initialized!"),
    }
}
