/// Generated by rustemo. Do not edit manually!
use std::fmt::Debug;
use std::hash::Hash;
use rustemo::{
    Result, Input as InputT, Lexer, Token, TokenRecognizer as TokenRecognizerT, Parser,
    ParserDefinition, State as StateT, Builder,
};
use regex::Regex;
use once_cell::sync::Lazy;
use rustemo::StringLexer;
use rustemo::LRBuilder;
use super::output_dir_actions;
use rustemo::{LRParser, LRContext};
use rustemo::Action::{self, Shift, Reduce, Accept};
#[allow(unused_imports)]
use rustemo::debug::{log, logn};
#[allow(unused_imports)]
#[cfg(debug_assertions)]
use colored::*;
pub type Input = str;
use rustemo::Action::Error;
const TERMINAL_COUNT: usize = 3usize;
const NONTERMINAL_COUNT: usize = 5usize;
const STATE_COUNT: usize = 7usize;
#[allow(dead_code)]
const MAX_ACTIONS: usize = 1usize;
const MAX_RECOGNIZERS: usize = 2usize;
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TokenKind {
    #[default]
    STOP,
    Tb,
    Num,
}
use TokenKind as TK;
impl From<TokenKind> for usize {
    fn from(t: TokenKind) -> Self {
        t as usize
    }
}
#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, PartialEq)]
pub enum ProdKind {
    AP1,
    B1P1,
    B1P2,
    BP1,
}
use ProdKind as PK;
impl std::fmt::Debug for ProdKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            ProdKind::AP1 => "A: B1 Num",
            ProdKind::B1P1 => "B1: B1 B",
            ProdKind::B1P2 => "B1: B",
            ProdKind::BP1 => "B: Tb",
        };
        write!(f, "{}", name)
    }
}
#[allow(clippy::upper_case_acronyms)]
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum NonTermKind {
    EMPTY,
    AUG,
    A,
    B1,
    B,
}
impl From<ProdKind> for NonTermKind {
    fn from(prod: ProdKind) -> Self {
        match prod {
            ProdKind::AP1 => NonTermKind::A,
            ProdKind::B1P1 => NonTermKind::B1,
            ProdKind::B1P2 => NonTermKind::B1,
            ProdKind::BP1 => NonTermKind::B,
        }
    }
}
#[allow(clippy::enum_variant_names)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum State {
    #[default]
    AUGS0,
    TbS1,
    AS2,
    B1S3,
    BS4,
    NumS5,
    BS6,
}
impl StateT for State {
    fn default_layout() -> Option<Self> {
        None
    }
}
impl From<State> for usize {
    fn from(s: State) -> Self {
        s as usize
    }
}
impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            State::AUGS0 => "0:AUG",
            State::TbS1 => "1:Tb",
            State::AS2 => "2:A",
            State::B1S3 => "3:B1",
            State::BS4 => "4:B",
            State::NumS5 => "5:Num",
            State::BS6 => "6:B",
        };
        write!(f, "{name}")
    }
}
#[derive(Debug)]
pub enum Symbol {
    Terminal(Terminal),
    NonTerminal(NonTerminal),
}
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub enum Terminal {
    Tb,
    Num(output_dir_actions::Num),
}
#[derive(Debug)]
pub enum NonTerminal {
    A(output_dir_actions::A),
    B1(output_dir_actions::B1),
    B(output_dir_actions::B),
}
pub struct OutputDirParserDefinition {
    actions: [[[Action<State, ProdKind>; MAX_ACTIONS]; TERMINAL_COUNT]; STATE_COUNT],
    gotos: [[Option<State>; NONTERMINAL_COUNT]; STATE_COUNT],
    token_kinds: [[Option<(TokenKind, bool)>; MAX_RECOGNIZERS]; STATE_COUNT],
}
pub(crate) static PARSER_DEFINITION: OutputDirParserDefinition = OutputDirParserDefinition {
    actions: [
        [[Error], [Shift(State::TbS1)], [Error]],
        [[Error], [Reduce(PK::BP1, 1usize)], [Reduce(PK::BP1, 1usize)]],
        [[Accept], [Error], [Error]],
        [[Error], [Shift(State::TbS1)], [Shift(State::NumS5)]],
        [[Error], [Reduce(PK::B1P2, 1usize)], [Reduce(PK::B1P2, 1usize)]],
        [[Reduce(PK::AP1, 2usize)], [Error], [Error]],
        [[Error], [Reduce(PK::B1P1, 2usize)], [Reduce(PK::B1P1, 2usize)]],
    ],
    gotos: [
        [None, None, Some(State::AS2), Some(State::B1S3), Some(State::BS4)],
        [None, None, None, None, None],
        [None, None, None, None, None],
        [None, None, None, None, Some(State::BS6)],
        [None, None, None, None, None],
        [None, None, None, None, None],
        [None, None, None, None, None],
    ],
    token_kinds: [
        [Some((TK::Tb, true)), None],
        [Some((TK::Tb, true)), Some((TK::Num, false))],
        [Some((TK::STOP, false)), None],
        [Some((TK::Tb, true)), Some((TK::Num, false))],
        [Some((TK::Tb, true)), Some((TK::Num, false))],
        [Some((TK::STOP, false)), None],
        [Some((TK::Tb, true)), Some((TK::Num, false))],
    ],
};
impl ParserDefinition<State, ProdKind, TokenKind, NonTermKind>
for OutputDirParserDefinition {
    fn actions(&self, state: State, token: TokenKind) -> Vec<Action<State, ProdKind>> {
        PARSER_DEFINITION
            .actions[state as usize][token as usize]
            .iter()
            .copied()
            .take_while(|a| !matches!(a, Action::Error))
            .collect()
    }
    fn goto(&self, state: State, nonterm: NonTermKind) -> State {
        PARSER_DEFINITION.gotos[state as usize][nonterm as usize].unwrap()
    }
    fn expected_token_kinds(&self, state: State) -> Vec<(TokenKind, bool)> {
        PARSER_DEFINITION.token_kinds[state as usize].iter().map_while(|t| *t).collect()
    }
    fn longest_match() -> bool {
        true
    }
    fn grammar_order() -> bool {
        true
    }
}
pub(crate) type Context<'i, I> = LRContext<'i, I, State, TokenKind>;
pub struct OutputDirParser<
    'i,
    I: InputT + ?Sized,
    L: Lexer<'i, Context<'i, I>, State, TokenKind, Input = I>,
    B,
