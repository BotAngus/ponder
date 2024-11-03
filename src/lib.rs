pub mod error;
pub mod parser;
pub mod span;

pub use error::*;
pub use parser::*;
pub use span::*;

pub type PResult<'src, I, O, S, E> = Result<(&'src [(I, S)], (O, S)), E>;

pub trait Input<'src>
where
    Self: Copy + PartialEq + 'src,
{
}

impl<'src, T: Copy + PartialEq + 'src> Input<'src> for T {}

pub trait SeqParser<'src, I, O, S, E>
where
    Self: Fn(&'src [(I, S)]) -> Result<(&'src [(I, S)], (O, S)), E>,
    I: Input<'src>,
    E: Error<'src, I, S>,
    S: Span<'src>,
{
    fn collect<F: FromIterator<(O, S)>>(&self) -> impl Parser<'src, I, F, S, E> {
        move |mut tokens| {
            let mut items = Vec::new();
            loop {
                match self(tokens) {
                    Ok((rest, v)) => {
                        tokens = rest;
                        items.push(v);
                    }
                    Err(_) => {
                        break Ok((tokens, {
                            let span = match (items.first(), items.last()) {
                                (Some((_, s1)), Some((_, s2))) => s1.merge(*s2),
                                _ => S::empty(),
                            };
                            (items.into_iter().collect(), span)
                        }))
                    }
                }
            }
        }
    }
    fn foldr<B, F: Fn(B, O) -> B>(
        &self,
        other: impl Parser<'src, I, B, S, E>,
        f: F,
    ) -> impl Parser<'src, I, B, S, E> {
        move |tokens| {
            let (rest, (out, _)) = self.collect::<Vec<(O, S)>>()(tokens)?;
            let (rest, single) = other(rest)?;
            Ok((
                rest,
                out.into_iter()
                    .rfold::<(B, S), _>(single, |(a1, a2), (b1, b2)| (f(a1, b1), a2.merge(b2))),
            ))
        }
    }
}

impl<'src, I, O, S, E, P> SeqParser<'src, I, O, S, E> for P
where
    P: Fn(&'src [(I, S)]) -> Result<(&'src [(I, S)], (O, S)), E>,
    I: Input<'src>,
    E: Error<'src, I, S>,
    S: Span<'src>,
{
}
pub trait Parser<'src, I, O, S, E>
where
    Self: Fn(&'src [(I, S)]) -> Result<(&'src [(I, S)], (O, S)), E>,
    I: Input<'src>,
    E: Error<'src, I, S>,
    S: Span<'src>,
{
    fn or(&self, other: impl Parser<'src, I, O, S, E>) -> impl Parser<'src, I, O, S, E> {
        move |tokens| match self(tokens) {
            Ok(v) => Ok(v),
            Err(e1) => match other(tokens) {
                Ok(v) => Ok(v),
                Err(e2) => Err(e1.merge(e2)),
            },
        }
    }

    fn then<O1>(
        &self,
        other: impl Parser<'src, I, O1, S, E>,
    ) -> impl Parser<'src, I, (O, O1), S, E> {
        move |tokens| match self(tokens) {
            Ok((rest, (parsed, span))) => match other(rest) {
                Ok((rest, (parsed_then, span_then))) => {
                    Ok((rest, ((parsed, parsed_then), span.merge(span_then))))
                }
                Err(_) => todo!(),
            },
            Err(e) => Err(e),
        }
    }

    fn delimited_by(&self, left: I, right: I) -> impl Parser<'src, I, O, S, E> {
        move |tokens| {
            just(left)
                .then(self)
                .then(just(right))
                .map(|(((), b), ())| b)(tokens)
        }
    }
    fn map<T, M: Fn(O) -> T>(&self, mapper: M) -> impl Parser<'src, I, T, S, E> {
        move |tokens| self.map_with(|o, _| mapper(o))(tokens)
    }
    fn map_with<T, M: Fn(O, S) -> T>(&self, mapper: M) -> impl Parser<'src, I, T, S, E> {
        move |tokens| self(tokens).map(|(rest, (tok, span))| (rest, (mapper(tok, span), span)))
    }
    fn repeated(&self) -> impl SeqParser<'src, I, O, S, E> {
        move |tokens| self(tokens)
    }

    fn span(&self) -> impl Parser<'src, I, S, S, E> {
        self.map_with(|_, s| s)
    }
    fn foldl<B, F: Fn(O, B) -> O>(
        &self,
        fold: impl SeqParser<'src, I, B, S, E>,
        f: F,
    ) -> impl Parser<'src, I, O, S, E> {
        move |tokens| {
            let (rest, first) = self(tokens)?;
            let (rest, (out, _)) = fold.collect::<Vec<(B, S)>>()(rest)?;
            let folded = out
                .into_iter()
                .fold(first, |(a, a_sp), (b, b_sp)| (f(a, b), a_sp.merge(b_sp)));
            Ok((rest, folded))
        }
    }

    fn infix<B, F: Fn(O, B, O) -> O>(
        &self,
        infix: impl Parser<'src, I, B, S, E>,
        f: F,
    ) -> impl Parser<'src, I, O, S, E> {
        move |tokens| self.foldl(infix.then(self), |a, (b, c)| f(a, b, c))(tokens)
    }
}

impl<'src, I, O, E, S, F> Parser<'src, I, O, S, E> for F
where
    F: Fn(&'src [(I, S)]) -> Result<(&'src [(I, S)], (O, S)), E>,
    I: Input<'src>,
    E: Error<'src, I, S>,
    S: Span<'src>,
{
}

#[cfg(test)]
mod tests {
    use crate::{Error, Input, PResult, Parser, Span};

    fn check<'src, I, O, S, E, P>(_: P)
    where
        P: Parser<'src, I, O, S, E>,
        I: Input<'src>,
        S: Span<'src>,
        E: Error<'src, I, S>,
    {
    }

    #[test]
    fn test1() {
        fn local<'src>(tokens: &'src [((), ())]) -> PResult<'src, (), (), (), ()> {
            Ok((tokens, ((), ())))
        }
        check(local.or(local).delimited_by((), ()));
    }
}
