//! Core components for AgenticOptio.
//!
//! This module provides the core message types and base classes used throughout
//! the AgenticOptio library.

pub mod messages;

pub use messages::{AIMessage, BaseMessage, HumanMessage, Message, SystemMessage, ToolMessage};
