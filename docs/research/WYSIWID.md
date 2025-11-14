# Research: The WYSIWID Pattern

**Source Paper:** [What You See Is What It Does: A Structural Pattern for Legible Software](https://arxiv.org/abs/2508.14511)

This document summarizes the key concepts from the "What You See Is What It Does" (WYSIWID) paper and outlines a plan for applying its principles to the `Gfx-Engine` project to improve modularity, flexibility, and legibility.

### Summary of the WYSIWID Pattern

1.  **Concepts as Independent Services:** The paper proposes structuring software as a collection of independent "Concepts" that act as self-contained services. In `Gfx-Engine`, these would be our core modules: `Physics`, `Rendering`, `Audio`, `Input`, etc. The goal is for each of these services to manage its own internal state and logic, without having direct knowledge of the others.

2.  **Synchronizations as Event-Based Rules:** Instead of services calling each other directly, they communicate indirectly through "Synchronizations." These are essentially event-based rules that define how the services interact. This is a perfect match for the **Event Bus** we've been designing. A `System` (part of a Concept) publishes an `Event`, and a `Conductor` (another Concept) reacts to it.

3.  **Legibility and Modularity:** The result is a highly "legible" architecture. To understand what the software *does*, you simply read the list of `Synchronizations` (events). To understand *how* it does it, you look inside the implementation of a specific `Concept` (system). This makes the entire application much easier to reason about, modify, and extend.

### How to Apply This to `Gfx-Engine`

The `Gfx-Engine` is already partially aligned with this pattern, but we can take it much further. The `app.rs` file currently acts as a central orchestrator that has direct dependencies on almost every other module. We can improve this by treating our core modules as true, independent services.

Here is a proposed plan to refactor the engine towards the WYSIWID pattern:

**1. Formalize "Concepts" with a `Service` Trait:**

We can introduce a generic `Service` trait that all our major modules will implement.

```rust
// In a new file, e.g., src/service.rs
use crate::ecs::world::World;

pub trait Service {
    fn initialize(&mut self, world: &mut World);
    fn update(&mut self, world: &mut World, context: &mut SystemContext);
    // Potentially add shutdown(), etc. later
}
```

**2. Create a `ServiceManager`:**

We can create a `ServiceManager` that is responsible for holding all the services and running them in the correct order. This would dramatically simplify the main loop in `app.rs`.

```rust
// In a new file, e.g., src/service_manager.rs
pub struct ServiceManager {
    services: Vec<Box<dyn Service>>,
}

impl ServiceManager {
    pub fn add(&mut self, service: Box<dyn Service>) { ... }
    pub fn update_all(&mut self, world: &mut World, context: &mut SystemContext) {
        for service in &mut self.services {
            service.update(world, context);
        }
    }
}
```

**3. Refactor `app.rs` to Use the `ServiceManager`:**

The main loop in `app.rs` would become much cleaner. It would simply initialize the `ServiceManager`, add all the services (Physics, Rendering, Audio, etc.), and then call `service_manager.update_all()` each frame.

**4. Fully Implement the Event Bus (The "Synchronization" Layer):**

This is the most critical piece. By implementing the type-based event bus we designed, we provide the "Synchronization" layer that allows the independent services to communicate without being coupled to each other.

*   The `PhysicsSystem` (part of the `PhysicsService`) would publish a `CollisionEvent`.
*   The `AudioService` would contain the `AudioConductorSystem`, which would listen for `CollisionEvent`s and play a sound.
*   The `PhysicsService` would have no knowledge of the `AudioService`, and vice-versa.

### The End Result

By taking these steps, we would transform the engine's architecture to be a clear reflection of the WYSIWID pattern. The `ServiceManager` would show you *what* the engine is composed of, and the `EventBus` and its events would show you *what it does*. This would make the project significantly more modular, flexible, and easier for any developer to understand and contribute to.
