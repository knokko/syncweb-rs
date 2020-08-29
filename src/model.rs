use crate::*;

pub trait Model {

    type ID: 'static;

    fn get_properties(&self) -> &'static PropertySet<Self::ID>;
}