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

pub trait Parser<'src, I, O, S, E>
where
    Self: Fn(&'src [(I, S)]) -> Result<(&'src [(I, S)], (O, S)), E>,
    I: Input<'src>,
    E: Error<'src, I, Span = S>,
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
            just(left, ())
                .then(self)
                .then(just(right, ()))
                .map(|(((), b), ())| b)(tokens)
        }
    }
    fn map<T, M: Fn(O) -> T>(&self, mapper: M) -> impl Parser<'src, I, T, S, E> {
        move |tokens| self(tokens).map(|(rest, (tok, span))| (rest, (mapper(tok), span)))
    }
}

impl<'src, I, O, E, S, F> Parser<'src, I, O, S, E> for F
where
    F: Fn(&'src [(I, S)]) -> Result<(&'src [(I, S)], (O, S)), E>,
    I: Input<'src>,
    E: Error<'src, I, Span = S>,
    S: Span<'src>,
{
}
