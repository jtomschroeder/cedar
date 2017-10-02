
pub enum Pair<T, U> {
    Left(T),
    Both(T, U),
    Right(U),
}

pub struct Zip<I, J> {
    i: I,
    j: J,
}

impl<I, J> Iterator for Zip<I, J>
where
    I: Iterator,
    J: Iterator,
{
    type Item = Pair<I::Item, J::Item>;
    fn next(&mut self) -> Option<Self::Item> {
        match (self.i.next(), self.j.next()) {
            (Some(i), Some(j)) => Some(Pair::Both(i, j)),
            (Some(i), _) => Some(Pair::Left(i)),
            (_, Some(j)) => Some(Pair::Right(j)),
            _ => None,
        }
    }
}

pub fn zip<I, J>(i: I, j: J) -> Zip<I::IntoIter, J::IntoIter>
where
    I: IntoIterator,
    J: IntoIterator,
{
    Zip {
        i: i.into_iter(),
        j: j.into_iter(),
    }
}
