// copied required bits from tree-sitter-cli
// this stuff is understandably private

use anyhow::{anyhow, Result};
use serde::Deserialize;
use serde_json::{Map, Value};
use smallbitvec::SmallBitVec;

#[derive(Deserialize)]
#[serde(tag = "type")]
#[allow(non_camel_case_types)]
enum RuleJSON {
    ALIAS {
        content: Box<RuleJSON>,
        named: bool,
        value: String,
    },
    BLANK,
    STRING {
        value: String,
    },
    PATTERN {
        value: String,
    },
    SYMBOL {
        name: String,
    },
    CHOICE {
        members: Vec<RuleJSON>,
    },
    FIELD {
        name: String,
        content: Box<RuleJSON>,
    },
    SEQ {
        members: Vec<RuleJSON>,
    },
    REPEAT {
        content: Box<RuleJSON>,
    },
    REPEAT1 {
        content: Box<RuleJSON>,
    },
    PREC_DYNAMIC {
        value: i32,
        content: Box<RuleJSON>,
    },
    PREC_LEFT {
        value: PrecedenceValueJSON,
        content: Box<RuleJSON>,
    },
    PREC_RIGHT {
        value: PrecedenceValueJSON,
        content: Box<RuleJSON>,
    },
    PREC {
        value: PrecedenceValueJSON,
        content: Box<RuleJSON>,
    },
    TOKEN {
        content: Box<RuleJSON>,
    },
    IMMEDIATE_TOKEN {
        content: Box<RuleJSON>,
    },
}

#[derive(Deserialize)]
#[serde(untagged)]
enum PrecedenceValueJSON {
    Integer(i32),
    Name(String),
}

#[derive(Deserialize)]
pub(crate) struct GrammarJSON {
    pub(crate) name: String,
    rules: Map<String, Value>,
    #[serde(default)]
    precedences: Vec<Vec<RuleJSON>>,
    #[serde(default)]
    conflicts: Vec<Vec<String>>,
    #[serde(default)]
    externals: Vec<RuleJSON>,
    #[serde(default)]
    extras: Vec<RuleJSON>,
    #[serde(default)]
    inline: Vec<String>,
    #[serde(default)]
    supertypes: Vec<String>,
    word: Option<String>,
}

pub(crate) fn parse_grammar(input: &str) -> Result<InputGrammar> {
    let grammar_json: GrammarJSON = serde_json::from_str(&input)?;

    let mut variables = Vec::with_capacity(grammar_json.rules.len());
    for (name, value) in grammar_json.rules {
        variables.push(Variable {
            name: name.to_owned(),
            kind: VariableType::Named,
            rule: parse_rule(serde_json::from_value(value)?),
        })
    }

    let mut precedence_orderings = Vec::with_capacity(grammar_json.precedences.len());
    for list in grammar_json.precedences {
        let mut ordering = Vec::with_capacity(list.len());
        for entry in list {
            ordering.push(match entry {
                RuleJSON::STRING { value } => PrecedenceEntry::Name(value),
                RuleJSON::SYMBOL { name } => PrecedenceEntry::Symbol(name),
                _ => {
                    return Err(anyhow!(
                        "Invalid rule in precedences array. Only strings and symbols are allowed"
                    ))
                }
            })
        }
        precedence_orderings.push(ordering);
    }

    let extra_symbols = grammar_json.extras.into_iter().map(parse_rule).collect();
    let external_tokens = grammar_json.externals.into_iter().map(parse_rule).collect();

    Ok(InputGrammar {
        name: grammar_json.name,
        word_token: grammar_json.word,
        expected_conflicts: grammar_json.conflicts,
        supertype_symbols: grammar_json.supertypes,
        variables_to_inline: grammar_json.inline,
        precedence_orderings,
        variables,
        extra_symbols,
        external_tokens,
    })
}

