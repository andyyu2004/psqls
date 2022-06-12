pub use self::highlight::{Highlight, HighlightRange};
pub use psqls_syn::{Rope, SyntaxDatabase, TextRange, TextSize};

mod highlight;

use std::ops::Deref;
use std::sync::Arc;

use salsa::ParallelDatabase;

#[derive(Default)]
pub struct Ide {
    db: IdeDatabase,
}

pub struct Snapshot {
    snapshot: salsa::Snapshot<IdeDatabase>,
}

impl Deref for Snapshot {
    type Target = salsa::Snapshot<IdeDatabase>;

    fn deref(&self) -> &Self::Target {
        &self.snapshot
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Change {
    pub url: Arc<str>,
    pub text: String,
}

impl Ide {
    pub fn snapshot(&self) -> Snapshot {
        Snapshot {
            snapshot: self.db.snapshot(),
        }
    }

    pub fn apply(&mut self, change: Change) {
        self.db.set_text(change.url, change.text.into());
    }
}

#[salsa::database(psqls_syn::SyntaxDatabaseStorage)]
#[derive(Default)]
pub struct IdeDatabase {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for IdeDatabase {}

impl salsa::ParallelDatabase for IdeDatabase {
    fn snapshot(&self) -> salsa::Snapshot<Self> {
        salsa::Snapshot::new(Self {
            storage: self.storage.snapshot(),
        })
    }
}

#[cfg(test)]
mod tests;
