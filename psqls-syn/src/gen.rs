use std::io::{Read, Write};
use std::process::{Command, Stdio};

use convert_case::{Case, Casing};
use expect_test::expect;
use quote::__private::{Ident, TokenStream};
use quote::{format_ident, quote};

use self::copy::{parse_grammar, InputGrammar, Symbol, VariableType};

mod copy;

#[test]
fn generate_psql_nodes() -> std::io::Result<()> {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../tree-sitter-sql/src/grammar.json");
    let grammar = parse_grammar(&std::fs::read_to_string(path)?).unwrap();
    let out = generate_nodes(grammar);
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/nodes.rs");
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

#[derive(Default)]
struct Gen {
    s: String,
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
        rustfmt(&self.s)
    }

    fn push(&mut self, stream: TokenStream) {
        self.push_str(&format!("{stream}"))
    }

    fn push_str(&mut self, string: &str) {
        self.s.push_str(string);
    }

    fn gen_rule(&mut self, name: &Ident, rule: Rule) {
        if self.gen_comma_list(name, &rule) {
            return;
        }

        match rule {
            Rule::Blank | Rule::String(_) | Rule::Pattern(_) | Rule::Symbol(_) => {}
            Rule::NamedSymbol(_) => {}
            Rule::Repeat(rule) => self.gen_rule(name, *rule),
            Rule::Choice(rules) | Rule::Seq(rules) => rules
                .into_vec()
                .into_iter()
                .for_each(|rule| self.gen_rule(name, rule)),
        }
    }

    fn gen_comma_list(&mut self, name: &Ident, rule: &Rule) -> bool {
        match rule {
            Rule::Seq(
                box [Rule::NamedSymbol(x), Rule::Choice(
                    box [Rule::Repeat(box Rule::Seq(box [Rule::String(sep), Rule::NamedSymbol(y)])), Rule::Blank],
                )],
            ) if x == y && sep == "," => {
                let pluralized = format_ident!("{}s", x.to_case(Case::Camel));
                let ty = format_ident!("{}", x.to_case(Case::Pascal));
                self.push(quote! {
                    impl #name {
                        pub fn #pluralized(&self) -> impl Iterator<Item = #ty> {
                            todo!()
                        }
                    }
                });
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
        let mut syntax_kinds = vec![];
        self.push(quote! {
            pub type SyntaxNode = rowan::SyntaxNode<Sql>;
        });

        self.push(quote! {
            #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
            pub enum Sql {}

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

        for v in grammar
            .variables
            .into_iter()
            .filter(|v| v.kind == VariableType::Named)
            .filter(|v| !v.name.starts_with('_'))
        {
            let name = format_ident!("{}", v.name.to_case(Case::Pascal));
            self.push(quote! {
                #[derive(Debug, Clone, PartialEq, Eq, Hash)]
                pub struct #name(SyntaxNode);
            });

            self.gen_rule(&name, v.rule.into());
            syntax_kinds.push(v.name);
        }

        let variants = syntax_kinds
            .iter()
            .map(|kind| format_ident!("{}", kind.to_case(Case::Pascal)));

        let cases = syntax_kinds.iter().map(|kind| {
            let variant = format_ident!("{}", kind.to_case(Case::Pascal));
            quote!(#kind => Self::#variant)
        });

        self.push(quote! {
            #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
            #[repr(u16)]
            pub enum SyntaxKind {
                #(#variants,)*
            }


            impl From<&'static str> for SyntaxKind {
                fn from(s: &'static str) -> Self {
                    match s {
                        #(#cases,)*
                        _ => unreachable!(),
                    }
                }
            }
        });
    }
}

#[test]
fn test_generate_nodes() {
    let out = generate_grammar(TEST_GRAMMAR);
    expect![[r#"
        pub type SyntaxNode = rowan::SyntaxNode<Sql>;
        #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
        pub enum Sql {}
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
        pub struct Array(SyntaxNode);
        impl Array {
            pub fn values(&self) -> impl Iterator<Item = Value> {
                todo!()
            }
        }
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct Document(SyntaxNode);
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct False(SyntaxNode);
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct Null(SyntaxNode);
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct Number(SyntaxNode);
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct Object(SyntaxNode);
        impl Object {
            pub fn pairs(&self) -> impl Iterator<Item = Pair> {
                todo!()
            }
        }
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct Pair(SyntaxNode);
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct String(SyntaxNode);
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct True(SyntaxNode);
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct Value(SyntaxNode);
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
        }
        impl From<&'static str> for SyntaxKind {
            fn from(s: &'static str) -> Self {
                match s {
                    "array" => Self::Array,
                    "document" => Self::Document,
                    "false" => Self::False,
                    "null" => Self::Null,
                    "number" => Self::Number,
                    "object" => Self::Object,
                    "pair" => Self::Pair,
                    "string" => Self::String,
                    "true" => Self::True,
                    "value" => Self::Value,
                    _ => unreachable!(),
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