fn parse_rule(json: RuleJSON) -> Rule {
    match json {
        RuleJSON::ALIAS {
            content,
            value,
            named,
        } => Rule::alias(parse_rule(*content), value, named),
        RuleJSON::BLANK => Rule::Blank,
        RuleJSON::STRING { value } => Rule::String(value),
        RuleJSON::PATTERN { value } => Rule::Pattern(value),
        RuleJSON::SYMBOL { name } => Rule::NamedSymbol(name),
        RuleJSON::CHOICE { members } => Rule::choice(members.into_iter().map(parse_rule).collect()),
        RuleJSON::FIELD { content, name } => Rule::field(name, parse_rule(*content)),
        RuleJSON::SEQ { members } => Rule::seq(members.into_iter().map(parse_rule).collect()),
        RuleJSON::REPEAT1 { content } => Rule::repeat(parse_rule(*content)),
        RuleJSON::REPEAT { content } => {
            Rule::choice(vec![Rule::repeat(parse_rule(*content)), Rule::Blank])
        }
        RuleJSON::PREC { value, content } => Rule::prec(value.into(), parse_rule(*content)),
        RuleJSON::PREC_LEFT { value, content } => {
            Rule::prec_left(value.into(), parse_rule(*content))
        }
        RuleJSON::PREC_RIGHT { value, content } => {
            Rule::prec_right(value.into(), parse_rule(*content))
        }
        RuleJSON::PREC_DYNAMIC { value, content } => {
            Rule::prec_dynamic(value, parse_rule(*content))
        }
        RuleJSON::TOKEN { content } => Rule::token(parse_rule(*content)),
        RuleJSON::IMMEDIATE_TOKEN { content } => Rule::immediate_token(parse_rule(*content)),
    }
}

impl Into<Precedence> for PrecedenceValueJSON {
    fn into(self) -> Precedence {
        match self {
            PrecedenceValueJSON::Integer(i) => Precedence::Integer(i),
            PrecedenceValueJSON::Name(i) => Precedence::Name(i),
        }
    }
}

