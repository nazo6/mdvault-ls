use tower_lsp::{lsp_types::MessageType, Client};

pub async fn notify_info(client: &Client, message: &str) {
    client
        .show_message(MessageType::INFO, message.to_string())
        .await;
}
