use expect_test::{expect, Expect};
use psqls_ide::{Change, Ide};

use super::semantic_tokens;

fn test(sql: &str, expect: Expect) {
    let mut ide = Ide::default();
    ide.apply(Change {
        url: "foo".into(),
        text: sql.into(),
    });

    expect.assert_debug_eq(&semantic_tokens(&ide.snapshot(), "foo".into()));
}

#[test]
fn test_semantic_tokens() {
    let sql = "
select * from bar
    ";
    test(
        sql,
        expect![[r#"
            SemanticTokens {
                result_id: None,
                data: [
                    SemanticToken {
                        delta_line: 1,
                        delta_start: 0,
                        length: 6,
                        token_type: 3,
                        token_modifiers_bitset: 0,
                    },
                    SemanticToken {
                        delta_line: 0,
                        delta_start: 9,
                        length: 4,
                        token_type: 3,
                        token_modifiers_bitset: 0,
                    },
                ],
            }
        "#]],
    )
}
