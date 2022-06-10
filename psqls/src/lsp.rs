use psqls_ide::{Change, Ide};
use tokio::sync::Mutex;
use tower_lsp::lsp_types::*;
use tower_lsp::{jsonrpc, Client, LanguageServer};

pub struct Lsp {
    ide: Mutex<Ide>,
    client: Client,
}

impl Lsp {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            ide: Default::default(),
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Lsp {
    async fn initialize(&self, _: InitializeParams) -> jsonrpc::Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        change: Some(TextDocumentSyncKind::FULL),
                        ..Default::default()
                    },
                )),
                semantic_tokens_provider: Some(
                    SemanticTokensServerCapabilities::SemanticTokensOptions(
                        SemanticTokensOptions {
                            legend: SemanticTokensLegend {
                                token_types: vec![],
                                token_modifiers: vec![],
                            },
                            range: Some(false),
                            full: Some(SemanticTokensFullOptions::Bool(true)),
                            work_done_progress_options: Default::default(),
                        },
                    ),
                ),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "sqls".to_owned(),
                version: None,
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {}

    async fn did_change(&self, mut params: DidChangeTextDocumentParams) {
        assert_eq!(params.content_changes.len(), 1);
        self.ide.lock().await.apply(Change {
            uri: params.text_document.uri.to_string(),
            text: params.content_changes.swap_remove(0).text,
        });
    }

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> jsonrpc::Result<Option<SemanticTokensResult>> {
        let ide = self.ide.lock().await.snapshot().highlight();
        todo!()
    }

    async fn shutdown(&self) -> jsonrpc::Result<()> {
        Ok(())
    }
}
