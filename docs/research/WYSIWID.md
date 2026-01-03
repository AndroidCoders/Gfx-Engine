# Research: The WYSIWID Pattern

**Source Paper:** [What You See Is What It Does: A Structural Pattern for Legible Software](https://arxiv.org/abs/2508.14511)

This document summarizes the key concepts from the "What You See Is What It Does" (WYSIWID) paper and outlines a plan for applying its principles to the `Gfx-Engine` project to improve modularity, flexibility, and legibility.

### Summary of the WYSIWID Pattern

1.  **Concepts as Independent Services:** The paper proposes structuring software as a collection of independent "Concepts" that act as self-contained services. In `Gfx-Engine`, these would be our core modules: `Physics`, `Rendering`, `Audio`, `Input`, etc. The goal is for each of these services to manage its own internal state and logic, without having direct knowledge of the others.

2.  **Synchronizations as Event-Based Rules:** Instead of services calling each other directly, they communicate indirectly through "Synchronizations." These are essentially event-based rules that define how the services interact. This is a perfect match for the **Event Bus** we've been designing. A `System` (part of a Concept) publishes an `Event`, and a `Conductor` (another Concept) reacts to it.

3.  **Legibility and Modularity:** The result is a highly "legible" architecture. To understand what the software *does*, you simply read the list of `Synchronizations` (events). To understand *how* it does it, you look inside the implementation of a specific `Concept` (system). This makes the entire application much easier to reason about, modify, and extend.

### How This is Applied in `Gfx-Engine`

The engine successfully implements the WYSIWID pattern using a modular ECS structure:

**1. Concepts as Modular Systems:**
Core logic is encapsulated in modular systems (e.g., `SystemPhysics`, `SystemAudio`, `SystemLifecycle`) implementing a unified `System<T>` trait.

**2. The Explicit Scheduler (`SystemManager`):**
Instead of a generic `ServiceManager`, the engine uses an explicit `SystemManager`. This manager defines the exact execution order using a `match` block on the `GameState` resource, making the engine's behavior legible at a glance.

**3. The Synchronization Layer:**
The `SystemSynchronization` and `SystemAudioSynchronization` acts as the "glue" layer, listening for strongly-typed events from the `EventBus` and triggering secondary actions in other systems.

### The End Result

By taking these steps, we would transform the engine's architecture to be a clear reflection of the WYSIWID pattern. The `ServiceManager` would show you *what* the engine is composed of, and the `EventBus` and its events would show you *what it does*. This would make the project significantly more modular, flexible, and easier for any developer to understand and contribute to.
