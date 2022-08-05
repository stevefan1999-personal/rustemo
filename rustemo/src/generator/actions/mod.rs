//! Parser/generator for actions file
//!
//! Provides default semantics actions implementation but allow for manual
//! changes.

use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
};

use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span};
use syn::{self, parse_quote};

use crate::{error::{Error, Result}, settings::{Settings, GenActions}, grammar::Terminal};
use crate::grammar::{Grammar, NonTerminal};

mod rule;
mod production;

pub(crate) trait ActionsGenerator {
    fn terminal_type(&self, terminal: &Terminal) -> syn::Item {
        let type_name_ident = Ident::new(&terminal.name, Span::call_site());
        parse_quote! {
            pub type #type_name_ident = String;
        }
    }
    fn terminal_action(&self, terminal: &Terminal) -> syn::Item {
        let type_name_ident = Ident::new(&terminal.name, Span::call_site());
        let action_name = terminal.name.to_case(Case::Snake);
        let action_name_ident = Ident::new(&action_name, Span::call_site());
        parse_quote! {
            pub fn #action_name_ident<'a>(token: Token<&'a str>) -> #type_name_ident {
                token.value.into()
            }
        }
    }

    /// Create Rust types for the given non-terminal.
    fn nonterminal_types(&self, nonterminal: &NonTerminal) -> Vec<(String, syn::Item)>;

    /// Creates an action function for each production of the given non-terminal.
    fn nonterminal_actions(&self, nonterminal: &NonTerminal,) -> Vec<(String, syn::Item)>;
}

pub(crate) fn generate_parser_actions<F>(
    grammar: &Grammar,
    grammar_path: F,
    settings: &Settings,
) -> Result<()>
where
    F: AsRef<Path> + core::fmt::Debug,
{
    let mut file_name = grammar_path
        .as_ref()
        .file_stem()
        .ok_or(Error::Error("Invalid file name.".into()))?
        .to_os_string();
    file_name.push("_actions.rs");
    let action_file =
        PathBuf::from(grammar_path.as_ref()).with_file_name(file_name);

    let mut ast = if action_file.exists() {
        log!("Parsing action file with Syn: {:?}", action_file);
        syn::parse_file(&std::fs::read_to_string(&action_file)?)?
    } else {
        // Create new empty file with common uses statements.
        log!("Action file not found. Creating: {:?}", action_file);
        parse_quote! {
            ///! This file is maintained by rustemo but can be modified manually.
            ///! All manual changes will be preserved except non-doc comments.
            use rustemo_rt::lexer::Token;
        }
    };

    // Collect function and type names
    let mut type_names = BTreeSet::new();
    let mut action_names = BTreeSet::new();
    for item in &ast.items {
        match item {
            // Used for grammar rules of the form:
            // NT: First | Second | Third;
            // TODO: Are non-terminals allowed in the RHS?
            syn::Item::Enum(e) => {
                let type_name = e.ident.to_string();
                log!("Found enum type '{}'", type_name);
                type_names.insert(e.ident.to_string())
            }
            syn::Item::Struct(e) => {
                let type_name = e.ident.to_string();
                log!("Found struct type '{}'", type_name);
                type_names.insert(e.ident.to_string())
            }
            // Used for actions
            syn::Item::Fn(f) => {
                let type_name = f.sig.ident.to_string();
                log!("Found action function '{}'", type_name);
                action_names.insert(type_name)
            }
            syn::Item::Type(t) => {
                let type_name = t.ident.to_string();
                log!("Found type '{}'", type_name);
                type_names.insert(type_name)
            }
            _ => false,
        };
    }

    let generator: Box<dyn ActionsGenerator> = match settings.gen_actions {
        GenActions::RuleBased => rule::RuleActionsGenerator::new(grammar),
        GenActions::ProductionBased => production::ProductionActionsGenerator::new(grammar),
    };

    // Generate types and actions for terminals
    for terminal in grammar.terminals().iter().filter(|t| t.has_content) {
        // Add terminal types
        let type_name = &terminal.name;
        if !type_names.contains(type_name) {
            log!("Create type for terminal '{type_name}'.");
            ast.items.push(generator.terminal_type(terminal));
        }
        // Add terminal actions
        let action_name = terminal.name.to_case(Case::Snake);
        if !action_names.contains(&action_name) {
            log!("Create action function for terminal '{type_name}'.");
            ast.items.push(generator.terminal_action(terminal))
        }
    }

    // Generate types and actions for non-terminals
    for nonterminal in grammar.nonterminals().iter().filter(|&nt| {
        let nt_symbol = grammar.nonterm_to_symbol_index(nt.idx);
        nt_symbol != grammar.augmented_index && nt_symbol != grammar.empty_index
    }) {
        // Add non-terminal type
        for (type_name, ty) in generator.nonterminal_types(nonterminal).into_iter() {
            if !type_names.contains(&type_name) {
                log!("Creating type for non-terminal '{type_name}'.");
                ast.items.push(ty);
            }
        }

        // Add non-terminal actions
        for (action_name, action) in generator.nonterminal_actions(nonterminal) {
            if !action_names.contains(&action_name) {
                log!("Creating action '{action_name}'.");
                ast.items.push(action);
            }
        }
    }

    log!("Writing action file {:?}", action_file);
    std::fs::write(action_file, prettyplease::unparse(&ast))?;

    Ok(())
}

