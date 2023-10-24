use tower_lsp::lsp_types::DidOpenTextDocumentParams;

use crate::{
    interface::{Backend, Document},
    utils::message::notify_info,
};

pub(crate) async fn handle(backend: &Backend, params: DidOpenTextDocumentParams) {
    let doc = params.text_document;
    let mut open_docs = backend.docs.write().await;
    let Some(tree) = backend.parser.lock().await.parse(&doc.text, None) else {
        notify_info(
            &backend.client,
            &format!("Could not parse document: {}", doc.uri),
        )
        .await;

        return;
    };
    open_docs.insert(
        doc.uri,
        Document {
            src: doc.text,
            tree,
            open: true,
        },
    );
}
