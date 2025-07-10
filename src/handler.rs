use async_trait::async_trait;
use rust_mcp_schema::{
    CallToolRequest, CallToolResult, ListToolsRequest, ListToolsResult, RpcError,
};
use rust_mcp_schema::schema_utils::CallToolError;
use rust_mcp_sdk::{
    mcp_server::ServerHandler,
    MCPServer,
};

use crate::tools::RefactorTools;

pub struct LineCookHandler;

#[async_trait]
impl ServerHandler for LineCookHandler {
    async fn handle_list_tools_request(
        &self,
        _request: ListToolsRequest,
        _server: &dyn MCPServer,
    ) -> Result<ListToolsResult, RpcError> {
        Ok(ListToolsResult {
            tools: RefactorTools::tools(),
            meta: None,
            next_cursor: None,
        })
    }

    async fn handle_call_tool_request(
        &self,
        request: CallToolRequest,
        _server: &dyn MCPServer,
    ) -> Result<CallToolResult, CallToolError> {
        let tool_params = RefactorTools::try_from(request)?;
        match tool_params {
            RefactorTools::RefactorGuidanceTool(tool) => tool.call_tool(),
        }
    }
}