use std::io::{Read, Write};
use std::process::{Command, Stdio};

use convert_case::{Case, Casing};
use expect_test::expect;
use itertools::Itertools;
use quote::__private::TokenStream;
use quote::{format_ident, quote};

use self::copy::{parse_grammar, InputGrammar, Rule, VariableType};

mod copy;

#[test]
fn generate() -> std::io::Result<()> {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../tree-sitter-sql/src/grammar.json");
    let grammar = parse_grammar(&std::fs::read_to_string(path)?).unwrap();
    // let _ = generate_nodes(grammar);
    Ok(())
}

fn generate_grammar(grammar: &str) -> String {
    let grammar = parse_grammar(grammar).unwrap();
    generate_nodes(grammar)
}

fn generate_nodes(grammar: InputGrammar) -> String {
    Gen::default().generate(grammar)
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
        self.push_str(&stream.to_string())
    }

    fn push_str(&mut self, s: &str) {
        self.s.push_str(s);
    }

    fn gen_rule(&mut self, rule: &Rule) {
        dbg!(rule);
        match &rule {
            Rule::Blank | Rule::String(_) => {}
            Rule::Pattern(_) => {}
            Rule::NamedSymbol(_) => {}
            Rule::Symbol(_) => {}
            Rule::Choice(_) => {}
            Rule::Metadata { params, rule } => {}
            Rule::Repeat(_) => {}
            Rule::Seq(rules) => rules.iter().for_each(|rule| {}),
        }
    }

    fn gen(&mut self, grammar: InputGrammar) {
        let mut syntax_kinds = vec![];
        self.push_str("pub type SyntaxNode = rowan::SyntaxNode<Sql>;\n");

        for v in grammar
            .variables
            .iter()
            .filter(|v| v.kind == VariableType::Named)
            .filter(|v| !v.name.starts_with('_'))
        {
            syntax_kinds.push(&v.name);

            let name = format_ident!("{}", v.name.to_case(Case::Pascal));
            self.push(quote! {
                #[derive(Debug, Clone, PartialEq, Eq, Hash)]
                pub struct #name(SyntaxNode);
            });

            self.gen_rule(&v.rule);
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
            enum SyntaxKind {
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
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct Array(SyntaxNode);
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
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct Pair(SyntaxNode);
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct String(SyntaxNode);
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct True(SyntaxNode);
        #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
        #[repr(u16)]
        enum SyntaxKind {
            Array,
            Document,
            False,
            Null,
            Number,
            Object,
            Pair,
            String,
            True,
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
      "name": "_value"
    },
    "_value": {
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
            "name": "_value"
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
                  "name": "_value"
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
                        "name": "_value"
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
    "_value"
  ]
}

"#;
