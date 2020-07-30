use crate::Ember;

pub trait App: 'static + Sized {
    fn new(ember: Ember) -> Self;
}