/// Generated by rustemo on 2022-07-25 11:54:05.910374629 +02:00

use regex::Regex;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

use std::marker::PhantomData;
use rustemo_rt::lexer::{Lexer, DefaultLexer, Token, LexerDefinition, RecognizerIterator};
use rustemo_rt::parser::Parser;
use rustemo_rt::lr::{LRParser, LRContext, ParserDefinition};
use rustemo_rt::lr::Action::{self, Shift, Reduce, Accept, Error};
use rustemo_rt::index::{StateIndex, TermIndex, NonTermIndex, ProdIndex};
use rustemo_rt::builder::Builder;
use rustemo_rt::grammar::{TerminalInfo, TerminalInfos, TerminalsState};
use rustemo_rt::debug::{log, logn};

use super::calculator_actions::*;

const TERMINAL_NO: usize = 6;
const NONTERMINAL_NO: usize = 5;
const STATE_NO: usize = 12;
const MAX_ACTIONS: usize = 4;

#[derive(Debug, Copy, Clone, TryFromPrimitive)]
#[repr(usize)]
pub enum TermKind {
    STOP = 0,
    Plus = 1,
    Mul = 2,
    LParen = 3,
    RParen = 4,
    Num = 5,
}

#[derive(Debug, Copy, Clone)]
pub enum NonTermKind {
    EMPTY = 0,
    AUG = 1,
    E = 2,
    T = 3,
    F = 4,
}

#[derive(Debug)]
pub enum Symbol {
    Terminal(Terminal),
    NonTerminal(NonTerminal)
}

#[derive(Debug)]
pub enum Terminal {
    STOP,
    Plus,
    Mul,
    LParen,
    RParen,
    Num(Num),
}

#[derive(Debug)]
pub enum NonTerminal {
    E(E),
    T(T),
    F(F),
}

#[derive(Copy, Clone, TryFromPrimitive)]
#[repr(usize)]
pub enum ProdKind {
    EP0 = 1,
    EP1 = 2,
    TP0 = 3,
    TP1 = 4,
    FP0 = 5,
    FP1 = 6,
}

pub struct CalculatorParserDefinition {
    actions: [[Action; TERMINAL_NO]; STATE_NO],
    gotos: [[Option<StateIndex>; NONTERMINAL_NO]; STATE_NO]
}

