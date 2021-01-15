trait Document {
    fn open(&mut self);
    fn close(&mut self);

    fn get_name(&self) -> String;
}

struct MyDocument {
    name: String,
}

impl MyDocument {
    fn new(name: String) -> MyDocument {
        MyDocument { name }
    }
}

impl Document for MyDocument {
    fn open(&mut self) {
        println!("   MyDocument: Open()");
    }

    fn close(&mut self) {
        println!("   MyDocument: Close()");
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}

trait Application<T: Document> {
    fn new_document(&mut self, name: String);
    fn report_docs(&self);
}

struct MyApplication {
    docs: Vec<MyDocument>,
}

impl MyApplication {
    fn new() -> Self {
        Self { docs: vec![] }
    }
}

impl Application<MyDocument> for MyApplication {
    fn new_document(&mut self, name: String) {
        let mut new_doc = MyDocument::new(name);
        new_doc.open();
        self.docs.push(new_doc);
    }

    fn report_docs(&self) {
        println!("MyApplication: report docs:");
        for doc in &self.docs {
            println!("   - {}", doc.get_name());
        }
    }
}

pub fn run_factory_logic() {
    let mut my_app = MyApplication::new();

    my_app.new_document("foo".to_string());
    my_app.new_document("bar".to_string());
    my_app.report_docs();
}
