# Testing Strategy (v2 - Blast Radius Standard)

This document defines the testing philosophy and mandatory requirements for the `Gfx-Engine`. Our goal is to ensure that the engine is stable, predictable, and highly resistant to "destructive edits" during refactoring.

## 1. Philosophy: Blast Radius Reduction

In a WYSIWID architecture, we want the "blast radius" of any change to be as small as possible. Testing is the primary tool for verifying this. 

*   **Small Changes, Immediate Verification:** Every refactor or feature must be verified by a test before it is merged.
*   **Identify Destructive Edits:** Tests must be sensitive enough to identify if a change in one system (e.g., Physics) has unintended consequences in another (e.g., Interaction).
*   **Reversion Safety:** If a test fails, we can confidently revert to the last known good state, knowing exactly what broke.

---

## 2. Mandatory Testing Requirements

To achieve our stability goals, the `Gfx-Engine` enforces the following rules:

### A. The "One Test Per Module" Rule
Every source code module (`.rs` file) must contain **at least one** automated test.
*   **Purpose:** Acts as a "Smoke Test" to ensure the module compiles and its basic logic is sound.
*   **Location:** Placed in a `#[cfg(test)] mod tests` block at the bottom of the file.

### B. The "System Contract" Rule
Main gameplay systems (Physics, Interaction, Health, etc.) must have **1-2 specific contract tests**.
*   **Purpose:** Verifies the "Main Responsibility" of the system. 
*   **Example:** A physics test must verify that an entity with a downward velocity actually changes its Y-position after an update.

### C. Doc Tests
All public functions must include a documentation example (`/// # Examples`). These are automatically verified by `cargo test`.

---

## 3. The ECS System Test Pattern

For our Event-Driven ECS, we use a standardized pattern for testing systems in isolation (Headless Mode):

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecs::world::World;
    use crate::ecs::systems::SystemContext;

    #[test]
    fn test_system_contract() {
        // 1. Setup: Create a minimal World and Context.
        let mut world = World::new();
        // 2. Arrange: Add the target entity and its components.
        let entity = world.create_entity();
        world.add_health(entity, Health { current: 10, max: 10 });
        
        // 3. Act: Execute the system's update logic.
        let mut system = ConceptHealth;
        system.update(&mut world, &mut mock_context);

        // 4. Assert: Verify the outcome.
        assert_eq!(world.healths.get(&entity).unwrap().current, 9);
    }
}
```

---

## 4. Industry Best Practice: Regression Replays

To ensure high-fidelity stability, we utilize our **Deterministic Replay System**:
*   **Golden Masters:** We maintain a set of "Golden Replay" files.
*   **Verification:** A test can run a replay headlessly and compare the final `World` state against a known baseline. Any deviation in physics or logic will be immediately flagged.

## 5. Definition of Done
A task is only **"Done"** when:
1.  Logic is implemented and blueprinted (Level 3 comments).
2.  The "One Test Per Module" rule is satisfied.
3.  All existing project tests pass (`cargo test`).