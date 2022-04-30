use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

#[derive(Debug)]
struct Backend {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        let capabilities = ServerCapabilities {
            text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
            selection_range_provider: None,
            hover_provider: None,
            completion_provider: None,
            signature_help_provider: None,
            definition_provider: None,
            type_definition_provider: None,
            implementation_provider: None,
            references_provider: None,
            document_highlight_provider: None,
            document_symbol_provider: None,
            workspace_symbol_provider: None,
            code_action_provider: None,
            code_lens_provider: None,
            document_formatting_provider: None,
            document_range_formatting_provider: None,
            document_on_type_formatting_provider: None,
            rename_provider: None,
            document_link_provider: None,
            color_provider: None,
            folding_range_provider: None,
            declaration_provider: None,
            execute_command_provider: None,
            workspace: None,
            call_hierarchy_provider: None,
            semantic_tokens_provider: None,
            moniker_provider: None,
            linked_editing_range_provider: None,
            experimental: None,
        };
        Ok(InitializeResult {
            capabilities: capabilities,
            server_info: None,
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        // println!("yay init");
        self.client
            .log_message(MessageType::INFO, "server was in fact initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    // async fn did_open(&self, params: DidOpenTextDocumentParams) {
    //     let _ = params;
    //     self.client.log_message(MessageType::INFO, "opened!").await;
    //     // println!("params text {}", params.text.unwrap());
    //     // warn!("Got a textDocument/didSave notification, but it is not implemented");
    // }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        let _ = params;
        self.client.log_message(MessageType::INFO, "saved!").await;
        self.client
            .publish_diagnostics(
                params.text_document.uri,
                vec![Diagnostic {
                    range: Range {
                        start: Position {
                            line: 0,
                            character: 2,
                        },
                        end: Position {
                            line: 0,
                            character: 4,
                        },
                    },
                    severity: Some(DiagnosticSeverity::WARNING),
                    code: None,
                    code_description: None,
                    source: Some(String::from("Chandler")),
                    message: String::from("Hello, world!"),
                    related_information: None,
                    tags: None,
                    data: None,
                }],
                None,
            )
            .await;
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend { client });
    Server::new(stdin, stdout, socket).serve(service).await;
}
