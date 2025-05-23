

pub trait MultiPeekCheckExt<'a, T: 'a>
where
    Self: Iterator<Item = &'a T> + Sized,
{
    fn multi_peek_check(&mut self, pattern: &[T]) -> bool
    where
        T: PartialEq;
}

impl<'a, I, T> MultiPeekCheckExt<'a, T> for I
where
    I: Iterator<Item = &'a T> + Clone, //  Clone to restore after peek
    T: PartialEq + 'a,
{
    fn multi_peek_check(&mut self, pattern: &[T]) -> bool {
        let mut clone = self.clone();

        for expected in pattern {
            match clone.next() {
                Some(actual) if actual == expected => continue,
                _ => return false,
            }
        }

        true
    }
}