trait Component {
    fn do_something(&self);
}

trait Decorator: Component {
    fn do_somthing_more(&self);
}

struct BaseObject {
    value: usize,
}

impl Component for BaseObject {
    fn do_something(&self) {
        println!("component value is: {}", self.value);
    }
}

struct DecoratorObject {
    base: BaseObject,
    more_value: usize,
}

impl Component for DecoratorObject {
    fn do_something(&self) {
        self.base.do_something();
    }
}

impl Decorator for DecoratorObject {
    fn do_somthing_more(&self) {
        println!(
            "decorator added to value: {} + {} = {}",
            self.base.value,
            self.more_value,
            self.base.value + self.more_value
        )
    }
}

fn process(c: &dyn Component) {
    c.do_something();
}

pub fn run_decorator_logic() {
    let obj = BaseObject { value: 1 };
    process(&obj);

    let decorated = DecoratorObject {
        base: obj,
        more_value: 2,
    };
    process(&decorated);

    decorated.do_somthing_more();
}
