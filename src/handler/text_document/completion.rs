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
    let data = {
        let open_docs = backend.open_docs.lock().await;
        open_docs
            .get(&uri)
            .ok_or_else(|| Error::invalid_params("Document not found"))?
            .clone()
    };

    let mut response = vec![];

    visit_node(&data.1.root_node(), |step| {
        let node = match step {
            Step::In(node) => node,
            Step::Out(node) => node,
        };
        if node.kind() == "atx_heading" {
            let node_text = node.utf8_text(data.0.as_bytes()).unwrap();
            response.push(CompletionItem::new_simple(
                node_text.to_string(),
                "text".to_string(),
            ))
        }

        ControlFlow::Continue
    });
    Ok(Some(CompletionResponse::Array(response)))
}
