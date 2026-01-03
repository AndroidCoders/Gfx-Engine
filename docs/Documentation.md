# Rust Documentation Best Practices (v2 - WYSIWID Standard)

This document defines the mandatory documentation standards for the `Gfx-Engine`. These standards are a critical enabler for the **WYSIWID** architecture, ensuring that what you see in the code is exactly what it does.

## The 3-Level Documentation Rule

Every source code module (`.rs` file) must implement these three levels of documentation to ensure maximum legibility for both humans and AI assistants.

### Level 1: Module Identity (`//!`)
Every file must start with a module-level doc comment explaining its role in the architecture.

*   **For Concepts:** State the domain it owns (e.g., Physics, Health) and the specific components it manages.
*   **For Synchronizations:** State the behavioral rule it implements (e.g., "When a collision occurs, determine if damage should be dealt").
*   **For Managers:** State the high-level orchestration it performs.

**Example:**
```rust
//! # Synchronization: Audio
//! 
//! This module acts as the "Glue" between gameplay facts and the audio engine.
//! It listens for events like [crate::ecs::event::EventEntityJumped] 
//! and triggers the appropriate sound effects.
```

### Level 2: Function Intent (`///`)
Every public and internal function must have a doc comment explaining its **Semantic Purpose**.

*   Avoid describing *what the code says* (e.g., "increments a counter").
*   Describe *what the process means* (e.g., "Increments the player's gold count and updates the persistent statistics").

**Example:**
```rust
/// Interprets a raw collision event to determine if a 'Stomp' or 'Injury' fact has occurred.
fn resolve_collision(&self, world: &mut World, event: EventCollision) { ... }
```

### Level 3: Logic Implementation (`//`)
Within the function body, use standard comments to explain the **Logical Steps** and the **Why** behind the code.

*   This acts as a "blueprint" for the implementation.
*   It is critical for AI assistants to follow these steps to avoid logic errors during refactoring.

**Example:**
```rust
fn update_health(&mut self, world: &mut World) {
    // 1. Consume all incoming 'CommandDamage' intents.
    // 2. Locate the entity's Health component.
    // 3. Subtract the damage value, clamping at zero to prevent underflow.
    // 4. Publish an [crate::ecs::event::EventPlayerDied] if health is zero.
}
```

---

## Advanced Standards (Mandatory)

To ensure robustness, performance, and architectural clarity, the following standards are mandatory for all core engine code.

### 1. Intra-Doc Links
Do not just write the name of a struct, system, or event; link to it using Rust's bracket syntax. This ensures that if the code changes (e.g., a struct is renamed), the documentation breaks (compilation error), ensuring docs remain up-to-date.

*   **Bad:** `// Publishes a collision event.`
*   **Good:** `// Publishes an [crate::ecs::event::EventCollision].`

### 2. Unit Specifications
Game engines mix many coordinate systems (Screen vs. World) and time units (Frames vs. Seconds). Every floating-point argument or variable **must** state its unit in the Level 2 (`///`) comment to prevent "floaty" physics bugs.

*   **Example:**
    ```rust
    /// * `velocity`: The speed in **World Units per Second** (px/s).
    /// * `duration`: The effect duration in **Seconds** (s).
    ```

### 3. The "Hotpath" Marker
Functions that are executed 120 times per second (in the fixed loop) or inside inner loops must be marked. This serves as a warning to avoid expensive operations like Heap Allocation (`Vec::new`), Cloning, or File I/O.

*   **Syntax:**
    ```rust
    /// ⚠️ **Hotpath**: Called 120x per second. Avoid heap allocations.
    fn update_physics(...)
    ```

### 4. Event Contracts (Side Effects)
In our Event-Driven WYSIWID architecture, a function's return signature often hides its true impact. You must list the Events that a function publishes in a specific `# Side Effects` section.

*   **Syntax:**
    ```rust
    /// Resolves the interaction between a player and an enemy.
    ///
    /// # Side Effects
    /// * Publishes [crate::ecs::event::EventPlayerDied] if the player touches the enemy's side.
    /// * Publishes [crate::ecs::event::EventEnemyStomped] if the player touches the enemy's top.
    fn resolve_interaction(...)
    ```

---

## Documentation Enforcement
*   **No Magic Numbers:** All values must be described via configuration or constants.
*   **Atomic Modules:** Documentation must strictly reflect the Single Responsibility of the module. If you cannot describe the module's identity in one clear sentence, the file is likely too large and must be refactored.