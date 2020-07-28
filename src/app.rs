pub trait App: 'static + Sized {
    fn title() -> &'static str;
    fn new() -> Self;
}