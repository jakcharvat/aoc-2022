pub trait Itertools<T> {
    fn iter_diagonal<'a>(&'a self) -> impl Iterator<Item = &'a T>
    where
        T: 'a;

    fn iter_diagonal_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T>
    where
        T: 'a;

    fn into_iter_diagonal(self) -> impl Iterator<Item = T>;
}

impl<T> Itertools<T> for Vec<Vec<T>> {
    fn iter_diagonal<'a>(&'a self) -> impl Iterator<Item = &'a T>
    where
        T: 'a,
    {
        self.iter().enumerate().map(|(idx, line)| &line[idx])
    }

    fn iter_diagonal_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T>
    where
        T: 'a,
    {
        self.iter_mut()
            .enumerate()
            .map(|(idx, line)| &mut line[idx])
    }

    fn into_iter_diagonal(self) -> impl Iterator<Item = T> {
        self.into_iter()
            .enumerate()
            .map(|(idx, mut line)| line.swap_remove(idx))
    }
}
