use crate::span::Span;

pub trait Error<'src, I>: 'src {
    type Span: Span<'src>;
    fn merge(self, other: Self) -> Self;
    fn eof<Iter: IntoIterator<Item = I>>(expected: Iter) -> Self;
    fn unexpected<Iter: IntoIterator<Item = I>>(expected: Iter, found: I, span: Self::Span)
        -> Self;
}
