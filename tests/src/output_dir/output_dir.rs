/// Generated by rustemo. Do not edit manually!
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use rustemo::Result;
use rustemo::lexer::{self, Token, AsStr, StringLexer};
use rustemo::parser::Parser;
use rustemo::builder::Builder;
use rustemo::lr::builder::LRBuilder;
use rustemo::lr::parser::{LRParser, ParserDefinition};
use rustemo::lr::parser::Action::{self, Shift, Reduce, Accept, Error};
use rustemo::index::{StateIndex, TermIndex, NonTermIndex, ProdIndex};
#[allow(unused_imports)]
use rustemo::debug::{log, logn};
const TERMINAL_COUNT: usize = 3usize;
const NONTERMINAL_COUNT: usize = 5usize;
const STATE_COUNT: usize = 7usize;
#[allow(dead_code)]
const MAX_ACTIONS: usize = 2usize;
use regex::Regex;
use once_cell::sync::Lazy;
use super::output_dir_actions;
pub type Input = str;
pub type Context<'i> = lexer::Context<'i, Input>;
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    #[default]
    STOP,
    Tb,
    Num,
}
impl AsStr for TokenKind {
    #[allow(dead_code)]
    fn as_str(&self) -> &'static str {
        match self {
            TokenKind::STOP => "STOP",
            TokenKind::Tb => "Tb",
            TokenKind::Num => "Num",
        }
    }
}
impl From<TermIndex> for TokenKind {
    fn from(term_index: TermIndex) -> Self {
        match term_index.0 {
            0usize => TokenKind::STOP,
            1usize => TokenKind::Tb,
            2usize => TokenKind::Num,
            _ => unreachable!(),
        }
    }
}
impl From<TokenKind> for TermIndex {
    fn from(token_kind: TokenKind) -> Self {
        match token_kind {
            TokenKind::STOP => TermIndex(0usize),
            TokenKind::Tb => TermIndex(1usize),
            TokenKind::Num => TermIndex(2usize),
        }
    }
}
#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy)]
pub enum ProdKind {
    AP1,
    B1P1,
    B1P2,
    BP1,
}
impl AsStr for ProdKind {
    #[allow(dead_code)]
    fn as_str(&self) -> &'static str {
        match self {
            ProdKind::AP1 => "AP1",
            ProdKind::B1P1 => "B1P1",
            ProdKind::B1P2 => "B1P2",
            ProdKind::BP1 => "BP1",
        }
    }
}
impl std::fmt::Display for ProdKind {
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
impl From<ProdIndex> for ProdKind {
    fn from(prod_index: ProdIndex) -> Self {
        match prod_index.0 {
            1usize => ProdKind::AP1,
            2usize => ProdKind::B1P1,
            3usize => ProdKind::B1P2,
            4usize => ProdKind::BP1,
            _ => unreachable!(),
        }
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
    actions: [[Action; TERMINAL_COUNT]; STATE_COUNT],
    gotos: [[Option<StateIndex>; NONTERMINAL_COUNT]; STATE_COUNT],
    token_recognizers: [[Option<TokenRecognizer>; 2usize]; STATE_COUNT],
}
pub(crate) static PARSER_DEFINITION: OutputDirParserDefinition = OutputDirParserDefinition {
    actions: [
        [Error, Shift(StateIndex(1usize)), Error],
        [
            Error,
            Reduce(ProdIndex(4usize), 1usize, NonTermIndex(4usize)),
            Reduce(ProdIndex(4usize), 1usize, NonTermIndex(4usize)),
        ],
        [Accept, Error, Error],
        [Error, Shift(StateIndex(1usize)), Shift(StateIndex(5usize))],
        [
            Error,
            Reduce(ProdIndex(3usize), 1usize, NonTermIndex(3usize)),
            Reduce(ProdIndex(3usize), 1usize, NonTermIndex(3usize)),
        ],
        [Reduce(ProdIndex(1usize), 2usize, NonTermIndex(2usize)), Error, Error],
        [
            Error,
            Reduce(ProdIndex(2usize), 2usize, NonTermIndex(3usize)),
            Reduce(ProdIndex(2usize), 2usize, NonTermIndex(3usize)),
        ],
    ],
    gotos: [
        [
            None,
            None,
            Some(StateIndex(2usize)),
            Some(StateIndex(3usize)),
            Some(StateIndex(4usize)),
        ],
        [None, None, None, None, None],
        [None, None, None, None, None],
        [None, None, None, None, Some(StateIndex(6usize))],
        [None, None, None, None, None],
        [None, None, None, None, None],
        [None, None, None, None, None],
    ],
    token_recognizers: [
        [
            Some(TokenRecognizer {
                token_kind: TokenKind::Tb,
                recognizer: Recognizer::StrMatch("b"),
                finish: true,
            }),
            None,
        ],
        [
            Some(TokenRecognizer {
                token_kind: TokenKind::Tb,
                recognizer: Recognizer::StrMatch("b"),
                finish: true,
            }),
            Some(TokenRecognizer {
                token_kind: TokenKind::Num,
                recognizer: Recognizer::RegexMatch(2usize),
                finish: true,
            }),
        ],
        [
            Some(TokenRecognizer {
                token_kind: TokenKind::STOP,
                recognizer: Recognizer::Stop,
                finish: true,
            }),
            None,
        ],
        [
            Some(TokenRecognizer {
                token_kind: TokenKind::Tb,
                recognizer: Recognizer::StrMatch("b"),
                finish: true,
            }),
            Some(TokenRecognizer {
                token_kind: TokenKind::Num,
                recognizer: Recognizer::RegexMatch(2usize),
                finish: true,
            }),
        ],
        [
            Some(TokenRecognizer {
                token_kind: TokenKind::Tb,
                recognizer: Recognizer::StrMatch("b"),
                finish: true,
            }),
            Some(TokenRecognizer {
                token_kind: TokenKind::Num,
                recognizer: Recognizer::RegexMatch(2usize),
                finish: true,
            }),
        ],
        [
            Some(TokenRecognizer {
                token_kind: TokenKind::STOP,
                recognizer: Recognizer::Stop,
                finish: true,
            }),
            None,
        ],
        [
            Some(TokenRecognizer {
                token_kind: TokenKind::Tb,
                recognizer: Recognizer::StrMatch("b"),
                finish: true,
            }),
            Some(TokenRecognizer {
                token_kind: TokenKind::Num,
                recognizer: Recognizer::RegexMatch(2usize),
                finish: true,
            }),
        ],
    ],
};
impl ParserDefinition<TokenRecognizer> for OutputDirParserDefinition {
    fn action(&self, state_index: StateIndex, term_index: TermIndex) -> Action {
        PARSER_DEFINITION.actions[state_index.0][term_index.0]
    }
    fn goto(&self, state_index: StateIndex, nonterm_index: NonTermIndex) -> StateIndex {
        PARSER_DEFINITION.gotos[state_index.0][nonterm_index.0].unwrap()
    }
    fn recognizers(&self, state_index: StateIndex) -> Vec<&TokenRecognizer> {
        PARSER_DEFINITION
            .token_recognizers[state_index.0]
            .iter()
            .map_while(|tr| tr.as_ref())
            .collect()
    }
}
#[derive(Default)]
pub struct OutputDirParser {
    content: Option<<Input as ToOwned>::Owned>,
}
#[allow(dead_code)]
impl<'i> OutputDirParser {
    pub fn new() -> Self {
        Self { content: None }
    }
    #[allow(clippy::needless_lifetimes)]
    pub fn parse_file<P: AsRef<std::path::Path>>(
        &'i mut self,
        file: P,
    ) -> Result<<DefaultBuilder as Builder>::Output> {
        self.content = Some(<Input as rustemo::lexer::Input>::read_file(&file)?);
        let mut context = Context::new(
            file.as_ref().to_string_lossy().to_string(),
            self.content.as_ref().unwrap(),
        );
        self.inner_parse(&mut context)
    }
    #[allow(clippy::needless_lifetimes)]
    pub fn parse(
        &self,
        input: &'i Input,
    ) -> Result<<DefaultBuilder as Builder>::Output> {
        let mut context = Context::new("<str>".to_string(), input);
        self.inner_parse(&mut context)
    }
    #[allow(clippy::needless_lifetimes)]
    fn inner_parse(
        &self,
        context: &mut Context<'i>,
    ) -> Result<<DefaultBuilder as Builder>::Output> {
        let local_lexer = StringLexer::new(true);
        let lexer = &local_lexer;
        let mut local_builder = DefaultBuilder::new();
        let builder = &mut local_builder;
        let mut parser = LRParser::new(&PARSER_DEFINITION, StateIndex(0), false);
        parser.parse(context, lexer, builder)
    }
}
pub(crate) static RECOGNIZERS: [Option<Lazy<Regex>>; TERMINAL_COUNT] = [
    None,
    None,
    Some(Lazy::new(|| { Regex::new(concat!("^", "\\d+")).unwrap() })),
];
#[allow(dead_code)]
#[derive(Debug)]
pub enum Recognizer {
    Stop,
    StrMatch(&'static str),
    RegexMatch(usize),
}
#[derive(Debug)]
pub struct TokenRecognizer {
    token_kind: TokenKind,
    recognizer: Recognizer,
    finish: bool,
}
impl lexer::TokenRecognizer for TokenRecognizer {
    type TokenKind = TokenKind;
    type Input = str;
    fn recognize<'i>(&self, input: &'i str) -> Option<&'i str> {
        match &self.recognizer {
            Recognizer::StrMatch(s) => {
                logn!("Recognizing <{:?}> -- ", self.token_kind());
                if input.starts_with(s) {
                    log!("recognized");
                    Some(s)
                } else {
                    log!("not recognized");
                    None
                }
            }
            Recognizer::RegexMatch(r) => {
                logn!("Recognizing <{:?}> -- ", self.token_kind());
                let match_str = RECOGNIZERS[*r].as_ref().unwrap().find(input);
                match match_str {
                    Some(x) => {
                        let x_str = x.as_str();
                        log!("recognized <{}>", x_str);
                        Some(x_str)
                    }
                    None => {
                        log!("not recognized");
                        None
                    }
                }
            }
            Recognizer::Stop => {
                logn!("Recognizing <STOP> -- ");
                if input.is_empty() {
                    log!("recognized");
                    Some("")
                } else {
                    log!("not recognized");
                    None
                }
            }
        }
    }
    #[inline]
    fn token_kind(&self) -> TokenKind {
        self.token_kind
    }
    #[inline]
    fn finish(&self) -> bool {
        self.finish
    }
}
impl PartialEq for TokenRecognizer {
    fn eq(&self, other: &Self) -> bool {
        self.token_kind == other.token_kind
    }
}
impl Eq for TokenRecognizer {}
impl Hash for TokenRecognizer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.token_kind.hash(state);
    }
}
pub struct DefaultBuilder {
    res_stack: Vec<Symbol>,
}
impl Builder for DefaultBuilder {
    type Output = output_dir_actions::A;
    fn new() -> Self {
        Self { res_stack: vec![] }
    }
    fn get_result(&mut self) -> Self::Output {
        match self.res_stack.pop().unwrap() {
            Symbol::NonTerminal(NonTerminal::A(r)) => r,
            _ => panic!("Invalid result on the parse stack!"),
        }
    }
}
impl<'i> LRBuilder<'i, Input, TokenKind> for DefaultBuilder {
    #![allow(unused_variables)]
    fn shift_action(
        &mut self,
        context: &mut Context<'i>,
        token: Token<'i, Input, TokenKind>,
    ) {
        let val = match token.kind {
            TokenKind::STOP => panic!("Cannot shift STOP token!"),
            TokenKind::Tb => Terminal::Tb,
            TokenKind::Num => Terminal::Num(output_dir_actions::num(context, token)),
        };
        self.res_stack.push(Symbol::Terminal(val));
    }
    fn reduce_action(
        &mut self,
        context: &mut Context<'i>,
        prod_idx: ProdIndex,
        _prod_len: usize,
    ) {
        let prod = match ProdKind::from(prod_idx) {
            ProdKind::AP1 => {
                let mut i = self
                    .res_stack
                    .split_off(self.res_stack.len() - 2usize)
                    .into_iter();
                match (i.next().unwrap(), i.next().unwrap()) {
                    (
                        Symbol::NonTerminal(NonTerminal::B1(p0)),
                        Symbol::Terminal(Terminal::Num(p1)),
                    ) => NonTerminal::A(output_dir_actions::a_c1(context, p0, p1)),
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
                    ) => NonTerminal::B1(output_dir_actions::b1_c1(context, p0, p1)),
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
                        NonTerminal::B1(output_dir_actions::b1_b(context, p0))
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
