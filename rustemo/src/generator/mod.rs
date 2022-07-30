use chrono::Local;
use convert_case::{Case, Casing};
use indoc::indoc;
use rustemo_rt::{
    error::{RustemoError, RustemoResult},
    index::{NonTermIndex, StateVec, SymbolIndex, TermIndex},
};
use std::{
    fmt::Debug,
    fs::File,
    io::{self, Write},
    iter::repeat,
    path::Path,
};

use crate::{
    grammar::{res_symbol, Grammar},
    rustemo_actions::Recognizer,
    settings::Settings,
    table::{lr_states_for_grammar, LRState},
};

#[cfg(test)]
mod tests;

macro_rules! geni {
    ($w:expr, $($args:tt)*) => {
        ($w).write_indented(&::std::fmt::format(format_args!($($args)*)))?
    }
}

macro_rules! gen {
    ($w:expr, $($args:tt)*) => {
        ($w).write(&::std::fmt::format(format_args!($($args)*)))?
    }
}

#[derive(Default)]
struct RustWrite<W: Write> {
    write: W,
    indent: usize,
}

const DEFAULT_INDENT: usize = 4;

impl<W: Write> RustWrite<W> {
    pub fn new(w: W) -> RustWrite<W> {
        RustWrite {
            write: w,
            indent: 0,
        }
    }

    pub fn inc_indent(&mut self) {
        self.indent += DEFAULT_INDENT;
    }

    pub fn dec_indent(&mut self) {
        self.indent -= DEFAULT_INDENT;
    }

    fn write_indented(&mut self, out: &str) -> io::Result<()> {
        let mut lines = out.lines().peekable();
        while let Some(line) = lines.next() {
            write!(self.write, "{0:1$}", "", self.indent)?;
            if lines.peek().is_some() {
                writeln!(self.write, "{}", line)?;
            } else if out.ends_with("\n") {
                writeln!(self.write, "{}", line)?;
            } else {
                write!(self.write, "{}", line)?;
            }
        }
        Ok(())
    }

    pub fn write(&mut self, out: &str) -> io::Result<()> {
        write!(self.write, "{}", out)
    }
}

pub fn generate_parser<F>(grammar_path: F) -> RustemoResult<()>
where
    F: AsRef<Path> + Debug,
{
    let mut file_name = String::from(
        grammar_path
            .as_ref()
            .file_name()
            .ok_or(RustemoError::Error("Invalid file name.".to_string()))?
            .to_str()
            .ok_or(RustemoError::Error(
                "Filename must be valid unicode.".to_string(),
            ))?,
    );
    file_name = String::from(file_name.strip_suffix(".rustemo").ok_or(
        RustemoError::Error(
            "Grammar filename must use .rustemo extension.".to_string(),
        ),
    )?);

    //let grammar = Grammar::from_file(grammar_path)?;
    let grammar_input = std::fs::read_to_string(grammar_path.as_ref())?;
    let grammar = Grammar::from_string(grammar_input)?;

    let states = lr_states_for_grammar(&grammar, &Settings::default());

    // Generate parser definition
    let out_file = grammar_path.as_ref().with_extension("rs");
    let out_file = File::create(out_file)?;
    generate_parser_definition(&grammar, &file_name, states, out_file)?;

    Ok(())
}

