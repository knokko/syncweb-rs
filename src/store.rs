use crate::*;

pub struct Store<M: Model> {

    model: M,

    // When the user/client of this Store reads from it
    read_tracker: PropertyStateMap<M::ID, bool>,

    // When an external entity changes the properties of the model of this Store
    did_receive_change: bool,
    on_receive_change: Box<dyn FnMut()>
}

impl<M: Model<ID = I>, I: 'static> Store<M> {

    pub fn new(initial_values: M, on_receive_change: Box<dyn FnMut()>) -> Self {
        Self {
            read_tracker: PropertyStateMap::new(initial_values.get_properties(), &false),
            model: initial_values,

            did_receive_change: false,
            on_receive_change
        }
    }

    pub fn forget_tracking_state(&mut self) {
        self.read_tracker.fill(&false);
        self.did_receive_change = false;
    }

    pub fn get<T>(&mut self, property: &TrackingProperty<M, T>) -> T {
        self.read_tracker.set_state(property, &true);
        property.get_value(&self.model)
    }

    pub fn receive_change<T>(&mut self, property: &TrackingProperty<M, T>, new_value: T) {
        if !self.did_receive_change {
            self.did_receive_change = true;
            self.on_receive_change.as_mut()();
        }
        property.set_value(&mut self.model, new_value);
    }

    // TODO Add send_change
}
