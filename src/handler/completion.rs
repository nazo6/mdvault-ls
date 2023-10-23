use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{CompletionItem, CompletionParams, CompletionResponse};

use crate::Backend;

pub(crate) async fn handle(
    backend: &Backend,
    params: CompletionParams,
) -> Result<Option<CompletionResponse>> {
    Ok(Some(CompletionResponse::Array(vec![
        CompletionItem::new_simple("Hello".to_string(), "Some detail".to_string()),
        CompletionItem::new_simple("Bye".to_string(), "More detail".to_string()),
    ])))
}
