use convert_case::{Case, Casing};
use expect_test::expect;
use itertools::Itertools;

use self::copy::{parse_grammar, InputGrammar, Rule, VariableType};

mod copy;

#[test]
fn generate() -> std::io::Result<()> {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../tree-sitter-sql/src/grammar.json");
    let grammar = parse_grammar(&std::fs::read_to_string(path)?).unwrap();
    let _ = generate_nodes(grammar);
    Ok(())
}

fn generate_grammar(grammar: &str) -> String {
    let grammar = parse_grammar(grammar).unwrap();
    generate_nodes(grammar)
}

fn generate_nodes(grammar: InputGrammar) -> String {
    let mut syntax_nodes = "pub type SyntaxNode = rowan::SyntaxNode<Sql>;\n".to_owned();

    let mut syntax_kinds = vec![];

    for v in grammar
        .variables
        .iter()
        .filter(|v| v.kind == VariableType::Named)
        .filter(|v| !v.name.starts_with('_'))
    {
        syntax_kinds.push(&v.name);

        syntax_nodes.push_str(&format!(
            "
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct {}(SyntaxNode);\n",
            v.name.to_case(Case::Pascal)
        ));
        match &v.rule {
            Rule::Blank => {}
            Rule::String(_) => {}
            Rule::Pattern(_) => {}
            Rule::NamedSymbol(_) => {}
            Rule::Symbol(_) => {}
            Rule::Choice(_) => {}
            Rule::Metadata { params, rule } => {}
            Rule::Repeat(_) => {}
            Rule::Seq(_) => {}
        }
    }

    let mut syntax_kinds = format!(
        r#"
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u16)]
enum SyntaxKind {{
    {variants}
}}

impl From<&'static str> for SyntaxKind {{
    fn from(s: &'static str) -> Self {{
        match s {{
{cases}
            _ => unreachable!(),
        }}
    }}
}}
"#,
        variants = syntax_kinds
            .iter()
            .map(|kind| kind.to_case(Case::Pascal))
            .join(",\n\t"),
        cases = syntax_kinds
            .iter()
            .map(|kind| format!("\t\t\t\"{}\" => Self::{}", kind, kind.to_case(Case::Pascal)))
            .join(",\n")
    );

    syntax_nodes.push_str(&syntax_kinds);
    syntax_nodes
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
        	True
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
        			"true" => Self::True
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
