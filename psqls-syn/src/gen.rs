use convert_case::{Case, Casing};
use expect_test::expect;

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
    let mut s = String::new();
    for variable in grammar
        .variables
        .into_iter()
        .filter(|v| v.kind == VariableType::Named)
        .filter(|v| !v.name.starts_with('_'))
    {
        s.push_str(&format!(
            "
            pub struct {}();
        ",
            variable.name.to_case(Case::Pascal)
        ));
        match variable.rule {
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
    s
}

#[test]
fn test_generate_nodes() {
    let out = generate_grammar(TEST_GRAMMAR);
    expect![[r#"

                    pub struct Array();
        
                    pub struct Document();
        
                    pub struct False();
        
                    pub struct Null();
        
                    pub struct Number();
        
                    pub struct Object();
        
                    pub struct Pair();
        
                    pub struct String();
        
                    pub struct True();
        "#]]
    .assert_eq(&out);
}

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
