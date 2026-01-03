//! # Manager: Player Module Root
//! 
//! This module acts as the central hub for the Player domain. 
//! It organizes the codebase into atomic processes:
//! - **states:** HSM logic for player behavior (the 'Brain').
//! - **factory:** The authoritative builder for player entities.

pub mod states;
pub mod factory;
