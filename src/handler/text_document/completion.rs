use tower_lsp::jsonrpc::Error;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{CompletionItem, CompletionParams, CompletionResponse};

use crate::utils::visit_node::visit_node;
use crate::utils::visit_node::ControlFlow;
use crate::utils::visit_node::Step;
use crate::Backend;

pub(crate) async fn handle(
    backend: &Backend,
    params: CompletionParams,
) -> Result<Option<CompletionResponse>> {
    let uri = params.text_document_position.text_document.uri;
    let tree = {
        let open_docs = backend.open_docs.lock().await;
        open_docs
            .get(&uri)
            .ok_or_else(|| Error::invalid_params("Document not found"))?
            .clone()
    };
    visit_node(&tree.root_node(), |step| {
        let node = match step {
            Step::In(node) => node,
            Step::Out(node) => node,
        };

        ControlFlow::Continue
    });
    Ok(Some(CompletionResponse::Array(vec![
        CompletionItem::new_simple("Hello".to_string(), "Some detail".to_string()),
        CompletionItem::new_simple("Bye".to_string(), "More detail".to_string()),
    ])))
}