pub(in crate) static PARSER_DEFINITION: CalculatorParserDefinition = CalculatorParserDefinition {
    actions: [
    // State 0:AUG
    [Error, Error, Error, Shift(StateIndex(1), TermIndex(3)), Error, Shift(StateIndex(2), TermIndex(5))],
    // State 1:LParen
    [Error, Error, Error, Shift(StateIndex(1), TermIndex(3)), Error, Shift(StateIndex(2), TermIndex(5))],
    // State 2:Num
    [Reduce(ProdIndex(6), 1, NonTermIndex(4), "<?>"), Reduce(ProdIndex(6), 1, NonTermIndex(4), "<?>"), Reduce(ProdIndex(6), 1, NonTermIndex(4), "<?>"), Error, Reduce(ProdIndex(6), 1, NonTermIndex(4), "<?>"), Error],
    // State 3:E
    [Accept, Shift(StateIndex(7), TermIndex(1)), Error, Error, Error, Error],
    // State 4:T
    [Reduce(ProdIndex(2), 1, NonTermIndex(2), "<?>"), Reduce(ProdIndex(2), 1, NonTermIndex(2), "<?>"), Shift(StateIndex(8), TermIndex(2)), Error, Reduce(ProdIndex(2), 1, NonTermIndex(2), "<?>"), Error],
    // State 5:F
    [Reduce(ProdIndex(4), 1, NonTermIndex(3), "<?>"), Reduce(ProdIndex(4), 1, NonTermIndex(3), "<?>"), Reduce(ProdIndex(4), 1, NonTermIndex(3), "<?>"), Error, Reduce(ProdIndex(4), 1, NonTermIndex(3), "<?>"), Error],
    // State 6:E
    [Error, Shift(StateIndex(7), TermIndex(1)), Error, Error, Shift(StateIndex(9), TermIndex(4)), Error],
    // State 7:Plus
    [Error, Error, Error, Shift(StateIndex(1), TermIndex(3)), Error, Shift(StateIndex(2), TermIndex(5))],
    // State 8:Mul
    [Error, Error, Error, Shift(StateIndex(1), TermIndex(3)), Error, Shift(StateIndex(2), TermIndex(5))],
    // State 9:RParen
    [Reduce(ProdIndex(5), 3, NonTermIndex(4), "<?>"), Reduce(ProdIndex(5), 3, NonTermIndex(4), "<?>"), Reduce(ProdIndex(5), 3, NonTermIndex(4), "<?>"), Error, Reduce(ProdIndex(5), 3, NonTermIndex(4), "<?>"), Error],
    // State 10:T
    [Reduce(ProdIndex(1), 3, NonTermIndex(2), "<?>"), Reduce(ProdIndex(1), 3, NonTermIndex(2), "<?>"), Shift(StateIndex(8), TermIndex(2)), Error, Reduce(ProdIndex(1), 3, NonTermIndex(2), "<?>"), Error],
    // State 11:F
    [Reduce(ProdIndex(3), 3, NonTermIndex(3), "<?>"), Reduce(ProdIndex(3), 3, NonTermIndex(3), "<?>"), Reduce(ProdIndex(3), 3, NonTermIndex(3), "<?>"), Error, Reduce(ProdIndex(3), 3, NonTermIndex(3), "<?>"), Error],
],
    gotos: [
    // State 0:AUG
    [None, None, Some(StateIndex(3)), Some(StateIndex(4)), Some(StateIndex(5))],
    // State 1:LParen
    [None, None, Some(StateIndex(6)), Some(StateIndex(4)), Some(StateIndex(5))],
    // State 2:Num
    [None, None, None, None, None],
    // State 3:E
    [None, None, None, None, None],
    // State 4:T
    [None, None, None, None, None],
    // State 5:F
    [None, None, None, None, None],
    // State 6:E
    [None, None, None, None, None],
    // State 7:Plus
    [None, None, None, Some(StateIndex(10)), Some(StateIndex(5))],
    // State 8:Mul
    [None, None, None, None, Some(StateIndex(11))],
    // State 9:RParen
    [None, None, None, None, None],
    // State 10:T
    [None, None, None, None, None],
    // State 11:F
    [None, None, None, None, None],
]};

impl ParserDefinition for CalculatorParserDefinition {
    fn action(&self, state_index: StateIndex, term_index: TermIndex) -> Action {
        PARSER_DEFINITION.actions[state_index.0][term_index.0]
    }
    fn goto(&self, state_index: StateIndex, nonterm_index: NonTermIndex) -> StateIndex {
        PARSER_DEFINITION.gotos[state_index.0][nonterm_index.0].unwrap()
    }
}

pub struct CalculatorParser<'i>(pub LRParser<&'i str, CalculatorParserDefinition>);

impl<'i, L, B> Parser<L, B> for CalculatorParser<'i>
where
    L: Lexer<Input = &'i str>,
    B: Builder<Lexer = L>
{
    fn parse(&mut self, lexer: L, mut builder: B) -> B::Output {
        self.0.parse(lexer, builder)
    }
}

impl<'i> CalculatorParser<'i> {
    pub fn parse_default(&mut self, lexer: CalculatorLexer<'i>) -> E {
        let builder =
            CalculatorBuilder::<'_, <CalculatorLexer as Lexer>::Input>::new();
        match self.parse(lexer, builder) {
            Symbol::NonTerminal(NonTerminal::E(p)) => p,
            _ => {
                panic!("Invalid return type of inner parse.")
            }
        }
    }
}

