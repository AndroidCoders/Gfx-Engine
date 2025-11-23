//! This module contains all the logic for the Entity-Component-System (ECS)
//! architecture of the game.
//!
//! - **component:** Defines all the data components that can be attached to entities.
//! - **system:** Defines all the systems that operate on entities with specific components.
//! - **world:** Defines the `World`, which is the container for all entities and components.

pub mod component;
pub mod event;
pub mod systems;
pub mod world;
