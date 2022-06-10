use tower_lsp::lsp_types::*;
use tower_lsp::{jsonrpc, Client, LanguageServer};

pub struct Lsp {
    client: Client,
}

impl Lsp {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Lsp {
    async fn initialize(&self, _: InitializeParams) -> jsonrpc::Result<InitializeResult> {
        Ok(InitializeResult::default())
    }

    async fn initialized(&self, _: InitializedParams) {}

    async fn shutdown(&self) -> jsonrpc::Result<()> {
        Ok(())
    }
}
