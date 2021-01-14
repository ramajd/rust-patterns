use uuid::Uuid;

pub trait Observer<T: Clone> {
    fn on_notify(&self, event: &T);
}

pub trait Subject<T: Clone> {
    fn notify(&self, event: &T);
    fn register(&mut self, observer: Box<dyn Observer<T>>) -> Uuid;
    fn unregister(&mut self, observer_id: Uuid);
}

#[derive(Debug, Clone)]
pub struct ObserverEvent {
    pub title: String,
}

impl ObserverEvent {
    pub fn new(title: String) -> ObserverEvent {
        ObserverEvent { title }
    }
}

pub struct ObserverImpl {
    pub title: String,
}

impl ObserverImpl {
    pub fn new(title: String) -> ObserverImpl {
        ObserverImpl { title }
    }
}

impl Observer<ObserverEvent> for ObserverImpl {
    fn on_notify(&self, event: &ObserverEvent) {
        println!("Observer Notified: id: {} - Event: {:?}", self.title, event);
    }
}

pub struct SubjectImpl {
    observers: Vec<(Uuid, Box<dyn Observer<ObserverEvent>>)>,
}

impl SubjectImpl {
    pub fn new() -> SubjectImpl {
        SubjectImpl { observers: vec![] }
    }
}

impl Subject<ObserverEvent> for SubjectImpl {
    fn notify(&self, event: &ObserverEvent) {
        for observer in &self.observers {
            observer.1.on_notify(event);
        }
    }

    fn register(&mut self, observer: Box<dyn Observer<ObserverEvent>>) -> Uuid {
        let id = Uuid::new_v4();
        self.observers.push((id, observer));
        println!("new observer registered: {}", id);
        return id;
    }

    fn unregister(&mut self, observer_id: Uuid) {
        for (index, observer) in self.observers.iter().enumerate() {
            if observer.0 == observer_id {
                self.observers.remove(index);
                println!("observer removed: {}", observer_id);
                break;
            }
        }
    }
}


pub fn run_observer_logic() {
    let mut subject = SubjectImpl::new();

    let observer1 = ObserverImpl::new(String::from("1st observer"));
    let observer2 = ObserverImpl::new(String::from("2nd observer"));
    let observer3 = ObserverImpl::new(String::from("3rd observer"));
    let observer4 = ObserverImpl::new(String::from("4th observer"));

    let oid1 = subject.register(Box::new(observer1));
    let oid2 = subject.register(Box::new(observer2));
    let oid3 = subject.register(Box::new(observer3));
    let oid4 = subject.register(Box::new(observer4));

    subject.notify(&ObserverEvent::new(String::from("1st event")));
    subject.unregister(oid1);

    subject.notify(&ObserverEvent::new(String::from("2nd event")));
    subject.unregister(oid2);

    subject.notify(&ObserverEvent::new(String::from("3rd event")));
    subject.unregister(oid3);

    subject.notify(&ObserverEvent::new(String::from("4th event")));
    subject.unregister(oid4);
    
    subject.notify(&ObserverEvent::new(String::from("5th event")));
}