use std::iter::FromIterator;
use std::{collections::HashMap, fmt};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum SymbolType {
    External,
    End,
    EndOfNonTerminalExtra,
    Terminal,
    NonTerminal,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum Associativity {
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct Alias {
    pub value: String,
    pub is_named: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Precedence {
    None,
    Integer(i32),
    Name(String),
}

pub(crate) type AliasMap = HashMap<Symbol, Alias>;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub(crate) struct MetadataParams {
    pub precedence: Precedence,
    pub dynamic_precedence: i32,
    pub associativity: Option<Associativity>,
    pub is_token: bool,
    pub is_string: bool,
    pub is_active: bool,
    pub is_main_token: bool,
    pub alias: Option<Alias>,
    pub field_name: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct Symbol {
    pub kind: SymbolType,
    pub index: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Rule {
    Blank,
    String(String),
    Pattern(String),
    NamedSymbol(String),
    Symbol(Symbol),
    Choice(Vec<Rule>),
    Metadata {
        params: MetadataParams,
        rule: Box<Rule>,
    },
    Repeat(Box<Rule>),
    Seq(Vec<Rule>),
}

// Because tokens are represented as small (~400 max) unsigned integers,
// sets of tokens can be efficiently represented as bit vectors with each
// index correspoding to a token, and each value representing whether or not
// the token is present in the set.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct TokenSet {
    terminal_bits: SmallBitVec,
    external_bits: SmallBitVec,
    eof: bool,
    end_of_nonterminal_extra: bool,
}

impl Rule {
    pub fn field(name: String, content: Rule) -> Self {
        add_metadata(content, move |params| {
            params.field_name = Some(name);
        })
    }

    pub fn alias(content: Rule, value: String, is_named: bool) -> Self {
        add_metadata(content, move |params| {
            params.alias = Some(Alias { is_named, value });
        })
    }

    pub fn token(content: Rule) -> Self {
        add_metadata(content, |params| {
            params.is_token = true;
        })
    }

    pub fn immediate_token(content: Rule) -> Self {
        add_metadata(content, |params| {
            params.is_token = true;
            params.is_main_token = true;
        })
    }

    pub fn prec(value: Precedence, content: Rule) -> Self {
        add_metadata(content, |params| {
            params.precedence = value;
        })
    }

    pub fn prec_left(value: Precedence, content: Rule) -> Self {
        add_metadata(content, |params| {
            params.associativity = Some(Associativity::Left);
            params.precedence = value;
        })
    }

    pub fn prec_right(value: Precedence, content: Rule) -> Self {
        add_metadata(content, |params| {
            params.associativity = Some(Associativity::Right);
            params.precedence = value;
        })
    }

    pub fn prec_dynamic(value: i32, content: Rule) -> Self {
        add_metadata(content, |params| {
            params.dynamic_precedence = value;
        })
    }

    pub fn repeat(rule: Rule) -> Self {
        Rule::Repeat(Box::new(rule))
    }

    pub fn choice(rules: Vec<Rule>) -> Self {
        let mut elements = Vec::with_capacity(rules.len());
        for rule in rules {
            choice_helper(&mut elements, rule);
        }
        Rule::Choice(elements)
    }

    pub fn seq(rules: Vec<Rule>) -> Self {
        Rule::Seq(rules)
    }
}

impl Alias {
    pub fn kind(&self) -> VariableType {
        if self.is_named {
            VariableType::Named
        } else {
            VariableType::Anonymous
        }
    }
}

impl Precedence {
    pub fn is_none(&self) -> bool {
        matches!(self, Precedence::None)
    }
}

#[cfg(test)]
impl Rule {
    pub fn terminal(index: usize) -> Self {
        Rule::Symbol(Symbol::terminal(index))
    }

    pub fn non_terminal(index: usize) -> Self {
        Rule::Symbol(Symbol::non_terminal(index))
    }

    pub fn external(index: usize) -> Self {
        Rule::Symbol(Symbol::external(index))
    }

    pub fn named(name: &'static str) -> Self {
        Rule::NamedSymbol(name.to_string())
    }

    pub fn string(value: &'static str) -> Self {
        Rule::String(value.to_string())
    }

    pub fn pattern(value: &'static str) -> Self {
        Rule::Pattern(value.to_string())
    }
}

impl Symbol {
    pub fn is_terminal(&self) -> bool {
        self.kind == SymbolType::Terminal
    }

    pub fn is_non_terminal(&self) -> bool {
        self.kind == SymbolType::NonTerminal
    }

    pub fn is_external(&self) -> bool {
        self.kind == SymbolType::External
    }

    pub fn is_eof(&self) -> bool {
        self.kind == SymbolType::End
    }

    pub fn non_terminal(index: usize) -> Self {
        Symbol {
            kind: SymbolType::NonTerminal,
            index,
        }
    }

    pub fn terminal(index: usize) -> Self {
        Symbol {
            kind: SymbolType::Terminal,
            index,
        }
    }

    pub fn external(index: usize) -> Self {
        Symbol {
            kind: SymbolType::External,
            index,
        }
    }

    pub fn end() -> Self {
        Symbol {
            kind: SymbolType::End,
            index: 0,
        }
    }

    pub fn end_of_nonterminal_extra() -> Self {
        Symbol {
            kind: SymbolType::EndOfNonTerminalExtra,
            index: 0,
        }
    }
}

impl From<Symbol> for Rule {
    fn from(symbol: Symbol) -> Self {
        Rule::Symbol(symbol)
    }
}

impl TokenSet {
    pub fn new() -> Self {
        Self {
            terminal_bits: SmallBitVec::new(),
            external_bits: SmallBitVec::new(),
            eof: false,
            end_of_nonterminal_extra: false,
        }
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = Symbol> + 'a {
        self.terminal_bits
            .iter()
            .enumerate()
            .filter_map(|(i, value)| {
                if value {
                    Some(Symbol::terminal(i))
                } else {
                    None
                }
            })
            .chain(
                self.external_bits
                    .iter()
                    .enumerate()
                    .filter_map(|(i, value)| {
                        if value {
                            Some(Symbol::external(i))
                        } else {
                            None
                        }
                    }),
            )
            .chain(if self.eof { Some(Symbol::end()) } else { None })
            .chain(if self.end_of_nonterminal_extra {
                Some(Symbol::end_of_nonterminal_extra())
            } else {
                None
            })
    }

    pub fn terminals<'a>(&'a self) -> impl Iterator<Item = Symbol> + 'a {
        self.terminal_bits
            .iter()
            .enumerate()
            .filter_map(|(i, value)| {
                if value {
                    Some(Symbol::terminal(i))
                } else {
                    None
                }
            })
    }

    pub fn contains(&self, symbol: &Symbol) -> bool {
        match symbol.kind {
            SymbolType::NonTerminal => panic!("Cannot store non-terminals in a TokenSet"),
            SymbolType::Terminal => self.terminal_bits.get(symbol.index).unwrap_or(false),
            SymbolType::External => self.external_bits.get(symbol.index).unwrap_or(false),
            SymbolType::End => self.eof,
            SymbolType::EndOfNonTerminalExtra => self.end_of_nonterminal_extra,
        }
    }

    pub fn contains_terminal(&self, index: usize) -> bool {
        self.terminal_bits.get(index).unwrap_or(false)
    }

    pub fn insert(&mut self, other: Symbol) {
        let vec = match other.kind {
            SymbolType::NonTerminal => panic!("Cannot store non-terminals in a TokenSet"),
            SymbolType::Terminal => &mut self.terminal_bits,
            SymbolType::External => &mut self.external_bits,
            SymbolType::End => {
                self.eof = true;
                return;
            }
            SymbolType::EndOfNonTerminalExtra => {
                self.end_of_nonterminal_extra = true;
                return;
            }
        };
        if other.index >= vec.len() {
            vec.resize(other.index + 1, false);
        }
        vec.set(other.index, true);
    }

    pub fn remove(&mut self, other: &Symbol) -> bool {
        let vec = match other.kind {
            SymbolType::NonTerminal => panic!("Cannot store non-terminals in a TokenSet"),
            SymbolType::Terminal => &mut self.terminal_bits,
            SymbolType::External => &mut self.external_bits,
            SymbolType::End => {
                return if self.eof {
                    self.eof = false;
                    true
                } else {
                    false
                }
            }
            SymbolType::EndOfNonTerminalExtra => {
                return if self.end_of_nonterminal_extra {
                    self.end_of_nonterminal_extra = false;
                    true
                } else {
                    false
                };
            }
        };
        if other.index < vec.len() {
            if vec[other.index] {
                vec.set(other.index, false);
                return true;
            }
        }
        false
    }

    pub fn is_empty(&self) -> bool {
        !self.eof
            && !self.end_of_nonterminal_extra
            && !self.terminal_bits.iter().any(|a| a)
            && !self.external_bits.iter().any(|a| a)
    }

    pub fn insert_all_terminals(&mut self, other: &TokenSet) -> bool {
        let mut result = false;
        if other.terminal_bits.len() > self.terminal_bits.len() {
            self.terminal_bits.resize(other.terminal_bits.len(), false);
        }
        for (i, element) in other.terminal_bits.iter().enumerate() {
            if element {
                result |= !self.terminal_bits[i];
                self.terminal_bits.set(i, element);
            }
        }
        result
    }

    fn insert_all_externals(&mut self, other: &TokenSet) -> bool {
        let mut result = false;
        if other.external_bits.len() > self.external_bits.len() {
            self.external_bits.resize(other.external_bits.len(), false);
        }
        for (i, element) in other.external_bits.iter().enumerate() {
            if element {
                result |= !self.external_bits[i];
                self.external_bits.set(i, element);
            }
        }
        result
    }

    pub fn insert_all(&mut self, other: &TokenSet) -> bool {
        let mut result = false;
        if other.eof {
            result |= !self.eof;
            self.eof = true;
        }
        if other.end_of_nonterminal_extra {
            result |= !self.end_of_nonterminal_extra;
            self.end_of_nonterminal_extra = true;
        }
        result |= self.insert_all_terminals(other);
        result |= self.insert_all_externals(other);
        result
    }
}

impl FromIterator<Symbol> for TokenSet {
    fn from_iter<T: IntoIterator<Item = Symbol>>(iter: T) -> Self {
        let mut result = Self::new();
        for symbol in iter {
            result.insert(symbol);
        }
        result
    }
}

fn add_metadata<T: FnOnce(&mut MetadataParams)>(input: Rule, f: T) -> Rule {
    match input {
        Rule::Metadata { rule, mut params } if !params.is_token => {
            f(&mut params);
            Rule::Metadata { rule, params }
        }
        _ => {
            let mut params = MetadataParams::default();
            f(&mut params);
            Rule::Metadata {
                rule: Box::new(input),
                params,
            }
        }
    }
}

fn choice_helper(result: &mut Vec<Rule>, rule: Rule) {
    match rule {
        Rule::Choice(elements) => {
            for element in elements {
                choice_helper(result, element);
            }
        }
        _ => {
            if !result.contains(&rule) {
                result.push(rule);
            }
        }
    }
}

impl fmt::Display for Precedence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Precedence::Integer(i) => write!(f, "{}", i),
            Precedence::Name(s) => write!(f, "'{}'", s),
            Precedence::None => write!(f, "none"),
        }
    }
}

