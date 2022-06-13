use std::collections::{BTreeMap, BTreeSet};
use std::io::{Read, Write};
use std::process::{Command, Stdio};

use convert_case::{Case, Casing};
use expect_test::expect;
use quote::__private::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::gen::copy::MetadataParams;

use self::copy::{parse_grammar, InputGrammar, Symbol, Variable, VariableType};

mod copy;
mod test_visit;

#[test]
fn generate_psql_nodes() -> std::io::Result<()> {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../tree-sitter-sql/src/grammar.json");
    let grammar = parse_grammar(&std::fs::read_to_string(path)?).unwrap();
    let out = generate_nodes(grammar);
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/generated.rs");
    let mut file = std::fs::File::create(path)?;
    file.write_all(out.as_bytes())?;
    Ok(())
}

fn generate_grammar(grammar: &str) -> String {
    let grammar = parse_grammar(grammar).unwrap();
    generate_nodes(grammar)
}

fn generate_nodes(grammar: InputGrammar) -> String {
    Gen::default().generate(grammar)
}

// basically the same as `copy::Rule` but easier to pattern match on
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Rule {
    Blank,
    String(String),
    Pattern(String),
    NamedSymbol(String),
    Symbol(Symbol),
    Choice(Box<[Rule]>),
    Repeat(Box<Rule>),
    Seq(Box<[Rule]>),
}

impl Rule {
    fn convert_slice(rules: impl IntoIterator<Item = copy::Rule>) -> Box<[Self]> {
        rules.into_iter().map(Into::into).collect()
    }
}

