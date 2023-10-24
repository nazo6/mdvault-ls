use std::collections::HashMap;
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Arc;

use educe::Educe;
use tokio::sync::Mutex;
use tokio::sync::RwLock;
use tower_lsp::lsp_types::*;
use tower_lsp::Client;
use tree_sitter::{Parser, Tree};

#[derive(Debug)]
pub struct Document {
    pub src: String,
    pub tree: Tree,
    pub open: bool,
}

#[derive(Educe)]
#[educe(Debug)]
pub struct Backend {
    pub client: Client,
    pub docs: RwLock<HashMap<Url, Document>>,
    #[educe(Debug(ignore))]
    pub parser: Mutex<Parser>,
    pub workspace_files: Arc<RwLock<HashSet<PathBuf>>>,
}