impl Default for Precedence {
    fn default() -> Self {
        Precedence::None
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum VariableType {
    Hidden,
    Auxiliary,
    Anonymous,
    Named,
}

// Input grammar

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Variable {
    pub name: String,
    pub kind: VariableType,
    pub rule: Rule,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum PrecedenceEntry {
    Name(String),
    Symbol(String),
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct InputGrammar {
    pub name: String,
    pub variables: Vec<Variable>,
    pub extra_symbols: Vec<Rule>,
    pub expected_conflicts: Vec<Vec<String>>,
    pub precedence_orderings: Vec<Vec<PrecedenceEntry>>,
    pub external_tokens: Vec<Rule>,
    pub variables_to_inline: Vec<String>,
    pub supertype_symbols: Vec<String>,
    pub word_token: Option<String>,
}

// Extracted lexical grammar

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct LexicalVariable {
    pub name: String,
    pub kind: VariableType,
    pub implicit_precedence: i32,
    pub start_state: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct ProductionStep {
    pub symbol: Symbol,
    pub precedence: Precedence,
    pub associativity: Option<Associativity>,
    pub alias: Option<Alias>,
    pub field_name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Production {
    pub steps: Vec<ProductionStep>,
    pub dynamic_precedence: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SyntaxVariable {
    pub name: String,
    pub kind: VariableType,
    pub productions: Vec<Production>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ExternalToken {
    pub name: String,
    pub kind: VariableType,
    pub corresponding_internal_token: Option<Symbol>,
}

#[derive(Debug, Default)]
pub(crate) struct SyntaxGrammar {
    pub variables: Vec<SyntaxVariable>,
    pub extra_symbols: Vec<Symbol>,
    pub expected_conflicts: Vec<Vec<Symbol>>,
    pub external_tokens: Vec<ExternalToken>,
    pub supertype_symbols: Vec<Symbol>,
    pub variables_to_inline: Vec<Symbol>,
    pub word_token: Option<Symbol>,
    pub precedence_orderings: Vec<Vec<PrecedenceEntry>>,
}

impl fmt::Display for PrecedenceEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PrecedenceEntry::Name(n) => write!(f, "'{}'", n),
            PrecedenceEntry::Symbol(s) => write!(f, "$.{}", s),
        }
    }
}
