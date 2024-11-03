use crate::{span::Span, Input};

pub trait Error<'src, I, S>
where
    Self: 'src,
    S: Span<'src>,
    I: Input<'src>,
{
    fn merge(self, other: Self) -> Self;
    fn eof<Iter: IntoIterator<Item = I>>(expected: Iter) -> Self;
    fn unexpected<Iter: IntoIterator<Item = I>>(expected: Iter, found: I, span: S) -> Self;
}

impl<'src, I, S> Error<'src, I, S> for ()
where
    I: Input<'src>,
    S: Span<'src>,
{
    fn merge(self, _: Self) -> Self {
        ()
    }

    fn eof<Iter: IntoIterator<Item = I>>(_: Iter) -> Self {
        ()
    }

    fn unexpected<Iter: IntoIterator<Item = I>>(_: Iter, _: I, _: S) -> Self {
        ()
    }
}
