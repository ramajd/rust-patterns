struct Director {
    builder: Box<dyn Builder>,
}

impl Director {
    pub fn new(builder: Box<dyn Builder>) -> Director {
        Director { builder }
    }

    fn construct(&mut self) {
        self.builder.make_title("Welcome".to_string());
        self.builder.make_string("builder pattern test".to_string());
        self.builder
            .make_items(vec!["first_item".to_string(), "second item".to_string()]);
        self.builder.make_string("some another string".to_string());
        self.builder
            .make_items(vec!["foo".to_string(), "bar".to_string()]);
        self.builder.close();
    }
}

pub trait Builder {
    fn make_title(&mut self, title: String);
    fn make_string(&mut self, string: String);
    fn make_items(&mut self, items: Vec<String>);
    fn close(&mut self);
    fn get_result(&self) -> String;
}

struct MarkdownBuilder {
    buffer: String,
}

impl MarkdownBuilder {
    fn new() -> MarkdownBuilder {
        MarkdownBuilder {
            buffer: String::new(),
        }
    }
}

impl Builder for MarkdownBuilder {
    fn make_title(&mut self, title: String) {
        self.buffer.push_str(&format!("# {}\n", title));
    }

    fn make_string(&mut self, string: String) {
        self.buffer.push_str(&format!("{}\n", string));
    }

    fn make_items(&mut self, items: Vec<String>) {
        self.buffer.push_str("\n");
        for item in &items {
            self.buffer.push_str(&format!(" - {}\n", item));
        }
        self.buffer.push_str("\n");
    }

    fn close(&mut self) {
        self.buffer.push_str("\n");
    }

    fn get_result(&self) -> String {
        self.buffer.clone()
    }
}

pub fn run_builder_pattern() {
    let md_builder = Box::new(MarkdownBuilder::new());
    let mut md_director = Director::new(md_builder);
    md_director.construct();
    let result = md_director.builder.get_result();
    println!("{}", result);
}
