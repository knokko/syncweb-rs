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
    }

    impl ExampleStore for Store<ExampleModel> {

        fn get_foo(&mut self) -> u32 {
            self.get(&PROPERTY_FOO)
        }

        fn get_bar(&mut self) -> u32 {
            self.get(&PROPERTY_BAR)
        }
    }

    #[test]
    fn test_very_simple() {

        let mut store = Store::new(ExampleModel { foo: 12, bar: 20});
        assert_eq!(12, store.get_foo());
        assert_eq!(20, store.get_bar());
    }
}
