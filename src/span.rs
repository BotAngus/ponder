pub trait Span<'src>
where
    Self: Copy + 'src,
{
    fn merge(self, other: Self) -> Self;
}

impl<'src> Span<'src> for () {
    fn merge(self, _: Self) -> Self {
        ()
    }
}
