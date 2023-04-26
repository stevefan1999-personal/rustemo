/// Generated by rustemo. Do not edit manually!
use regex::Regex;
use std::fmt::Debug;
use rustemo::lexer::{self, Token, AsStr};
use rustemo::parser::Parser;
use rustemo::builder::Builder;
use rustemo::Result;
use rustemo::lr::lexer::{LRStringLexer, LexerDefinition, RecognizerIterator};
use rustemo::lr::builder::LRBuilder;
use rustemo::lr::parser::{LRParser, ParserDefinition};
use rustemo::lr::parser::Action::{self, Shift, Reduce, Accept, Error};
use rustemo::index::{StateIndex, TermIndex, NonTermIndex, ProdIndex};
use rustemo::grammar::TerminalsState;
use rustemo::debug::{log, logn};
const TERMINAL_NO: usize = 3usize;
const NONTERMINAL_NO: usize = 3usize;
const STATE_NO: usize = 5usize;
#[allow(dead_code)]
const MAX_ACTIONS: usize = 1usize;
use super::calculator_actions;
pub type Input = str;
pub type Context<'i> = lexer::Context<'i, Input, StateIndex>;
use lazy_static::lazy_static;
lazy_static! {
    static ref REGEX_OPERAND : Regex = Regex::new(concat!("^", "\\d+(\\.\\d+)?"))
    .unwrap(); static ref REGEX_OPERATOR : Regex = Regex::new(concat!("^",
    "\\+|-|\\*|/")).unwrap();
}
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Default, Clone, Copy)]
pub enum TokenKind {
    #[default]
    STOP,
    Operand,
    Operator,
}
impl AsStr for TokenKind {
    #[allow(dead_code)]
    fn as_str(&self) -> &'static str {
        match self {
            TokenKind::STOP => "STOP",
            TokenKind::Operand => "Operand",
            TokenKind::Operator => "Operator",
        }
    }
}
impl From<TermIndex> for TokenKind {
    fn from(term_index: TermIndex) -> Self {
        match term_index.0 {
            0usize => TokenKind::STOP,
            1usize => TokenKind::Operand,
            2usize => TokenKind::Operator,
            _ => unreachable!(),
        }
    }
}
impl From<TokenKind> for TermIndex {
    fn from(token_kind: TokenKind) -> Self {
        match token_kind {
            TokenKind::STOP => TermIndex(0usize),
            TokenKind::Operand => TermIndex(1usize),
            TokenKind::Operator => TermIndex(2usize),
        }
    }
}
#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy)]
pub enum ProdKind {
    ExpressionP1,
}
impl AsStr for ProdKind {
    #[allow(dead_code)]
    fn as_str(&self) -> &'static str {
        match self {
            ProdKind::ExpressionP1 => "ExpressionP1",
        }
    }
}
impl std::fmt::Display for ProdKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            ProdKind::ExpressionP1 => "Expression: Operand Operator Operand",
        };
        write!(f, "{}", name)
    }
}
impl From<ProdIndex> for ProdKind {
    fn from(prod_index: ProdIndex) -> Self {
        match prod_index.0 {
            1usize => ProdKind::ExpressionP1,
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
    Operand(calculator_actions::Operand),
    Operator(calculator_actions::Operator),
}
#[derive(Debug)]
pub enum NonTerminal {
    Expression(calculator_actions::Expression),
}
pub struct CalculatorParserDefinition {
    actions: [[Action; TERMINAL_NO]; STATE_NO],
    gotos: [[Option<StateIndex>; NONTERMINAL_NO]; STATE_NO],
}
pub(crate) static PARSER_DEFINITION: CalculatorParserDefinition = CalculatorParserDefinition {
    actions: [
        [Error, Shift(StateIndex(1usize)), Error],
        [Error, Error, Shift(StateIndex(3usize))],
        [Accept, Error, Error],
        [Error, Shift(StateIndex(4usize)), Error],
        [Reduce(ProdIndex(1usize), 3usize, NonTermIndex(2usize)), Error, Error],
    ],
    gotos: [
        [None, None, Some(StateIndex(2usize))],
        [None, None, None],
        [None, None, None],
        [None, None, None],
        [None, None, None],
    ],
};
impl ParserDefinition for CalculatorParserDefinition {
    fn action(&self, state_index: StateIndex, term_index: TermIndex) -> Action {
        PARSER_DEFINITION.actions[state_index.0][term_index.0]
    }
    fn goto(&self, state_index: StateIndex, nonterm_index: NonTermIndex) -> StateIndex {
        PARSER_DEFINITION.gotos[state_index.0][nonterm_index.0].unwrap()
    }
}
#[derive(Default)]
pub struct CalculatorParser {
    content: Option<<Input as ToOwned>::Owned>,
}
#[allow(dead_code)]
impl CalculatorParser {
    pub fn new() -> Self {
        Default::default()
    }
    #[allow(clippy::needless_lifetimes)]
    pub fn parse_file<'i, P: AsRef<std::path::Path>>(
        &'i mut self,
        file: P,
    ) -> Result<calculator_actions::Expression> {
        self.content = Some(<Input as rustemo::lexer::Input>::read_file(&file)?);
        let mut context = Context::new(
            file.as_ref().to_string_lossy().to_string(),
            self.content.as_ref().unwrap(),
        );
        Self::inner_parse(&mut context)
    }
    #[allow(clippy::needless_lifetimes)]
    pub fn parse<'i>(input: &'i Input) -> Result<calculator_actions::Expression> {
        let mut context = Context::new("<str>".to_string(), input);
        Self::inner_parse(&mut context)
    }
    #[allow(clippy::needless_lifetimes)]
    fn inner_parse<'i>(
        context: &mut Context<'i>,
    ) -> Result<calculator_actions::Expression> {
        let lexer = LRStringLexer::new(&LEXER_DEFINITION, false, true);
        let mut builder = CalculatorBuilder::new();
        let mut parser = LRParser::new(&PARSER_DEFINITION, StateIndex(0));
        parser.parse(context, &lexer, &mut builder)
    }
}
pub struct CalculatorLexerDefinition {
    terminals_for_state: TerminalsState<MAX_ACTIONS, STATE_NO>,
    recognizers: [fn(&str) -> Option<&str>; TERMINAL_NO],
}
#[allow(clippy::single_char_pattern)]
pub(crate) static LEXER_DEFINITION: CalculatorLexerDefinition = CalculatorLexerDefinition {
    terminals_for_state: [
        [Some(1usize)],
        [Some(2usize)],
        [Some(0usize)],
        [Some(1usize)],
        [Some(0usize)],
    ],
    recognizers: [
        |input: &str| {
            logn!("Recognizing <STOP> -- ");
            if input.is_empty() {
                log!("recognized");
                Some("")
            } else {
                log!("not recognized");
                None
            }
        },
        |input: &str| {
            logn!("Recognizing <{}> -- ", "Operand");
            let match_str = REGEX_OPERAND.find(input);
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
        },
        |input: &str| {
            logn!("Recognizing <{}> -- ", "Operator");
            let match_str = REGEX_OPERATOR.find(input);
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
        },
    ],
};
impl LexerDefinition for CalculatorLexerDefinition {
    type Recognizer = for<'i> fn(&'i str) -> Option<&'i str>;
    fn recognizers(
        &self,
        state_index: StateIndex,
    ) -> RecognizerIterator<Self::Recognizer> {
        RecognizerIterator {
            terminals_for_state: &LEXER_DEFINITION
                .terminals_for_state[state_index.0][..],
            recognizers: &LEXER_DEFINITION.recognizers,
            index: 0,
        }
    }
}
struct CalculatorBuilder {
    res_stack: Vec<Symbol>,
}
impl Builder for CalculatorBuilder {
    type Output = calculator_actions::Expression;
    fn new() -> Self {
        Self { res_stack: vec![] }
    }
    fn get_result(&mut self) -> Self::Output {
        match self.res_stack.pop().unwrap() {
            Symbol::NonTerminal(NonTerminal::Expression(r)) => r,
            _ => panic!("Invalid result on the parse stack!"),
        }
    }
}
impl<'i> LRBuilder<'i, Input, TokenKind> for CalculatorBuilder {
    #![allow(unused_variables)]
    fn shift_action(
        &mut self,
        context: &mut Context<'i>,
        token: Token<'i, Input, TokenKind>,
    ) {
        let val = match token.kind {
            TokenKind::STOP => panic!("Cannot shift STOP token!"),
            TokenKind::Operand => {
                Terminal::Operand(calculator_actions::operand(context, token))
            }
            TokenKind::Operator => {
                Terminal::Operator(calculator_actions::operator(context, token))
            }
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
            ProdKind::ExpressionP1 => {
                let mut i = self
                    .res_stack
                    .split_off(self.res_stack.len() - 3usize)
                    .into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {
                    (
                        Symbol::Terminal(Terminal::Operand(p0)),
                        Symbol::Terminal(Terminal::Operator(p1)),
                        Symbol::Terminal(Terminal::Operand(p2)),
                    ) => {
                        NonTerminal::Expression(
                            calculator_actions::expression_c1(context, p0, p1, p2),
                        )
                    }
                    _ => panic!("Invalid symbol parse stack data."),
                }
            }
        };
        self.res_stack.push(Symbol::NonTerminal(prod));
    }
}
