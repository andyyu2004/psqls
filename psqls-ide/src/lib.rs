mod highlight;

use psqls_syn::SyntaxDatabase;
use salsa::ParallelDatabase;

#[derive(Default)]
pub struct Ide {
    db: IdeDatabase,
}

pub struct Snapshot {
    snapshot: salsa::Snapshot<IdeDatabase>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Change {
    pub uri: String,
    pub text: String,
}

impl Ide {
    pub fn snapshot(&self) -> Snapshot {
        Snapshot {
            snapshot: self.db.snapshot(),
        }
    }

    pub fn apply(&mut self, change: Change) {
        self.db.set_text(change.uri, change.text.into());
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
