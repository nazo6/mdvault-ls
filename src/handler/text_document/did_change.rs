use tower_lsp::lsp_types::DidChangeTextDocumentParams;

use crate::Backend;

pub(crate) async fn handle(backend: &Backend, params: DidChangeTextDocumentParams) {
    let mut changes = params.content_changes;
    let changes_len = changes.len();
    if changes_len == 0 {
        return;
    }
    let new = changes.remove(changes_len - 1).text;

    let mut open_docs = backend.open_docs.lock().await;
    open_docs.insert(params.text_document.uri, new);
}
