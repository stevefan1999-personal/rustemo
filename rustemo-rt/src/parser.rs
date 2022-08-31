use crate::{
    builder::Builder,
    error::Result,
    lexer::{Context, Lexer},
};

pub trait Parser<I, L, B, LO, ST>
where
    L: Lexer<I, LO, ST>,
    B: Builder,
{
    fn parse(
        &mut self,
        context: &mut Context<I, LO, ST>,
        lexer: &L,
        builder: &mut B,
    ) -> Result<B::Output>;
}
