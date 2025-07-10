use rust_mcp_schema::{CallToolResult, TextContent, Tool, CallToolResultContentItem, ToolInputSchema, CallToolRequest};
use rust_mcp_schema::schema_utils::CallToolError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefactorGuidanceTool {
    pub code_snippet: String,
    pub refactor_type: String,
}

impl RefactorGuidanceTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let guidance = generate_refactor_guidance(&self.code_snippet, &self.refactor_type);
        
        Ok(CallToolResult {
            content: vec![CallToolResultContentItem::TextContent(TextContent::new(guidance, None))],
            is_error: Some(false),
            meta: None,
        })
    }
}

pub enum RefactorTools {
    RefactorGuidanceTool(RefactorGuidanceTool),
}

impl RefactorTools {
    pub fn tools() -> Vec<Tool> {
        vec![
            Tool {
                name: "refactor_guidance".to_string(),
                description: Some("Provides guidance for piece-wise refactoring and code generation with minimal information".to_string()),
                input_schema: ToolInputSchema::new(vec!["code_snippet".to_string(), "refactor_type".to_string()], None),
            }
        ]
    }
}

impl TryFrom<CallToolRequest> for RefactorTools {
    type Error = CallToolError;

    fn try_from(params: CallToolRequest) -> Result<Self, Self::Error> {
        match params.params.name.as_str() {
            "refactor_guidance" => {
                let code_snippet = params.params.arguments.as_ref()
                    .and_then(|args| args.get("code_snippet"))
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| CallToolError::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Missing code_snippet")))?
                    .to_string();
                
                let refactor_type = params.params.arguments.as_ref()
                    .and_then(|args| args.get("refactor_type"))
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| CallToolError::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Missing refactor_type")))?
                    .to_string();

                Ok(RefactorTools::RefactorGuidanceTool(RefactorGuidanceTool {
                    code_snippet,
                    refactor_type,
                }))
            }
            _ => Err(CallToolError::unknown_tool(params.params.name)),
        }
    }
}

fn generate_refactor_guidance(code_snippet: &str, refactor_type: &str) -> String {
    match refactor_type {
        "extract_function" => {
            format!("For extracting a function from this code snippet:\n\n{}\n\n1. Identify the logical unit of work\n2. Determine input parameters and return type\n3. Choose a descriptive function name\n4. Ensure minimal dependencies\n5. Test the extracted function independently", code_snippet)
        }
        "rename_variable" => {
            format!("For renaming variables in this code snippet:\n\n{}\n\n1. Use descriptive, intention-revealing names\n2. Follow language conventions\n3. Avoid abbreviations unless widely understood\n4. Consider the scope and context\n5. Update all references consistently", code_snippet)
        }
        "simplify_logic" => {
            format!("For simplifying logic in this code snippet:\n\n{}\n\n1. Reduce nesting levels\n2. Use early returns where appropriate\n3. Extract complex conditions into well-named variables\n4. Consider using guard clauses\n5. Break down complex expressions", code_snippet)
        }
        _ => {
            format!("General refactoring guidance for code snippet:\n\n{}\n\n1. Maintain single responsibility principle\n2. Minimize information exposure\n3. Use clear, descriptive names\n4. Keep functions small and focused\n5. Ensure proper error handling", code_snippet)
        }
    }
}