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
    
    struct ExampleModel {

        foo: u32,
        bar: u32
    }

    impl Model for ExampleModel {

    }

    const PROPERTY_FOO: TrackingProperty<ExampleModel, u32> = TrackingProperty::first(
        &|example: &ExampleModel| example.foo,
        &|example: &mut ExampleModel, new_foo| example.foo = new_foo
    );
    const PROPERTY_BAR: TrackingProperty<ExampleModel, u32> = PROPERTY_FOO.next(
        &|example: &ExampleModel| example.bar,
        &|example: &mut ExampleModel, new_bar| example.bar = new_bar
    );
    const PROPERTIES_EXAMPLE: PropertySet<ExampleModel> = PROPERTY_BAR.finish_set();

    trait ExampleStore : Store {

        fn get_foo(&mut self) -> u32;

        fn get_bar(&mut self) -> u32;
    }

    struct DirectExampleStore {

        model: ExampleModel,
        tracker: ReadTracker<ExampleModel>,
    }

    impl DirectExampleStore {

        pub fn new(foo: u32, bar: u32) -> Self {
            Self {
                model: ExampleModel { foo, bar },
                tracker: ReadTracker::new(&PROPERTIES_EXAMPLE)
            }
        }
    }

    impl Store for DirectExampleStore {

        type ModelType = ExampleModel;

        fn forget_previous_gets(&mut self) {
            self.tracker.forget_read_properties();
        }

        fn get_model(&mut self) -> &mut ExampleModel {
            &mut self.model
        }

        fn get_tracker(&mut self) -> &mut ReadTracker<ExampleModel> {
            &mut self.tracker
        }
    }

    impl ExampleStore for DirectExampleStore {

        fn get_foo(&mut self) -> u32 {
            self.get(&PROPERTY_FOO)
        }

        fn get_bar(&mut self) -> u32 {
            self.get(&PROPERTY_BAR)
        }
    }

    #[test]
    fn test_very_simple() {

        let mut store = DirectExampleStore::new(12, 20);
        assert_eq!(12, store.get_foo());
        assert_eq!(20, store.get_bar());
    }
}
