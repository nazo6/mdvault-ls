use tower_lsp::lsp_types::DidChangeTextDocumentParams;

use crate::interface::{Backend, Document};

pub(crate) async fn handle(backend: &Backend, params: DidChangeTextDocumentParams) {
    let mut changes = params.content_changes;
    let changes_len = changes.len();
    if changes_len == 0 {
        return;
    }
    let new = changes.remove(changes_len - 1).text;

    let mut docs = backend.docs.write().await;
    let tree = backend.parser.lock().await.parse(&new, None).unwrap();
    docs.insert(
        params.text_document.uri,
        Document {
            src: new,
            tree,
            open: true,
        },
    );
}