fn generate_parser_definition<W: Write>(
    grammar: &Grammar,
    file_name: &str,
    states: StateVec<LRState>,
    out: W,
) -> RustemoResult<()> {
    let mut out = RustWrite::new(out);
    let parser_name = file_name.to_case(Case::Pascal);
    let root_symbol_name =
        grammar.symbol_name(grammar.nonterm_to_symbol(NonTermIndex(2)));

    geni!(out, "/// Generated by rustemo on {}\n\n", Local::now());

    let max_actions = states
        .iter()
        .map(|x| x.actions.iter().filter(|x| !x.is_empty()).count())
        .max()
        .unwrap();
    geni!(
        out,
        indoc! {r#"
        use regex::Regex;
        use num_enum::TryFromPrimitive;
        use std:: {{
            convert::TryFrom,
            path::Path,
            fs,
            fmt::Debug,
        }};

        use rustemo_rt::lexer::{{Lexer, Context, Token}};
        use rustemo_rt::parser::Parser;
        use rustemo_rt::builder::Builder;
        use rustemo_rt::error::RustemoResult;
        use rustemo_rt::lr::lexer::{{LRStringLexer, LRContext, LexerDefinition, RecognizerIterator}};
        use rustemo_rt::lr::builder::LRBuilder;
        use rustemo_rt::lr::parser::{{LRParser, ParserDefinition}};
        use rustemo_rt::lr::parser::Action::{{self, Shift, Reduce, Accept, Error}};
        use rustemo_rt::index::{{StateIndex, TermIndex, NonTermIndex, ProdIndex}};
        use rustemo_rt::grammar::{{TerminalInfo, TerminalInfos, TerminalsState}};
        use rustemo_rt::debug::{{log, logn}};

        use super::{file_name}_actions;

        const TERMINAL_NO: usize = {term_count};
        const NONTERMINAL_NO: usize = {nonterm_count};
        const STATE_NO: usize = {states_count};
        const MAX_ACTIONS: usize = {max_actions};

    "#},
        file_name = file_name,
        term_count = grammar.term_len(),
        nonterm_count = grammar.nonterm_len(),
        states_count = states.len(),
        max_actions = max_actions,
    );

    generate_parser_types(&grammar, file_name, &mut out)?;

    geni!(
        out,
        indoc! {r#"
        pub struct {parser_name}ParserDefinition {{
            actions: [[Action; TERMINAL_NO]; STATE_NO],
            gotos: [[Option<StateIndex>; NONTERMINAL_NO]; STATE_NO]
        }}

        pub(in crate) static PARSER_DEFINITION: {parser_name}ParserDefinition = {parser_name}ParserDefinition {{
    "#},
        parser_name = parser_name,
    );

    out.inc_indent();
    geni!(out, "actions: [\n");
    for state in &states {
        geni!(
            out,
            "// State {}:{}\n",
            state.idx,
            grammar.symbol_name(state.symbol)
        );
        geni!(out, "[");
        gen!(
            out,
            "{}",
            state
                .actions
                .iter()
                .map(|action| match action.len() {
                    0 => "Error".into(),
                    1 => format!("{}", action[0]),
                    _ => panic!("Multiple actions for state {}", state.idx),
                })
                .collect::<Vec<_>>()
                .join(", ")
        );
        gen!(out, "],\n");
    }
    out.dec_indent();
    geni!(out, "],\n");

    out.inc_indent();
    geni!(out, "gotos: [\n");
    for state in &states {
        geni!(
            out,
            "// State {}:{}\n",
            state.idx,
            grammar.symbol_name(state.symbol)
        );
        geni!(out, "[");
        gen!(
            out,
            "{}",
            state
                .gotos
                .iter()
                .map(|x| match x {
                    Some(state) => format!("Some(StateIndex({}))", state),
                    None => "None".to_string(),
                })
                .collect::<Vec<_>>()
                .join(", ")
        );
        gen!(out, "],\n");
    }
    out.dec_indent();
    geni!(out, "]}};\n\n");

    geni!(
        out,
        indoc! {r#"
        impl ParserDefinition for {parser_name}ParserDefinition {{
            fn action(&self, state_index: StateIndex, term_index: TermIndex) -> Action {{
                PARSER_DEFINITION.actions[state_index.0][term_index.0]
            }}
            fn goto(&self, state_index: StateIndex, nonterm_index: NonTermIndex) -> StateIndex {{
                PARSER_DEFINITION.gotos[state_index.0][nonterm_index.0].unwrap()
            }}
        }}

        pub struct {parser_name}Parser(LRParser<{parser_name}ParserDefinition>);

        impl<I, L, B> Parser<I, LRContext<I>, L, B> for {parser_name}Parser
        where
            I: Debug,
            L: Lexer<I, LRContext<I>>,
            B: LRBuilder<I>,
        {{
            fn parse(&mut self, context: LRContext<I>, lexer: L, mut builder: B) -> RustemoResult<B::Output> {{
                {parser_name}Parser::default().0.parse(context, lexer, builder)
            }}
        }}

        impl {parser_name}Parser
        {{
            pub fn parse_str<'i>(input: &'i str) -> RustemoResult<<{parser_name}Builder as Builder>::Output> {{
                let context = LRContext::new("<str>".to_string(), input);
                let lexer = LRStringLexer::new(&LEXER_DEFINITION);
                let builder = {parser_name}Builder::new();
                {parser_name}Parser::default().0.parse(context, lexer, builder)
            }}
        }}

        impl Default for {parser_name}Parser {{
            fn default() -> Self {{
                Self(LRParser::new(&PARSER_DEFINITION))
            }}
        }}

        pub struct {parser_name}LexerDefinition {{
            terminals: TerminalInfos<TERMINAL_NO>,
            terminals_for_state: TerminalsState<MAX_ACTIONS, STATE_NO>,
            recognizers: [fn(&str) -> Option<&str>; TERMINAL_NO]
        }}

        pub(in crate) static LEXER_DEFINITION: {parser_name}LexerDefinition = {parser_name}LexerDefinition {{
    "#},
        parser_name = parser_name,
    );

    out.inc_indent();
    geni!(out, "terminals: [\n");
    for terminal in grammar.terminals() {
        geni!(out, "TerminalInfo {{\n");
        out.inc_indent();
        geni!(out, "id: TermIndex({}),\n", terminal.idx);
        geni!(out, "name: \"{}\",\n", terminal.name);
        geni!(out, "location: None,\n");
        out.dec_indent();
        geni!(out, "}},\n");
    }
    out.dec_indent();
    geni!(out, "],\n");

    out.inc_indent();
    geni!(
        out,
        indoc! {"
             // Expected terminals/tokens indexed by state id.
             // Sorted by priority.\n"
        }
    );

    geni!(out, "terminals_for_state: [\n");
    for state in &states {
        geni!(
            out,
            "// State {}:{}\n",
            state.idx,
            grammar.symbol_name(state.symbol)
        );
        geni!(out, "[");
        gen!(
            out,
            "{}",
            &state
                .sorted_terminals
                .iter()
                .map(|x| format!("Some({})", x))
                .chain(
                    // Fill the rest with "None"
                    repeat("None".to_string())
                        .take(max_actions - &state.sorted_terminals.len())
                )
                .collect::<Vec<_>>()
                .join(", ")
        );
        gen!(out, "],\n");
    }
    out.dec_indent();
    geni!(out, "],\n");

    geni!(out, "recognizers: [\n");
    out.inc_indent();
    for terminal in grammar.terminals() {
        if let Some(recognizer) = &terminal.recognizer {
            geni!(out, "// {}:{}\n", terminal.idx, terminal.name);
            match recognizer {
                Recognizer::StrConst(str_match) => {
                    geni!(
                        out,
                        indoc! {
                           r#"
                            |input: &str| {{
                                logn!("Recognizing <{term_name}> -- ");
                                if input.starts_with("{str_match}"){{
                                    log!("recognized");
                                    Some("{str_match}")
                                }} else {{
                                    log!("not recognized");
                                    None
                                }}
                            }},
                            "#
                        },
                        term_name = terminal.name,
                        str_match = str_match
                    )
                }
                Recognizer::RegExTerm(regex_match) => {
                    geni!(
                        out,
                        indoc! {
                           r###"
                            |input: &str| {{
                                logn!("Recognizing <{term_name}> -- ");
                                let regex = Regex::new(r#"^{regex_match}"#).unwrap();
                                let match_str = regex.find(input);
                                match match_str {{
                                    Some(x) => {{
                                        let x_str = x.as_str();
                                        log!("recognized <{{}}>", x_str);
                                        Some(x_str)
                                    }},
                                    None => {{
                                        log!("not recognized");
                                        None
                                    }}
                                }}
                            }},
                            "###
                        },
                        term_name = terminal.name,
                        regex_match = regex_match
                    )
                }
            }
        } else {
            // STOP recognition
            if terminal.idx == TermIndex(0) {
                geni!(
                    out,
                    indoc! {
                        r#"
                        // 0:STOP
                        |input: &str| {{
                            logn!("Recognizing <STOP> -- ");
                            if input.len() == 0 {{
                                log!("recognized");
                                Some("")
                            }} else {{
                                log!("not recognized");
                                None
                            }}
                        }},
                        "#
                    },
                )
            } else {
                // TODO: Custom recognizers?
                unreachable!()
            }
        }
    }
    geni!(out, "],\n");
    out.dec_indent();
    geni!(out, "}};\n");

    geni!(
        out,
        indoc! {r#"

            impl LexerDefinition for {parser_name}LexerDefinition {{
                type Recognizer = for<'i> fn(&'i str) -> Option<&'i str>;

                fn recognizers(&self, state_index: StateIndex) -> RecognizerIterator<Self::Recognizer> {{
                        RecognizerIterator {{
                            terminals: &LEXER_DEFINITION.terminals,
                            terminals_for_state: &LEXER_DEFINITION.terminals_for_state[state_index.0][..],
                            recognizers: &LEXER_DEFINITION.recognizers,
                            index: 0
                        }}
                }}
            }}

            pub struct {parser_name}Builder {{
                res_stack: Vec<Symbol>,
            }}

            impl Builder for {parser_name}Builder
            {{
                type Output = {file_name}_actions::{root_symbol_name};

                fn new() -> Self {{
                    {parser_name}Builder {{
                        res_stack: vec![],
                    }}
                }}

                fn get_result(&mut self) -> RustemoResult<Self::Output> {{
                    match self.res_stack.pop().unwrap() {{
                        Symbol::NonTerminal(NonTerminal::{root_symbol_name}(r)) => Ok(r),
                        _ => panic!("Invalid result on the parsing stack!"),
                    }}
                }}
            }}
        "#
        },
        parser_name = parser_name,
        file_name = file_name,
        root_symbol_name = root_symbol_name,
    );

    geni!(
        out,
        indoc! {r#"
            impl<'i> LRBuilder<&'i str> for {parser_name}Builder {{

                fn shift_action(&mut self, term_idx: TermIndex, token: Token<&'i str>) {{
                    let termval = match TermKind::try_from(term_idx.0).unwrap() {{
        "#},
        parser_name = parser_name,
    );

    out.inc_indent();
    out.inc_indent();
    out.inc_indent();
    for terminal in grammar.terminals() {
        if let Some(Recognizer::RegExTerm(_)) = terminal.recognizer {
            geni!(out,
                  "TermKind::{term_name} => Terminal::{term_name}({file_name}_actions::{action_fun}(token)),\n",
                    term_name=terminal.name,
                    file_name=file_name,
                    action_fun=terminal.name.to_case(Case::Snake)
            )
        } else {
            geni!(
                out,
                "TermKind::{term_name} => Terminal::{term_name},\n",
                term_name = terminal.name
            );
        }
    }
    out.dec_indent();
    geni!(out, "}};\n");
    geni!(out, "self.res_stack.push(Symbol::Terminal(termval));\n");
    out.dec_indent();
    geni!(out, "}}\n");

    geni!(
        out,
        indoc! {r#"

            fn reduce_action(&mut self, prod_kind: ProdIndex, prod_len: usize, _prod_str: &'static str) {{
                let prod = match ProdKind::try_from(prod_kind.0).unwrap() {{
        "#
        }
    );

    // Calling actions on reductions
    out.inc_indent();
    out.inc_indent();
    for production in &grammar.productions()[1..] {
        let prod_nt_name = &grammar.nonterminals()[production.nonterminal].name;
        geni!(
            out,
            "ProdKind::{}P{} => {{\n",
            prod_nt_name,
            production.ntidx
        );
        let rhs_len = production.rhs.len();
        out.inc_indent();
        geni!(out,
              "let mut i = self.res_stack.split_off(self.res_stack.len()-{rhs_len}).into_iter();\n",
              rhs_len=rhs_len
        );
        geni!(
            out,
            "match {}{}{} {{",
            if rhs_len > 1 { "(" } else { "" },
            repeat("i.next().unwrap()")
                .take(rhs_len)
                .collect::<Vec<_>>()
                .join(", "),
            if rhs_len > 1 { ")" } else { "" },
        );
        geni!(out, "\n");
        out.inc_indent();
        let mut counter = 0;
        let mut lhs = production
            .rhs
            .iter()
            .map(|assign| {
                let symbol = res_symbol(assign);
                if grammar.is_term(symbol) {
                    let t =
                        &grammar.terminals()[grammar.symbol_to_term(symbol)];
                    if t.has_content {
                        counter += 1;
                        format!(
                            "Symbol::Terminal(Terminal::{}(p{}))",
                            t.name,
                            counter - 1
                        )
                    } else {
                        "_".to_string()
                    }
                } else {
                    // Special handling of EMPTY non-terminal
                    if grammar.empty_index == symbol {
                        String::from("Symbol::NonTerminal(NonTerminal::EMPTY)")
                    } else {
                        let nt = &grammar.nonterminals()
                            [grammar.symbol_to_nonterm(symbol)];
                        counter += 1;
                        format!(
                            "Symbol::NonTerminal(NonTerminal::{}(p{}))",
                            nt.name,
                            counter - 1
                        )
                    }
                }
            })
            .collect::<Vec<_>>()
            .join(", ");

        if rhs_len > 1 {
            lhs = format!("({})", lhs);
        }

        geni!(
            out,
            "{} => NonTerminal::{}({}_actions::{}_p{}({})),\n",
            lhs,
            prod_nt_name,
            file_name,
            prod_nt_name.to_case(Case::Snake),
            production.ntidx,
            (0..counter)
                .map(|x| format!("p{}", x))
                .collect::<Vec<_>>()
                .join(", ")
        );

        geni!(out, "_ => panic!(\"Invalid symbol parse stack data.\")\n");

        out.dec_indent();
        geni!(out, "}}\n");
        out.dec_indent();
        geni!(out, "}},\n");
    }

    out.dec_indent();
    geni!(out, "}};\n");

    geni!(
        out,
        indoc! {r#"
            self.res_stack.push(Symbol::NonTerminal(prod));
        "#
        }
    );

    out.dec_indent();
    geni!(out, "}}\n");

    out.dec_indent();
    geni!(out, "}}\n");

    Ok(())
}

fn generate_parser_types<W: Write>(
    grammar: &Grammar,
    file_name: &str,
    out: &mut RustWrite<W>,
) -> RustemoResult<()> {
    geni!(
        out,
        indoc! {
            r#"
                #[derive(Debug, Copy, Clone, TryFromPrimitive)]
                #[repr(usize)]
                pub enum TermKind {{
              "#
        }
    );

    out.inc_indent();
    for terminal in grammar.terminals() {
        geni!(out, "{} = {},\n", terminal.name, terminal.idx);
    }
    out.dec_indent();
    geni!(out, "}}\n\n");

    geni!(
        out,
        indoc! {
            r#"
                #[derive(Debug, Copy, Clone)]
                pub enum NonTermKind {{
              "#
        }
    );

    out.inc_indent();
    for nonterminal in grammar.nonterminals() {
        geni!(out, "{} = {},\n", nonterminal.name, nonterminal.idx);
    }
    out.dec_indent();
    geni!(out, "}}\n\n");

    geni!(
        out,
        indoc! {
            r#"
                #[derive(Debug)]
                pub enum Symbol {{
                    Terminal(Terminal),
                    NonTerminal(NonTerminal)
                }}

                #[derive(Debug)]
                pub enum Terminal {{
              "#
        }
    );

    out.inc_indent();
    for terminal in grammar.terminals() {
        geni!(
            out,
            "{}{},\n",
            terminal.name,
            if terminal.has_content {
                format!("({}_actions::{})", file_name, terminal.name)
            } else {
                "".to_string()
            }
        );
    }
    out.dec_indent();
    geni!(out, "}}\n\n");

    geni!(
        out,
        indoc! {
            r#"
                #[derive(Debug)]
                pub enum NonTerminal {{
                    EMPTY,
              "#
        }
    );

    out.inc_indent();
    for nonterminal in &grammar.nonterminals()[2..] {
        geni!(
            out,
            "{name}({file_name}_actions::{name}),\n",
            file_name = file_name,
            name = nonterminal.name
        );
    }
    out.dec_indent();
    geni!(out, "}}\n\n");

    geni!(
        out,
        indoc! {
            r#"
                #[derive(Copy, Clone, TryFromPrimitive)]
                #[repr(usize)]
                pub enum ProdKind {{
              "#
        }
    );

    out.inc_indent();
    for production in &grammar.productions()[1..] {
        let nt = production.nonterminal(grammar);
        geni!(
            out,
            "{}P{} = {},\n",
            nt.name,
            production.ntidx,
            production.idx
        );
    }
    out.dec_indent();
    geni!(out, "}}\n\n");

    Ok(())
}
