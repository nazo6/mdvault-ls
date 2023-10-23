use tower_lsp::lsp_types::DidCloseTextDocumentParams;

use crate::Backend;

pub(crate) async fn handle(backend: &Backend, params: DidCloseTextDocumentParams) {
    let mut open_docs = backend.open_docs.lock().await;
    open_docs.remove(&params.text_document.uri);
}
