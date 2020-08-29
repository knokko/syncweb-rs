use crate::*;

pub trait Store {

    type ModelType;

    fn get_model(&mut self) -> &mut Self::ModelType;

    fn get_tracker(&mut self) -> &mut ReadTracker<Self::ModelType>;

    fn forget_previous_gets(&mut self);
}

impl<S: Store, T> GenericStore<T> for S {}

pub trait GenericStore<T>: Store {

    fn get(&mut self, property: &TrackingProperty<Self::ModelType, T>) -> T {
        self.get_tracker().read_property(property);
        property.get_value(self.get_model())
    }

    fn set(&mut self, property: &TrackingProperty<Self::ModelType, T>, new_value: T) {
        property.set_value(self.get_model(), new_value);
    }
}