use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::LanguageServer;

use crate::handler;
use crate::interface::Backend;
use crate::watcher::async_watch;

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        let workspace = params.workspace_folders.ok_or_else(|| {
            tower_lsp::jsonrpc::Error::invalid_params("Single file is not supported")
        })?;
        let workspace = workspace.get(0).ok_or_else(|| {
            tower_lsp::jsonrpc::Error::invalid_params("No workspace is specified")
        })?;
        let path = workspace.uri.path().to_string();

        tokio::spawn(async move {
            async_watch(path).await;
        });

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(vec![" ".to_string(), ".".to_string()]),
                    ..Default::default()
                }),
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        handler::text_document::completion::handle(self, params).await
    }

    async fn hover(&self, _: HoverParams) -> Result<Option<Hover>> {
        Ok(None)
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        handler::text_document::did_open::handle(self, params).await
    }
    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        handler::text_document::did_change::handle(self, params).await
    }
    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        handler::text_document::did_close::handle(self, params).await
    }
}
