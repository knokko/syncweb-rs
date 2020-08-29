use crate::*;

pub struct Store<M: Model> {

    model: M,
    tracker: ReadTracker<M::ID>
}

impl<M: Model<ID = I>, I: 'static> Store<M> {

    pub fn new(initial_values: M) -> Self {
        Self {
            tracker: ReadTracker::new(initial_values.get_properties()),
            model: initial_values,
        }
    }

    pub fn forget_previous_gets(&mut self) {
        self.tracker.forget_read_properties();
    }

    pub fn get<T>(&mut self, property: &TrackingProperty<M, T>) -> T {
        self.tracker.read_property(property);
        property.get_value(&self.model)
    }

    // TODO Think about how to support syncing of set operations
    pub fn set<T>(&mut self, property: &TrackingProperty<M, T>, new_value: T) {
        property.set_value(&mut self.model, new_value);
    }
}
