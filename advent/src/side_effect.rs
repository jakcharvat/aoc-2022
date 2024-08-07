pub trait SideEffecting {
    fn side_effect<T, F>(self, mut f: F) -> impl Iterator<Item = T>
    where
        Self: Sized + Iterator<Item = T>,
        F: FnMut(&T),
    {
        self.map(move |el| {
            f(&el);
            el
        })
    }
}

impl<I> SideEffecting for I where I: Iterator {}