>(
    LRParser<
        'i,
        Context<'i, I>,
        State,
        ProdKind,
        TokenKind,
        NonTermKind,
        OutputDirParserDefinition,
        L,
        B,
        I,
    >,
);
#[allow(dead_code)]
impl<
    'i,
> OutputDirParser<
    'i,
    Input,
    StringLexer<Context<'i, Input>, State, TokenKind, TokenRecognizer, TERMINAL_COUNT>,
    DefaultBuilder,
> {
    pub fn new() -> Self {
        Self(
            LRParser::new(
                &PARSER_DEFINITION,
                State::default(),
                false,
                false,
                StringLexer::new(true, &RECOGNIZERS),
                DefaultBuilder::new(),
            ),
        )
    }
}
#[allow(dead_code)]
impl<'i, I, L, B> Parser<'i, I, Context<'i, I>, State, TokenKind>
for OutputDirParser<'i, I, L, B>
where
    I: InputT + ?Sized + Debug,
    L: Lexer<'i, Context<'i, I>, State, TokenKind, Input = I>,
    B: LRBuilder<'i, I, Context<'i, I>, State, ProdKind, TokenKind>,
{
    type Output = B::Output;
    fn parse(&self, input: &'i I) -> Result<Self::Output> {
        self.0.parse(input)
    }
    fn parse_with_context(
        &self,
        context: &mut Context<'i, I>,
        input: &'i I,
    ) -> Result<Self::Output> {
        self.0.parse_with_context(context, input)
    }
    fn parse_file<'a, F: AsRef<std::path::Path>>(
        &'a mut self,
        file: F,
    ) -> Result<Self::Output>
    where
        'a: 'i,
    {
        self.0.parse_file(file)
    }
}
#[allow(dead_code)]
#[derive(Debug)]
pub enum Recognizer {
    Stop,
    StrMatch(&'static str),
    RegexMatch(Lazy<Regex>),
}
#[allow(dead_code)]
#[derive(Debug)]
pub struct TokenRecognizer(TokenKind, Recognizer);
impl<'i> TokenRecognizerT<'i> for TokenRecognizer {
    fn recognize(&self, input: &'i str) -> Option<&'i str> {
        match &self {
            #[allow(unused_variables)]
            TokenRecognizer(token_kind, Recognizer::StrMatch(s)) => {
                logn!("{} {:?} -- ", "    Recognizing".green(), token_kind);
                if input.starts_with(s) {
                    log!("{}", "recognized".bold().green());
                    Some(s)
                } else {
                    log!("{}", "not recognized".red());
                    None
                }
            }
            #[allow(unused_variables)]
            TokenRecognizer(token_kind, Recognizer::RegexMatch(r)) => {
                logn!("{} {:?} -- ", "    Recognizing".green(), token_kind);
                let match_str = r.find(input);
                match match_str {
                    Some(x) => {
                        let x_str = x.as_str();
                        log!("{} '{}'", "recognized".bold().green(), x_str);
                        Some(x_str)
                    }
                    None => {
                        log!("{}", "not recognized".red());
                        None
                    }
                }
            }
            TokenRecognizer(_, Recognizer::Stop) => {
                logn!("{} STOP -- ", "    Recognizing".green());
                if input.is_empty() {
                    log!("{}", "recognized".bold().green());
                    Some("")
                } else {
                    log!("{}", "not recognized".red());
                    None
                }
            }
        }
    }
}
pub(crate) static RECOGNIZERS: [TokenRecognizer; TERMINAL_COUNT] = [
    TokenRecognizer(TokenKind::STOP, Recognizer::Stop),
    TokenRecognizer(TokenKind::Tb, Recognizer::StrMatch("b")),
    TokenRecognizer(
        TokenKind::Num,
        Recognizer::RegexMatch(
            Lazy::new(|| { Regex::new(concat!("^", "\\d+")).unwrap() }),
        ),
    ),
];
pub struct DefaultBuilder {
    res_stack: Vec<Symbol>,
}
impl DefaultBuilder {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { res_stack: vec![] }
    }
}
impl Builder for DefaultBuilder {
    type Output = output_dir_actions::A;
    fn get_result(&mut self) -> Self::Output {
        match self.res_stack.pop().unwrap() {
            Symbol::NonTerminal(NonTerminal::A(r)) => r,
            _ => panic!("Invalid result on the parse stack!"),
        }
    }
}
impl<'i> LRBuilder<'i, Input, Context<'i, Input>, State, ProdKind, TokenKind>
for DefaultBuilder {
    #![allow(unused_variables)]
    fn shift_action(
        &mut self,
        context: &mut Context<'i, Input>,
        token: Token<'i, Input, TokenKind>,
    ) {
        let val = match token.kind {
            TokenKind::STOP => panic!("Cannot shift STOP token!"),
            TokenKind::Tb => Terminal::Tb,
            TokenKind::Num => Terminal::Num(output_dir_actions::num(&*context, token)),
        };
        self.res_stack.push(Symbol::Terminal(val));
    }
    fn reduce_action(
        &mut self,
        context: &mut Context<'i, Input>,
        prod: ProdKind,
        _prod_len: usize,
    ) {
        let prod = match prod {
            ProdKind::AP1 => {
                let mut i = self
                    .res_stack
                    .split_off(self.res_stack.len() - 2usize)
                    .into_iter();
                match (i.next().unwrap(), i.next().unwrap()) {
                    (
                        Symbol::NonTerminal(NonTerminal::B1(p0)),
                        Symbol::Terminal(Terminal::Num(p1)),
                    ) => NonTerminal::A(output_dir_actions::a_c1(&*context, p0, p1)),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::B1P1 => {
                let mut i = self
                    .res_stack
                    .split_off(self.res_stack.len() - 2usize)
                    .into_iter();
                match (i.next().unwrap(), i.next().unwrap()) {
                    (
                        Symbol::NonTerminal(NonTerminal::B1(p0)),
                        Symbol::NonTerminal(NonTerminal::B(p1)),
                    ) => NonTerminal::B1(output_dir_actions::b1_c1(&*context, p0, p1)),
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::B1P2 => {
                let mut i = self
                    .res_stack
                    .split_off(self.res_stack.len() - 1usize)
                    .into_iter();
                match i.next().unwrap() {
                    Symbol::NonTerminal(NonTerminal::B(p0)) => {
                        NonTerminal::B1(output_dir_actions::b1_b(&*context, p0))
                    }
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
            ProdKind::BP1 => {
                let _ = self
                    .res_stack
                    .split_off(self.res_stack.len() - 1usize)
                    .into_iter();
                NonTerminal::B(output_dir_actions::b_tb(context))
            }
        };
        self.res_stack.push(Symbol::NonTerminal(prod));
    }
}
