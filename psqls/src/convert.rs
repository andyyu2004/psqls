use psqls_ide::Rope;
use tower_lsp::lsp_types;

use crate::tokens;

pub(crate) trait ConvertWith {
    type Converted;
    type Context;
    fn convert_with(self, _: &Self::Context) -> Self::Converted;
}

pub(crate) trait Convert {
    type Converted;
    fn convert(self) -> Self::Converted;
}

impl<C: Convert> ConvertWith for C {
    type Context = Rope;
    type Converted = C::Converted;
    fn convert_with(self, _: &Self::Context) -> Self::Converted {
        self.convert()
    }
}

impl ConvertWith for psqls_ide::TextSize {
    type Converted = lsp_types::Position;
    type Context = Rope;

    fn convert_with(self, rope: &Self::Context) -> Self::Converted {
        let pos = self.into();
        let line = rope.byte_to_line(pos);
        let character = (pos - rope.line_to_byte(line)) as u32;
        lsp_types::Position {
            line: line as u32,
            character,
        }
    }
}

impl ConvertWith for psqls_ide::TextRange {
    type Converted = lsp_types::Range;
    type Context = Rope;

    fn convert_with(self, rope: &Self::Context) -> Self::Converted {
        lsp_types::Range {
            start: self.start().convert_with(rope),
            end: self.end().convert_with(rope),
        }
    }
}

impl Convert for psqls_ide::Highlight {
    type Converted = (u32, u32);

    fn convert(self) -> Self::Converted {
        let i = tokens::TYPES
            .iter()
            .position(|t| {
                t == &match self {
                    psqls_ide::Highlight::Keyword => lsp_types::SemanticTokenType::KEYWORD,
                }
            })
            .unwrap();
        (i as u32, 0)
    }
}
