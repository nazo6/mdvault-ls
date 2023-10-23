use tower_lsp::lsp_types::DidOpenTextDocumentParams;

use crate::Backend;

pub(crate) async fn handle(backend: &Backend, params: DidOpenTextDocumentParams) {
    let doc = params.text_document;
    let mut open_docs = backend.open_docs.lock().await;
    open_docs.insert(doc.uri, doc.text);
}
