use crate::*;

use std::marker::PhantomData;

type Getter<M, T> = &'static dyn Fn(&M) -> T;
type Setter<M, T> = &'static dyn Fn(&mut M, T);

pub struct TrackingProperty<M: 'static, T: 'static> {
    index: u16,

    get: Getter<M, T>,
    set: Setter<M, T>,

    phantom: PhantomData<M>,
}

impl<M: Model, T> TrackingProperty<M, T> {
    const fn new(index: u16, get: Getter<M, T>, set: Setter<M, T>) -> Self {
        Self {
            index,
            get,
            set,
            phantom: PhantomData,
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
            phanton: PhantomData,
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
    phanton: PhantomData<M>,
}

pub struct PropertyStateMap<I, V> {
    states: Vec<V>,
    phantom: PhantomData<I>,
}

impl<I: 'static, V: Clone> PropertyStateMap<I, V> {
    pub fn new(set: &PropertySet<I>, default_state: &V) -> Self {
        Self {
            states: vec![default_state.clone(); set.amount as usize],
            phantom: PhantomData,
        }
    }

    pub fn set_state<T, M: Model<ID = I>>(
        &mut self,
        property: &TrackingProperty<M, T>,
        new_state: &V,
    ) {
        self.states[property.index as usize] = new_state.clone();
    }

    pub fn get_state<T, M: Model<ID = I>>(&self, property: &TrackingProperty<M, T>) -> V {
        self.states[property.index as usize].clone()
    }

    pub fn fill(&mut self, value: &V) {
        for index in 0..self.states.len() {
            self.states[index] = value.clone();
        }
    }
}
