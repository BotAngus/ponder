use crate::{error::Error, span::Span, Input, Parser};

pub fn just<'src, I, O, S, E: Error<I>>(expect: I, give: O) -> impl Parser<'src, I, O, S, E>
where
    S: Span<'src>,
    O: Clone,
    I: Input<'src>,
    E: Error<I, Span<'src> = S>,
{
    move |tokens| match tokens {
        [(tok, span), tail @ ..] if *tok == expect => Ok((tail, (give.clone(), *span))),
        [(tok, span), ..] => Err(E::unexpected(Some(*tok), *tok, *span)),
        [] => Err(E::eof(Some(expect))),
    }
}
