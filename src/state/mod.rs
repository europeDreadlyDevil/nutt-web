pub struct State<T> {
    value: T,
}

impl<T> State<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}