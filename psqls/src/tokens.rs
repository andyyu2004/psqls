use psqls_ide::{HighlightRange, Rope};
use tower_lsp::lsp_types::*;

use crate::convert::{Convert, ConvertWith};

pub const TYPES: &[SemanticTokenType] = &[
    SemanticTokenType::COMMENT,
    SemanticTokenType::ENUM,
    SemanticTokenType::ENUM_MEMBER,
    SemanticTokenType::KEYWORD,
    SemanticTokenType::NUMBER,
    SemanticTokenType::OPERATOR,
    SemanticTokenType::PARAMETER,
    SemanticTokenType::PROPERTY,
    SemanticTokenType::STRING,
    SemanticTokenType::STRUCT,
    SemanticTokenType::TYPE,
];

pub const MODIFIERS: &[SemanticTokenModifier] = &[];

pub(crate) fn convert(rope: &Rope, highlights: Vec<HighlightRange>) -> SemanticTokens {
    let mut builder = SemanticTokensBuilder::new();
    for token in highlights {
        let (token_index, modifier_bitset) = token.hl.convert();
        builder.push(
            token.range.convert_with(&rope),
            token_index,
            modifier_bitset,
        );
    }
    builder.build()
}

// from rust-analyzer
pub(crate) struct SemanticTokensBuilder {
    prev_line: u32,
    prev_char: u32,
    data: Vec<SemanticToken>,
}

impl SemanticTokensBuilder {
    pub(crate) fn new() -> Self {
        SemanticTokensBuilder {
            prev_line: 0,
            prev_char: 0,
            data: Default::default(),
        }
    }

    /// Push a new token onto the builder
    pub(crate) fn push(&mut self, range: Range, token_index: u32, modifier_bitset: u32) {
        let mut push_line = range.start.line as u32;
        let mut push_char = range.start.character as u32;

        if !self.data.is_empty() {
            push_line -= self.prev_line;
            if push_line == 0 {
                push_char -= self.prev_char;
            }
        }

        // A token cannot be multiline
        let token_len = range.end.character - range.start.character;

        let token = SemanticToken {
            delta_line: push_line,
            delta_start: push_char,
            length: token_len as u32,
            token_type: token_index,
            token_modifiers_bitset: modifier_bitset,
        };

        self.data.push(token);

        self.prev_line = range.start.line as u32;
        self.prev_char = range.start.character as u32;
    }

    pub(crate) fn build(self) -> SemanticTokens {
        SemanticTokens {
            result_id: None,
            data: self.data,
        }
    }
}
