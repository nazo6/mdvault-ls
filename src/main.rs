use std::collections::HashMap;

use educe::Educe;
use tokio::sync::Mutex;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LspService, Server};
use tree_sitter::{Parser, Tree};

mod handler;
mod server;
mod utils;

#[derive(Educe)]
#[educe(Debug)]
struct Backend {
    client: Client,
    open_docs: Mutex<HashMap<Url, (String, Tree)>>,
    #[educe(Debug(ignore))]
    parser: Mutex<Parser>,
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let language = tree_sitter_md::language();
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(language).unwrap();

    let (service, socket) = LspService::new(|client| Backend {
        client,
        open_docs: Mutex::new(HashMap::new()),
        parser: Mutex::new(parser),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
