use rust_mcp_schema::{
    Implementation, InitializeResult, ServerCapabilities, ServerCapabilitiesTools,
    LATEST_PROTOCOL_VERSION,
};
use rust_mcp_sdk::{
    error::SdkResult,
    mcp_server::{server_runtime, ServerRuntime},
    MCPServer, StdioTransport, TransportOptions,
};

mod handler;
mod tools;

use handler::LineCookHandler;

#[tokio::main]
async fn main() -> SdkResult<()> {
    let server_details = InitializeResult {
        server_info: Implementation {
            name: "Line Cook MCP Server".to_string(),
            version: "0.1.0".to_string(),
        },
        capabilities: ServerCapabilities {
            tools: Some(ServerCapabilitiesTools { list_changed: None }),
            ..Default::default()
        },
        meta: None,
        instructions: Some("MCP server for piece-wise refactoring and code generation guidance".to_string()),
        protocol_version: LATEST_PROTOCOL_VERSION.to_string(),
    };

    let transport = StdioTransport::new(TransportOptions::default())?;
    let handler = LineCookHandler {};
    let server: ServerRuntime = server_runtime::create_server(server_details, transport, handler);

    server.start().await?;

    Ok(())
}
