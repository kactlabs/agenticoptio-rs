//! Message types for AgenticOptio.
//!
//! Lightweight message implementations compatible with standard chat API formats.

use serde::{Deserialize, Serialize};

/// Tool call information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub args: serde_json::Value,
}

/// Base message trait
pub trait BaseMessage {
    fn role(&self) -> &str;
    fn content(&self) -> &str;
    fn to_dict(&self) -> serde_json::Value;
}

/// System message for setting agent behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMessage {
    pub content: String,
}

impl SystemMessage {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
        }
    }
}

impl BaseMessage for SystemMessage {
    fn role(&self) -> &str {
        "system"
    }

    fn content(&self) -> &str {
        &self.content
    }

    fn to_dict(&self) -> serde_json::Value {
        serde_json::json!({
            "role": "system",
            "content": self.content
        })
    }
}

/// User/human message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanMessage {
    pub content: String,
}

impl HumanMessage {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
        }
    }
}

impl BaseMessage for HumanMessage {
    fn role(&self) -> &str {
        "user"
    }

    fn content(&self) -> &str {
        &self.content
    }

    fn to_dict(&self) -> serde_json::Value {
        serde_json::json!({
            "role": "user",
            "content": self.content
        })
    }
}

/// Assistant/AI message with optional tool calls
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIMessage {
    pub content: String,
    #[serde(default)]
    pub tool_calls: Vec<ToolCall>,
}

impl AIMessage {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            tool_calls: Vec::new(),
        }
    }

    pub fn with_tool_calls(content: impl Into<String>, tool_calls: Vec<ToolCall>) -> Self {
        Self {
            content: content.into(),
            tool_calls,
        }
    }
}

impl BaseMessage for AIMessage {
    fn role(&self) -> &str {
        "assistant"
    }

    fn content(&self) -> &str {
        &self.content
    }

    fn to_dict(&self) -> serde_json::Value {
        let mut msg = serde_json::json!({
            "role": "assistant",
            "content": self.content
        });

        if !self.tool_calls.is_empty() {
            let tool_calls: Vec<serde_json::Value> = self
                .tool_calls
                .iter()
                .map(|tc| {
                    serde_json::json!({
                        "id": tc.id,
                        "type": "function",
                        "function": {
                            "name": tc.name,
                            "arguments": serde_json::to_string(&tc.args).unwrap_or_default()
                        }
                    })
                })
                .collect();
            msg["tool_calls"] = serde_json::json!(tool_calls);
        }

        msg
    }
}

/// Tool/function result message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolMessage {
    pub content: String,
    pub tool_call_id: String,
}

impl ToolMessage {
    pub fn new(content: impl Into<String>, tool_call_id: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            tool_call_id: tool_call_id.into(),
        }
    }
}

impl BaseMessage for ToolMessage {
    fn role(&self) -> &str {
        "tool"
    }

    fn content(&self) -> &str {
        &self.content
    }

    fn to_dict(&self) -> serde_json::Value {
        serde_json::json!({
            "role": "tool",
            "content": self.content,
            "tool_call_id": self.tool_call_id
        })
    }
}

/// Unified message enum for easier handling
#[derive(Debug, Clone)]
pub enum Message {
    System(SystemMessage),
    Human(HumanMessage),
    AI(AIMessage),
    Tool(ToolMessage),
}

impl Message {
    pub fn system(content: impl Into<String>) -> Self {
        Message::System(SystemMessage::new(content))
    }

    pub fn user(content: impl Into<String>) -> Self {
        Message::Human(HumanMessage::new(content))
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Message::AI(AIMessage::new(content))
    }

    pub fn tool(content: impl Into<String>, tool_call_id: impl Into<String>) -> Self {
        Message::Tool(ToolMessage::new(content, tool_call_id))
    }

    pub fn to_dict(&self) -> serde_json::Value {
        match self {
            Message::System(m) => m.to_dict(),
            Message::Human(m) => m.to_dict(),
            Message::AI(m) => m.to_dict(),
            Message::Tool(m) => m.to_dict(),
        }
    }

    pub fn role(&self) -> &str {
        match self {
            Message::System(m) => m.role(),
            Message::Human(m) => m.role(),
            Message::AI(m) => m.role(),
            Message::Tool(m) => m.role(),
        }
    }

    pub fn content(&self) -> &str {
        match self {
            Message::System(m) => m.content(),
            Message::Human(m) => m.content(),
            Message::AI(m) => m.content(),
            Message::Tool(m) => m.content(),
        }
    }
}

/// Convert messages to dict format for API calls
pub fn messages_to_dict(messages: &[Message]) -> Vec<serde_json::Value> {
    messages.iter().map(|m| m.to_dict()).collect()
}