impl From<copy::Rule> for Rule {
    fn from(rule: copy::Rule) -> Self {
        match rule {
            copy::Rule::Blank => Rule::Blank,
            copy::Rule::String(s) => Rule::String(s),
            copy::Rule::Pattern(p) => Rule::Pattern(p),
            copy::Rule::NamedSymbol(s) => Rule::NamedSymbol(s),
            copy::Rule::Symbol(s) => Rule::Symbol(s),
            copy::Rule::Choice(rules) => Rule::Choice(Rule::convert_slice(rules)),
            copy::Rule::Metadata { rule, .. } => (*rule).into(),
            copy::Rule::Repeat(rule) => Rule::Repeat(Box::new((*rule).into())),
            copy::Rule::Seq(rules) => Rule::Seq(Rule::convert_slice(rules)),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Cardinality {
    One,
    Many,
}

#[derive(Default)]
struct Gen {
    s: String,
    enums: BTreeMap<Ident, EnumData>,
    method_sets: BTreeMap<Ident, BTreeMap<String, Cardinality>>,
    syntax_kinds: BTreeSet<(String, SyntaxKindSort)>,
}

#[derive(Debug)]
struct EnumData {
    variants: Vec<Ident>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum SyntaxKindSort {
    Sym,
    Kw,
}

fn rustfmt(code: &str) -> String {
    let child = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.unwrap().write_all(code.as_bytes()).unwrap();
    let mut s = String::with_capacity(code.len());
    child.stdout.unwrap().read_to_string(&mut s).unwrap();
    s
}

impl Gen {
    pub fn generate(mut self, grammar: InputGrammar) -> String {
        self.gen(grammar);
        self.generate_visitor();
        self.generate_syntax_kinds();

        rustfmt(&self.s)
    }

    fn generate_visitor(&mut self) {
        let visit_methods = self.method_sets.iter().map(|(name, fields)| {
            let snake = name.to_string().to_case(Case::Snake);
            let param = format_ident!("r#{}", snake);
            let f = format_ident!("visit_{}", snake);

            match self.enums.get(&name) {
                None => {
                    let stmts = fields.iter().map(|(field, &cardinality)| {
                        let snake = format_ident!("{}", field.to_case(Case::Snake));
                        let raw_field = format_ident!("r#{snake}");
                        let fields = format_ident!("{snake}s");
                        let visit = format_ident!("visit_{}", snake);

                        if field.ends_with("Kw") {
                            assert_eq!(cardinality, Cardinality::One);
                            return quote! {
                                if let Some(kw) = #param.#snake() {
                                    self.visit_kw(kw);
                                }
                            };
                        }

                        if !self
                            .method_sets
                            .contains_key(&format_ident!("{}", field.to_case(Case::Pascal)))
                        {
                            return quote! {};
                        }
                        match cardinality {
                            Cardinality::One => {
                                quote! {
                                    for #raw_field in #param.#raw_field() {
                                        self.#visit(#raw_field);
                                    }
                                }
                            }
                            Cardinality::Many => {
                                quote! {
                                    for #raw_field in #param.#fields() {
                                        self.#visit(#raw_field);
                                    }
                                }
                            }
                        }
                    });

                    quote! {
                        fn #f(&mut self, #param: #name) {
                            #(#stmts)*
                        }
                    }
                }
                Some(data) => {
                    let variants = &data.variants;
                    let visits = variants.iter().map(|field| {
                        format_ident!("visit_{}", field.to_string().to_case(Case::Snake))
                    });
                    quote! {
                        fn #f(&mut self, #param: #name) {
                            match #param {
                                #(#name::#variants(node) => self.#visits(node),)*
                            }
                        }
                    }
                }
            }
        });

        let stream = quote! {
            pub trait Visitor {
                fn visit_kw(&mut self, kw: SyntaxToken) {}

                #(#visit_methods)*
            }
        };

        self.push(stream);
    }

    fn generate_syntax_kinds(&mut self) {
        let variants = self.syntax_kinds.iter().map(|(kind, sort)| match sort {
            SyntaxKindSort::Sym => format_ident!("{}", kind.to_case(Case::Pascal)),
            SyntaxKindSort::Kw => format_ident!("{}Kw", kind.to_case(Case::Pascal)),
        });

        let cases = self.syntax_kinds.iter().map(|(kind, sort)| {
            let (kind, variant) = match sort {
                SyntaxKindSort::Sym => (
                    kind.to_case(Case::Snake),
                    format_ident!("{}", kind.to_case(Case::Pascal)),
                ),
                SyntaxKindSort::Kw => (
                    kind.to_case(Case::ScreamingSnake),
                    format_ident!("{}Kw", kind.to_case(Case::Pascal)),
                ),
            };
            quote!(#kind => Ok(Self::#variant))
        });

        self.push(quote! {
            #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
            #[repr(u16)]
            pub enum SyntaxKind {
                #(#variants,)*
                Token, // catchall kind for tokens
                Whitespace, // catchall for arbitrary whitespace
                Err,
            }

            impl TryFrom<&'static str> for SyntaxKind {
                type Error = ();
                fn try_from(s: &'static str) -> Result<Self, Self::Error> {
                    match s {
                        #(#cases,)*
                        "ERROR" => Ok(Self::Err),
                        _ => Err(())
                    }
                }
            }
        });
    }

    fn push(&mut self, stream: TokenStream) {
        self.push_str(&format!("{stream}"))
    }

    fn push_str(&mut self, string: &str) {
        self.s.push_str(string);
    }

    fn gen_var(&mut self, v: Variable) {
        let name = format_ident!("{}", v.name.to_case(Case::Pascal));
        self.method_sets.entry(name.clone()).or_default();
        let stream = match &v.rule {
            copy::Rule::Choice(choices)
                if choices
                    .iter()
                    .all(|rule| matches!(rule, copy::Rule::NamedSymbol(_))) =>
            {
                let variants = choices
                    .iter()
                    .map(|rule| match rule {
                        copy::Rule::NamedSymbol(sym) => {
                            format_ident!("{}", sym.to_case(Case::Pascal))
                        }
                        _ => unreachable!(),
                    })
                    .collect::<Vec<Ident>>();

                let tokens = quote! {
                    pub enum #name {
                        #(#variants(#variants)),*
                    }

                    impl Node for #name {
                        fn can_cast(kind: SyntaxKind) -> bool {
                            matches!(kind, #(SyntaxKind::#variants)|*)
                        }

                        fn cast(syntax: SyntaxNode) -> Option<Self> {
                            match syntax.kind() {
                                #(SyntaxKind::#variants => Some(#name::#variants(#variants(syntax))),)*
                                _ => None,
                            }
                        }

                        fn syntax(&self) -> &SyntaxNode {
                            match self {
                                #(Self::#variants(node) => node.syntax()),*
                            }
                        }
                    }
                };
                assert!(
                    self.enums
                        .insert(name.clone(), EnumData { variants })
                        .is_none(),
                    "duplicate enum {name}"
                );
                tokens
            }
            _ => quote! {
                #[derive(Debug, Clone, PartialEq, Eq, Hash)]
                pub struct #name(pub(crate) SyntaxNode);

                impl Node for #name {
                    fn can_cast(kind: SyntaxKind) -> bool {
                        kind == SyntaxKind::#name
                    }

                    fn cast(syntax: SyntaxNode) -> Option<Self> {
                        Self::can_cast(syntax.kind()).then(|| Self(syntax))
                    }

                    fn syntax(&self) -> &SyntaxNode {
                        &self.0
                    }
                }
            },
        };
        self.push(stream);
        self.gen_rule(&name, v.rule.into());
    }

    fn gen_rule(&mut self, name: &Ident, rule: Rule) {
        if self.gen_list(name, &rule) {
            return;
        }

        match rule {
            Rule::Blank => {}
            Rule::Pattern(pat) => {
                // look for patterns of the form `[fF][aA][lL][sS][eE]`
                // which represent case insensitive keywords
                if pat.len() % 4 != 0 {
                    return;
                }
                let kw = pat
                    .as_bytes()
                    .chunks_exact(4)
                    .filter_map(|chunk| {
                        if chunk[0] != b'[' || chunk[3] != b']' || !chunk[1].is_ascii_alphabetic() {
                            return None;
                        }
                        Some(chunk[1] as char)
                    })
                    .collect::<String>();

                if kw.is_empty() {
                    return;
                }

                let kind = format!("{}Kw", kw.to_case(Case::Pascal));
                self.syntax_kinds.insert((kw, SyntaxKindSort::Kw));
                let f = format_ident!("{}", kind.to_case(Case::Snake));
                let variant = format_ident!("{kind}");
                let methods = self.method_sets.entry(name.clone()).or_default();
                if !methods.contains_key(&kind) {
                    let stream = quote! {
                        impl #name {
                            pub fn #f(&self) -> Option<SyntaxToken> {
                                self.token(SyntaxKind::#variant)
                            }
                        }
                    };
                    methods.insert(kind, Cardinality::One);
                    self.push(stream);
                }
            }
            Rule::String(_) | Rule::Symbol(_) => {}
            Rule::NamedSymbol(sym) => {
                let f = format_ident!("r#{}", sym.to_case(Case::Snake));
                let ty = format_ident!("{}", sym.to_case(Case::Pascal));
                let methods = self.method_sets.entry(name.clone()).or_default();
                if !methods.contains_key(&sym) {
                    let stream = quote! {
                        impl #name {
                            pub fn #f(&self) -> Option<#ty> {
                                self.child()
                            }
                        }
                    };
                    methods.insert(sym, Cardinality::One);
                    self.push(stream);
                }
            }
            Rule::Repeat(rule) => match *rule {
                Rule::NamedSymbol(sym) => self.gen_children_accessor(name, &sym),
                rule => self.gen_rule(name, rule),
            },
            Rule::Choice(rules) | Rule::Seq(rules) => rules
                .into_vec()
                .into_iter()
                .for_each(|rule| self.gen_rule(name, rule)),
        }
    }

    fn gen_children_accessor(&mut self, name: &Ident, sym: &str) {
        let methods = self.method_sets.entry(name.clone()).or_default();
        match methods.get(sym) {
            Some(_) => todo!(),
            None => methods.insert(sym.to_owned(), Cardinality::Many),
        };

        let pluralized = format_ident!("{}s", sym.to_case(Case::Snake));
        let ty = format_ident!("{}", sym.to_case(Case::Pascal));
        self.push(quote! {
            impl #name {
                pub fn #pluralized(&self) -> impl Iterator<Item = #ty> {
                    self.children()
                }
            }
        });
    }

    // comma separated or semicolon separated for now
    // there may be patterns that aren't captured by the ones below
    fn gen_list(&mut self, name: &Ident, rule: &Rule) -> bool {
        match rule {
            Rule::Seq(
                box [Rule::NamedSymbol(x), Rule::Choice(
                    box [Rule::Repeat(box Rule::Seq(box [Rule::String(sep), Rule::NamedSymbol(y)])), Rule::Blank],
                )],
            ) if x == y && !x.starts_with('_') && [";", ","].contains(&sep.as_str()) => {
                self.gen_children_accessor(name, x);
                true
            }
            _ => false,
        }
        // example
        // rules = [
        //     Seq(
        //         [
        //             NamedSymbol(
        //                 "pair",
        //             ),
        //             Choice(
        //                 [
        //                     Repeat(
        //                         Seq(
        //                             [
        //                                 String(
        //                                     ",",
        //                                 ),
        //                                 NamedSymbol(
        //                                     "pair",
        //                                 ),
        //                             ],
        //                         ),
        //                     ),
        //                     Blank,
        //                 ],
        //             ),
        //         ],
        //     ),
        //     Blank,
        // ]
    }

    fn gen(&mut self, grammar: InputGrammar) {
        self.push_str("//! generated, do not edit\n");
        self.push(quote! {
            #![allow(unused)]
            use crate::node::*;

            impl rowan::Language for Sql {
                type Kind = SyntaxKind;

                fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
                    unsafe { std::mem::transmute(raw) }
                }

                fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
                    unsafe { std::mem::transmute(kind) }
                }
            }
        });

        grammar
            .variables
            .into_iter()
            .filter(|v| v.kind == VariableType::Named)
            .for_each(|v| {
                if let copy::Rule::Metadata {
                    params:
                        MetadataParams {
                            alias: Some(alias), ..
                        },
                    rule: _,
                } = &v.rule
                {
                    self.syntax_kinds
                        .insert((alias.value.clone(), SyntaxKindSort::Sym));
                } else {
                    self.syntax_kinds
                        .insert((v.name.clone(), SyntaxKindSort::Sym));
                }
                self.gen_var(v);
            });
    }
}

