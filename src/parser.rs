use crate::{error::Error, span::Span, Input, Parser};

pub fn just<'src, I, S, E>(expect: I) -> impl Parser<'src, I, (), S, E>
where
    S: Span<'src>,
    I: Input<'src>,
    E: Error<'src, I, S>,
{
    move |tokens| match tokens {
        [(tok, span), tail @ ..] if *tok == expect => Ok((tail, ((), *span))),
        [(tok, span), ..] => Err(E::unexpected(Some(*tok), *tok, *span)),
        [] => Err(E::eof(Some(expect))),
    }
}
