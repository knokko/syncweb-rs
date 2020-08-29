use crate::*;

use std::marker::PhantomData;

pub struct ReadTracker<I> {

    read_properties: Vec<bool>,
    phantom: PhantomData<I>
}

impl<I: 'static> ReadTracker<I> {

    pub fn new(set: &PropertySet<I>) -> Self {
        ReadTracker {
            read_properties: vec![false; set.amount as usize],
            phantom: PhantomData
        }
    }

    pub fn read_property<T, M: Model<ID = I>>(&mut self, property: &TrackingProperty<M, T>) {
        self.read_properties[property.index as usize] = true;
    }

    pub fn was_property_read<T, M: Model<ID = I>>(&self, property: &TrackingProperty<M, T>) -> bool {
        self.read_properties[property.index as usize]
    }

    pub fn forget_read_properties(&mut self) {
        for index in 0..self.read_properties.len() {
            self.read_properties[index] = false;
        }
    }
}

type Getter<M, T> = &'static dyn Fn(&M) -> T;
type Setter<M, T> = &'static dyn Fn(&mut M, T);

pub struct TrackingProperty<M: 'static, T: 'static> {

    index: u16,

    get: Getter<M, T>,
    set: Setter<M, T>,

    phantom: PhantomData<M>
}

impl<M: Model, T> TrackingProperty<M, T> {

    const fn new(index: u16, get: Getter<M, T>, set: Setter<M, T>) -> Self {
        Self {
            index,
            get,
            set,
            phantom: PhantomData
        }
    }

    pub const fn first(getter: Getter<M, T>, setter: Setter<M, T>) -> Self {
        Self::new(0, getter, setter)        
    }

    pub const fn next(&self, getter: Getter<M, T>, setter: Setter<M, T>) -> TrackingProperty<M, T> {
        TrackingProperty::new(self.index + 1, getter, setter)
    }

    pub const fn finish_set(&self) -> PropertySet<M::ID> {
        PropertySet {
            amount: self.index + 1,
            phanton: PhantomData
        }
    }

    pub fn get_value(&self, target: &M) -> T {
        self.get.call((target,))
    }

    pub fn set_value(&self, target: &mut M, new_value: T) {
        self.set.call((target, new_value))
    }
}

pub struct PropertySet<M> {

    amount: u16,
    phanton: PhantomData<M>
}
