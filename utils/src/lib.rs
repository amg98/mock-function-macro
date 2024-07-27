pub struct MockDisposeBag {
    mocks: Vec<Box<dyn FnMut()>>,
}

impl MockDisposeBag {
    pub fn new() -> Self {
        Self { mocks: Vec::new() }
    }

    pub fn add<T: FnMut() + 'static>(&mut self, mock: T) {
        self.mocks.push(Box::new(mock));
    }
}

impl Drop for MockDisposeBag {
    fn drop(&mut self) {
        for mut mock in self.mocks.drain(..) {
            mock();
        }
    }
}
