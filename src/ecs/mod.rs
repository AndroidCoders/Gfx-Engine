//! # Manager: ECS Module Root
//!
//! This module acts as the central hub for the Entity-Component-System logic.
//! It organizes the codebase into atomic domains:
//! - **component:** Pure data structures (the 'What').
//! - **system:** Domain-specific logic processors (the 'How').
//! - **world:** The central data container and entity manager.
//! - **event:** The decoupled communication bus.

pub mod component;
pub mod event;
pub mod systems;
pub mod world;
pub mod system_manager;
pub mod resources;