impl<'i> Default for CalculatorParser<'i> {
    fn default() -> Self {
        Self(LRParser {
            context: LRContext {
                parse_stack: vec![StateIndex(0)],
                current_state: StateIndex(0),
                position: 0,
                token: None,
            },
            definition: &PARSER_DEFINITION,
        })
    }
}

pub struct CalculatorLexerDefinition {
    terminals: TerminalInfos<TERMINAL_NO>,
    terminals_for_state: TerminalsState<MAX_ACTIONS, STATE_NO>,
    recognizers: [fn(&str) -> Option<&str>; TERMINAL_NO]
}

pub(in crate) static LEXER_DEFINITION: CalculatorLexerDefinition = CalculatorLexerDefinition {
    terminals: [
    TerminalInfo {
        id: TermIndex(0),
        name: "STOP",
        location: None,
    },
    TerminalInfo {
        id: TermIndex(1),
        name: "Plus",
        location: None,
    },
    TerminalInfo {
        id: TermIndex(2),
        name: "Mul",
        location: None,
    },
    TerminalInfo {
        id: TermIndex(3),
        name: "LParen",
        location: None,
    },
    TerminalInfo {
        id: TermIndex(4),
        name: "RParen",
        location: None,
    },
    TerminalInfo {
        id: TermIndex(5),
        name: "Num",
        location: None,
    },
],
    // Expected terminals/tokens indexed by state id.
    // Sorted by priority.
    terminals_for_state: [
    // State 0:AUG
    [Some(5), Some(3), None, None],
    // State 1:LParen
    [Some(5), Some(3), None, None],
    // State 2:Num
    [Some(0), Some(1), Some(2), Some(4)],
    // State 3:E
    [Some(0), Some(1), None, None],
    // State 4:T
    [Some(0), Some(1), Some(2), Some(4)],
    // State 5:F
    [Some(0), Some(1), Some(2), Some(4)],
    // State 6:E
    [Some(1), Some(4), None, None],
    // State 7:Plus
    [Some(5), Some(3), None, None],
    // State 8:Mul
    [Some(5), Some(3), None, None],
    // State 9:RParen
    [Some(0), Some(1), Some(2), Some(4)],
    // State 10:T
    [Some(0), Some(1), Some(2), Some(4)],
    // State 11:F
    [Some(0), Some(1), Some(2), Some(4)],
],
recognizers: [
    // 0:STOP
    |input: &str| {
        logn!("Recognizing <STOP> -- ");
        if input.len() == 0 {
            log!("recognized");
            Some("")
        } else {
            log!("not recognized");
            None
        }
    },
    // 1:Plus
    |input: &str| {
        logn!("Recognizing <Plus> -- ");
        if input.starts_with("+"){
            log!("recognized");
            Some("+")
        } else {
            log!("not recognized");
            None
        }
    },
    // 2:Mul
    |input: &str| {
        logn!("Recognizing <Mul> -- ");
        if input.starts_with("*"){
            log!("recognized");
            Some("*")
        } else {
            log!("not recognized");
            None
        }
    },
    // 3:LParen
    |input: &str| {
        logn!("Recognizing <LParen> -- ");
        if input.starts_with("("){
            log!("recognized");
            Some("(")
        } else {
            log!("not recognized");
            None
        }
    },
    // 4:RParen
    |input: &str| {
        logn!("Recognizing <RParen> -- ");
        if input.starts_with(")"){
            log!("recognized");
            Some(")")
        } else {
            log!("not recognized");
            None
        }
    },
    // 5:Num
    |input: &str| {
        logn!("Recognizing <Num> -- ");
        let regex = Regex::new(r#"^\d+"#).unwrap();
        let match_str = regex.find(input);
        match match_str {
            Some(x) => {
                let x_str = x.as_str();
                log!("recognized <{}>", x_str);
                Some(x_str)
            },
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

    fn recognizers(&self, state_index: StateIndex) -> RecognizerIterator<Self::Recognizer> {
            RecognizerIterator {
                terminals: &LEXER_DEFINITION.terminals,
                terminals_for_state: &LEXER_DEFINITION.terminals_for_state[state_index.0][..],
                recognizers: &LEXER_DEFINITION.recognizers,
                index: 0
            }
    }
}

pub struct CalculatorLexer<'i>(DefaultLexer<'i, CalculatorLexerDefinition>);

impl<'i> Lexer for CalculatorLexer<'i> {
    type Input = &'i str;

    fn next_token(
        &self,
        context: &mut impl rustemo_rt::parser::Context<Self::Input>,
    ) -> Option<rustemo_rt::lexer::Token<Self::Input>> {
        self.0.next_token(context)
    }
}

// Enables creating a lexer from a reference to an object that can be converted
// to a string reference.
impl<'i, T> From<&'i T> for CalculatorLexer<'i>
where
    T: AsRef<str> + ?Sized,
{
    fn from(input: &'i T) -> Self {
        Self(DefaultLexer::new(input.as_ref(), &LEXER_DEFINITION))
    }
}

pub struct CalculatorBuilder<'i, I: 'i> {
    res_stack: Vec<Symbol>,
    phantom: PhantomData<&'i I>
}

impl<'i, I> Builder for CalculatorBuilder<'i, I>
{
    type Output = Symbol;
    type Lexer = CalculatorLexer<'i>;

    fn new() -> Self {
        CalculatorBuilder {
            res_stack: vec![],
            phantom: PhantomData,
        }
    }

    fn shift_action(&mut self, term_idx: TermIndex, token: Token<<Self::Lexer as Lexer>::Input>) {
        let termval = match TermKind::try_from(term_idx.0).unwrap() {
            TermKind::STOP => Terminal::STOP,
            TermKind::Plus => Terminal::Plus,
            TermKind::Mul => Terminal::Mul,
            TermKind::LParen => Terminal::LParen,
            TermKind::RParen => Terminal::RParen,
            TermKind::Num => Terminal::Num(num(token)),
        };
        self.res_stack.push(Symbol::Terminal(termval));
    }
    
    fn reduce_action(&mut self, prod_kind: ProdIndex, prod_len: usize, _prod_str: &'static str) {
        let prod = match ProdKind::try_from(prod_kind.0).unwrap() {
            ProdKind::EP0 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-3).into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {                
                    (Symbol::NonTerminal(NonTerminal::E(p0)), _, Symbol::NonTerminal(NonTerminal::T(p1))) => NonTerminal::E(e_p0(p0, p1)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::EP1 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match i.next().unwrap() {                
                    Symbol::NonTerminal(NonTerminal::T(p0)) => NonTerminal::E(e_p1(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::TP0 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-3).into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {                
                    (Symbol::NonTerminal(NonTerminal::T(p0)), _, Symbol::NonTerminal(NonTerminal::F(p1))) => NonTerminal::T(t_p0(p0, p1)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::TP1 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match i.next().unwrap() {                
                    Symbol::NonTerminal(NonTerminal::F(p0)) => NonTerminal::T(t_p1(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::FP0 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-3).into_iter();
                match (i.next().unwrap(), i.next().unwrap(), i.next().unwrap()) {                
                    (_, Symbol::NonTerminal(NonTerminal::E(p0)), _) => NonTerminal::F(f_p0(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
            ProdKind::FP1 => {
                let mut i = self.res_stack.split_off(self.res_stack.len()-1).into_iter();
                match i.next().unwrap() {                
                    Symbol::Terminal(Terminal::Num(p0)) => NonTerminal::F(f_p1(p0)),
                    _ => panic!("Invalid symbol parse stack data.")
                }
            },
        };
        self.res_stack.push(Symbol::NonTerminal(prod));
    }
    fn get_result(&mut self) -> Self::Output {
        self.res_stack.pop().unwrap()
    }
}