#[test]
fn test_generate_nodes() {
    let out = generate_grammar(TEST_GRAMMAR);
    expect![[r#"
        //! generated, do not edit
        #![allow(unused)]
        use crate::node::*;
        impl rowan::Language for Sql {
            type Kind = SyntaxKind;
            fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
                unsafe { std::mem::transmute(raw) }
            }
            fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
                unsafe { std::mem::transmute(kind) }
            }
        }
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct Array(pub(crate) SyntaxNode);
        impl Node for Array {
            fn can_cast(kind: SyntaxKind) -> bool {
                kind == SyntaxKind::Array
            }
            fn cast(syntax: SyntaxNode) -> Option<Self> {
                Self::can_cast(syntax.kind()).then(|| Self(syntax))
            }
            fn syntax(&self) -> &SyntaxNode {
                &self.0
            }
        }
        impl Array {
            pub fn values(&self) -> impl Iterator<Item = Value> {
                self.children()
            }
        }
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct Document(pub(crate) SyntaxNode);
        impl Node for Document {
            fn can_cast(kind: SyntaxKind) -> bool {
                kind == SyntaxKind::Document
            }
            fn cast(syntax: SyntaxNode) -> Option<Self> {
                Self::can_cast(syntax.kind()).then(|| Self(syntax))
            }
            fn syntax(&self) -> &SyntaxNode {
                &self.0
            }
        }
        impl Document {
            pub fn r#value(&self) -> Option<Value> {
                self.child()
            }
        }
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct False(pub(crate) SyntaxNode);
        impl Node for False {
            fn can_cast(kind: SyntaxKind) -> bool {
                kind == SyntaxKind::False
            }
            fn cast(syntax: SyntaxNode) -> Option<Self> {
                Self::can_cast(syntax.kind()).then(|| Self(syntax))
            }
            fn syntax(&self) -> &SyntaxNode {
                &self.0
            }
        }
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct Null(pub(crate) SyntaxNode);
        impl Node for Null {
            fn can_cast(kind: SyntaxKind) -> bool {
                kind == SyntaxKind::Null
            }
            fn cast(syntax: SyntaxNode) -> Option<Self> {
                Self::can_cast(syntax.kind()).then(|| Self(syntax))
            }
            fn syntax(&self) -> &SyntaxNode {
                &self.0
            }
        }
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct Number(pub(crate) SyntaxNode);
        impl Node for Number {
            fn can_cast(kind: SyntaxKind) -> bool {
                kind == SyntaxKind::Number
            }
            fn cast(syntax: SyntaxNode) -> Option<Self> {
                Self::can_cast(syntax.kind()).then(|| Self(syntax))
            }
            fn syntax(&self) -> &SyntaxNode {
                &self.0
            }
        }
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct Object(pub(crate) SyntaxNode);
        impl Node for Object {
            fn can_cast(kind: SyntaxKind) -> bool {
                kind == SyntaxKind::Object
            }
            fn cast(syntax: SyntaxNode) -> Option<Self> {
                Self::can_cast(syntax.kind()).then(|| Self(syntax))
            }
            fn syntax(&self) -> &SyntaxNode {
                &self.0
            }
        }
        impl Object {
            pub fn pairs(&self) -> impl Iterator<Item = Pair> {
                self.children()
            }
        }
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct Pair(pub(crate) SyntaxNode);
        impl Node for Pair {
            fn can_cast(kind: SyntaxKind) -> bool {
                kind == SyntaxKind::Pair
            }
            fn cast(syntax: SyntaxNode) -> Option<Self> {
                Self::can_cast(syntax.kind()).then(|| Self(syntax))
            }
            fn syntax(&self) -> &SyntaxNode {
                &self.0
            }
        }
        impl Pair {
            pub fn r#string(&self) -> Option<String> {
                self.child()
            }
        }
        impl Pair {
            pub fn r#number(&self) -> Option<Number> {
                self.child()
            }
        }
        impl Pair {
            pub fn r#value(&self) -> Option<Value> {
                self.child()
            }
        }
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct String(pub(crate) SyntaxNode);
        impl Node for String {
            fn can_cast(kind: SyntaxKind) -> bool {
                kind == SyntaxKind::String
            }
            fn cast(syntax: SyntaxNode) -> Option<Self> {
                Self::can_cast(syntax.kind()).then(|| Self(syntax))
            }
            fn syntax(&self) -> &SyntaxNode {
                &self.0
            }
        }
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct True(pub(crate) SyntaxNode);
        impl Node for True {
            fn can_cast(kind: SyntaxKind) -> bool {
                kind == SyntaxKind::True
            }
            fn cast(syntax: SyntaxNode) -> Option<Self> {
                Self::can_cast(syntax.kind()).then(|| Self(syntax))
            }
            fn syntax(&self) -> &SyntaxNode {
                &self.0
            }
        }
        pub enum Value {
            Object(Object),
            Array(Array),
            Number(Number),
            String(String),
            True(True),
            False(False),
            Null(Null),
        }
        impl Node for Value {
            fn can_cast(kind: SyntaxKind) -> bool {
                matches!(
                    kind,
                    SyntaxKind::Object
                        | SyntaxKind::Array
                        | SyntaxKind::Number
                        | SyntaxKind::String
                        | SyntaxKind::True
                        | SyntaxKind::False
                        | SyntaxKind::Null
                )
            }
            fn cast(syntax: SyntaxNode) -> Option<Self> {
                match syntax.kind() {
                    SyntaxKind::Object => Some(Value::Object(Object(syntax))),
                    SyntaxKind::Array => Some(Value::Array(Array(syntax))),
                    SyntaxKind::Number => Some(Value::Number(Number(syntax))),
                    SyntaxKind::String => Some(Value::String(String(syntax))),
                    SyntaxKind::True => Some(Value::True(True(syntax))),
                    SyntaxKind::False => Some(Value::False(False(syntax))),
                    SyntaxKind::Null => Some(Value::Null(Null(syntax))),
                    _ => None,
                }
            }
            fn syntax(&self) -> &SyntaxNode {
                match self {
                    Self::Object(node) => node.syntax(),
                    Self::Array(node) => node.syntax(),
                    Self::Number(node) => node.syntax(),
                    Self::String(node) => node.syntax(),
                    Self::True(node) => node.syntax(),
                    Self::False(node) => node.syntax(),
                    Self::Null(node) => node.syntax(),
                }
            }
        }
        impl Value {
            pub fn r#object(&self) -> Option<Object> {
                self.child()
            }
        }
        impl Value {
            pub fn r#array(&self) -> Option<Array> {
                self.child()
            }
        }
        impl Value {
            pub fn r#number(&self) -> Option<Number> {
                self.child()
            }
        }
        impl Value {
            pub fn r#string(&self) -> Option<String> {
                self.child()
            }
        }
        impl Value {
            pub fn r#true(&self) -> Option<True> {
                self.child()
            }
        }
        impl Value {
            pub fn r#false(&self) -> Option<False> {
                self.child()
            }
        }
        impl Value {
            pub fn r#null(&self) -> Option<Null> {
                self.child()
            }
        }
        pub trait Visitor {
            fn visit_kw(&mut self, kw: SyntaxToken) {}
            fn visit_array(&mut self, r#array: Array) {
                for r#value in r#array.values() {
                    self.visit_value(r#value);
                }
            }
            fn visit_document(&mut self, r#document: Document) {
                for r#value in r#document.r#value() {
                    self.visit_value(r#value);
                }
            }
            fn visit_false(&mut self, r#false: False) {}
            fn visit_null(&mut self, r#null: Null) {}
            fn visit_number(&mut self, r#number: Number) {}
            fn visit_object(&mut self, r#object: Object) {
                for r#pair in r#object.pairs() {
                    self.visit_pair(r#pair);
                }
            }
            fn visit_pair(&mut self, r#pair: Pair) {
                for r#number in r#pair.r#number() {
                    self.visit_number(r#number);
                }
                for r#string in r#pair.r#string() {
                    self.visit_string(r#string);
                }
                for r#value in r#pair.r#value() {
                    self.visit_value(r#value);
                }
            }
            fn visit_string(&mut self, r#string: String) {}
            fn visit_true(&mut self, r#true: True) {}
            fn visit_value(&mut self, r#value: Value) {
                match r#value {
                    Value::Object(node) => self.visit_object(node),
                    Value::Array(node) => self.visit_array(node),
                    Value::Number(node) => self.visit_number(node),
                    Value::String(node) => self.visit_string(node),
                    Value::True(node) => self.visit_true(node),
                    Value::False(node) => self.visit_false(node),
                    Value::Null(node) => self.visit_null(node),
                }
            }
        }
        #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
        #[repr(u16)]
        pub enum SyntaxKind {
            Array,
            Document,
            False,
            Null,
            Number,
            Object,
            Pair,
            String,
            True,
            Value,
            Token,
            Whitespace,
            Err,
        }
        impl TryFrom<&'static str> for SyntaxKind {
            type Error = ();
            fn try_from(s: &'static str) -> Result<Self, Self::Error> {
                match s {
                    "array" => Ok(Self::Array),
                    "document" => Ok(Self::Document),
                    "false" => Ok(Self::False),
                    "null" => Ok(Self::Null),
                    "number" => Ok(Self::Number),
                    "object" => Ok(Self::Object),
                    "pair" => Ok(Self::Pair),
                    "string" => Ok(Self::String),
                    "true" => Ok(Self::True),
                    "value" => Ok(Self::Value),
                    "ERROR" => Ok(Self::Err),
                    _ => Err(()),
                }
            }
        }
    "#]]
    .assert_eq(&out);
}

// simplified jsonish grammar
const TEST_GRAMMAR: &str = r#"
{
  "name": "json",
  "rules": {
    "document": {
      "type": "SYMBOL",
      "name": "value"
    },
    "value": {
      "type": "CHOICE",
      "members": [
        {
          "type": "SYMBOL",
          "name": "object"
        },
        {
          "type": "SYMBOL",
          "name": "array"
        },
        {
          "type": "SYMBOL",
          "name": "number"
        },
        {
          "type": "SYMBOL",
          "name": "string"
        },
        {
          "type": "SYMBOL",
          "name": "true"
        },
        {
          "type": "SYMBOL",
          "name": "false"
        },
        {
          "type": "SYMBOL",
          "name": "null"
        }
      ]
    },
    "object": {
      "type": "SEQ",
      "members": [
        {
          "type": "STRING",
          "value": "{"
        },
        {
          "type": "CHOICE",
          "members": [
            {
              "type": "SEQ",
              "members": [
                {
                  "type": "SYMBOL",
                  "name": "pair"
                },
                {
                  "type": "REPEAT",
                  "content": {
                    "type": "SEQ",
                    "members": [
                      {
                        "type": "STRING",
                        "value": ","
                      },
                      {
                        "type": "SYMBOL",
                        "name": "pair"
                      }
                    ]
                  }
                }
              ]
            },
            {
              "type": "BLANK"
            }
          ]
        },
        {
          "type": "STRING",
          "value": "}"
        }
      ]
    },
    "pair": {
      "type": "SEQ",
      "members": [
        {
          "type": "FIELD",
          "name": "key",
          "content": {
            "type": "CHOICE",
            "members": [
              {
                "type": "SYMBOL",
                "name": "string"
              },
              {
                "type": "SYMBOL",
                "name": "number"
              }
            ]
          }
        },
        {
          "type": "STRING",
          "value": ":"
        },
        {
          "type": "FIELD",
          "name": "value",
          "content": {
            "type": "SYMBOL",
            "name": "value"
          }
        }
      ]
    },
    "array": {
      "type": "SEQ",
      "members": [
        {
          "type": "STRING",
          "value": "["
        },
        {
          "type": "CHOICE",
          "members": [
            {
              "type": "SEQ",
              "members": [
                {
                  "type": "SYMBOL",
                  "name": "value"
                },
                {
                  "type": "REPEAT",
                  "content": {
                    "type": "SEQ",
                    "members": [
                      {
                        "type": "STRING",
                        "value": ","
                      },
                      {
                        "type": "SYMBOL",
                        "name": "value"
                      }
                    ]
                  }
                }
              ]
            },
            {
              "type": "BLANK"
            }
          ]
        },
        {
          "type": "STRING",
          "value": "]"
        }
      ]
    },
    "string": {
      "type": "SEQ",
      "members": [
        {
          "type": "STRING",
          "value": "\""
        },
        {
          "type": "STRING",
          "value": "\""
        }
      ]
    },
    "number": {
      "type": "STRING",
      "value": "0"
    },
    "true": {
      "type": "STRING",
      "value": "true"
    },
    "false": {
      "type": "STRING",
      "value": "false"
    },
    "null": {
      "type": "STRING",
      "value": "null"
    }
  },
  "extras": [
    {
      "type": "PATTERN",
      "value": "\\s"
    }
  ],
  "conflicts": [],
  "precedences": [],
  "externals": [],
  "inline": [],
  "supertypes": [
    "value"
  ]
}

"#;
