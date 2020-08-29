#![feature(const_fn, fn_traits)]

mod store;
mod model;
mod tracking;

pub use store::*;
pub use model::*;
pub use tracking::*;

#[cfg(test)]
mod tests {

    use crate::*;

    use std::cell::Cell;
    use std::rc::Rc;
    
    struct ExampleModel {

        foo: u32,
        bar: u32
    }

    struct ExampleID {}

    impl Model for ExampleModel {

        type ID = ExampleID;

        fn get_properties(&self) -> &'static PropertySet<ExampleID> {
            &PROPERTIES_EXAMPLE
        }
    }

    const PROPERTY_FOO: TrackingProperty<ExampleModel, u32> = TrackingProperty::first(
        &|example: &ExampleModel| example.foo,
        &|example: &mut ExampleModel, new_foo| example.foo = new_foo
    );
    const PROPERTY_BAR: TrackingProperty<ExampleModel, u32> = PROPERTY_FOO.next(
        &|example: &ExampleModel| example.bar,
        &|example: &mut ExampleModel, new_bar| example.bar = new_bar
    );
    const PROPERTIES_EXAMPLE: PropertySet<ExampleID> = PROPERTY_BAR.finish_set();

    trait ExampleStore {

        fn get_foo(&mut self) -> u32;

        fn get_bar(&mut self) -> u32;

        fn receive_foo(&mut self, new_foo: u32);

        fn receive_bar(&mut self, new_bar: u32);
    }

    impl ExampleStore for Store<ExampleModel> {

        fn get_foo(&mut self) -> u32 {
            self.get(&PROPERTY_FOO)
        }

        fn get_bar(&mut self) -> u32 {
            self.get(&PROPERTY_BAR)
        }

        fn receive_foo(&mut self, new_foo: u32) {
            self.receive_change(&PROPERTY_FOO, new_foo);
        }

        fn receive_bar(&mut self, new_bar: u32) {
            self.receive_change(&PROPERTY_BAR, new_bar);
        }
    }

    #[test]
    fn test_receive() {

        let count_cell = Rc::new(Cell::new(0));
        let ref_count_cell = Rc::clone(&count_cell);

        let mut store = Store::new(
            ExampleModel { foo: 12, bar: 20},
            Box::new(move || ref_count_cell.set(ref_count_cell.get() + 1))
        );

        // Initially, the values should be as received in the constructor
        assert_eq!(12, store.get_foo());
        assert_eq!(20, store.get_bar());
        // And the on_change function shouldn't have been called yet
        assert_eq!(0, count_cell.get());

        // We now change the value of `foo` from the 'outside'
        store.receive_foo(6);
        // This should cause `get_foo` to return the new value 
        assert_eq!(6, store.get_foo());
        // And the on_change function should have been called once
        assert_eq!(1, count_cell.get());

        // We now change the value of `bar` as well
        store.receive_bar(10);
        // This should cause `get_bar` to return the new value
        assert_eq!(10, store.get_bar());
        // But the on_change function should not be called again
        assert_eq!(1, count_cell.get());

        // Forget the tracking state of the store, so that we can listen again
        store.forget_tracking_state();
        // This shouldn't cause the on_change function to be called (yet)
        assert_eq!(1, count_cell.get());
        // But it should be called as soon as we change foo or bar again
        store.receive_bar(8);
        assert_eq!(2, count_cell.get());
        // And obviously, the value returned by get_bar should also change
        assert_eq!(8, store.get_bar());
    }
}
