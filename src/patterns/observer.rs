use uuid::Uuid;

pub trait Observer<T: Clone> {
    fn on_notify(&self, event: &T);
}

pub trait Subject<T: Clone> {
    fn notify(&self, event: &T);
    fn register(&mut self, observer: Box<dyn Observer<T>>) -> Uuid;
    fn unregister(&mut self, observer_id: Uuid);
}
