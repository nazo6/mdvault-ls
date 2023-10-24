use tower_lsp::lsp_types::DidCloseTextDocumentParams;

use crate::interface::Backend;

pub(crate) async fn handle(backend: &Backend, params: DidCloseTextDocumentParams) {
    let mut open_docs = backend.docs.write().await;
    let Some(doc) = open_docs.get_mut(&params.text_document.uri) else {
        return;
    };
    doc.open = false;
}
