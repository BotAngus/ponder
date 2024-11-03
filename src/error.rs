use crate::span::Span;

pub trait Error<I> {
    type Span<'src>: Span<'src>;
    fn merge(self, other: Self) -> Self;
    fn eof<Iter: IntoIterator<Item = I>>(expected: Iter) -> Self;
    fn unexpected<'src, Iter: IntoIterator<Item = I>>(
        expected: Iter,
        found: I,
        span: Self::Span<'src>,
    ) -> Self;
}
