use std::{collections::HashMap, sync::Arc};

use tokio::sync::{Mutex, RwLock};
use tower_lsp::{LspService, Server};

mod handler;
mod interface;
mod server;
mod utils;
mod watcher;

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let language = tree_sitter_md::language();
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(language).unwrap();

    let (service, socket) = LspService::new(|client| interface::Backend {
        client,
        docs: RwLock::new(HashMap::new()),
        parser: Mutex::new(parser),
        workspace_files: Arc::new(Mutex::new(vec![])),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
