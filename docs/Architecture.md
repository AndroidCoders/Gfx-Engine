# Engine Architecture: WYSIWID (What You See Is What It Does)

This document defines the core architectural pattern of the `Gfx-Engine`, inspired by the research paper: [**"What You See Is What It Does: A Structural Pattern for Legible Software"**](https://arxiv.org/abs/2508.14511).

## 1. High-Level Philosophy

The primary goal of this architecture is **Legibility**. In a traditional game engine, logic is often "entangled"—the Physics system might directly call the Audio system to play a sound, or the Player state might directly update an animation frame. This creates "hidden" dependencies that make the code harder to reason about and risky to modify.

**WYSIWID** solves this by ensuring that the code directly reflects the observed behavior of the software. It partitions the engine into two distinct, decoupled layers:

1.  **Concepts (The "What"):** These are the independent building blocks of the engine (e.g., Physics, Health, Input). They manage their own data and logic but have **zero knowledge** of other concepts.
2.  **Synchronizations (The "How"):** These are the explicit "behavioral rules" that bridge concepts (e.g., "When a player hits an enemy, deduct health"). 

### Benefits for Game Engine Development:
*   **Decoupled Presentation:** Core logic (Physics/State) is never polluted by presentation logic (Audio/Visuals). You can change the "Stomp" animation without ever touching the "Stomp" physics.
*   **AI-Assisted Productivity:** By localizing behavior into small, declarative rules, an LLM coding assistant can add features or fix bugs with a minimal "blast radius," ensuring high safety and accuracy.
*   **Modular Extensibility:** New features (e.g., "Collectibles," "Power-ups") are added by creating new Concepts and Synchronizations, rather than modifying existing monolithic systems.

---

## 2. Technical Implementation

### A. Concepts (Atomic Services)
Concepts are implemented as **ECS Systems** that operate on a specific set of components. 
*   **Constraint:** A Concept system never reads the `EventBus` to make decisions; it performs its task and publishes **Facts** (Events) or reacts to **Intents** (Commands).
*   **Example:** `ConceptPhysics` integrates velocity into position. It doesn't care *why* the velocity changed.

### B. Synchronizations (Behavioral Rules)
Synchronizations are implemented as **Rule Systems** (often called "Conductors").
*   **Logic Pattern:** `when <Event> [where <Condition>] then <Action>`
*   **Mechanism:** They subscribe to the `EventBus`, interpret the "Facts," and issue "Commands" to other Concepts.
*   **Example:** `RuleInteraction` listens for `EventCollision`. It identifies a "Player vs Enemy" fact and issues a `CommandDamage` to the `ConceptHealth` system.

### C. The Command Pattern
To maintain purity, Synchronizations do not modify component data directly. Instead, they publish **Commands** (Intents) that the relevant Concept system then executes. This ensures that the Concept remains the "Single Authority" over its domain.

## 3. Legibility through Documentation

For a system to be truly "What You See Is What It Does," the code must be accompanied by explicit human-and-AI-readable metadata.

The `Gfx-Engine` implements a mandatory **3-Level Documentation Standard**:
1.  **Module Identity**: Every file defines its architectural role at the top.
2.  **Function Intent**: Every function defines its semantic purpose.
3.  **Logical Blueprint**: Function bodies use step-by-step comments to explain the "Why" and the logic flow.

See [**Documentation.md**](../docs/Documentation.md) for full details.

---

## 4. The "Atomic Module" Rule

To maximize comprehension and legibility, the `Gfx-Engine` enforces strict limits on source code structure:

*   **File Size Limit:** Source code modules (`.rs` files) should ideally be between **200-300 lines**. Any file exceeding **500 lines** must be refactored.
*   **Single Responsibility:** Each module must represent **one** atomic process. A file should contain one Concept or one set of related Synchronization Rules—never both.
*   **Small Blast Radius:** This ensures that the entire logic of a process fits within the context window of an AI assistant, minimizing hallucinations and errors.

---

## 4. Refactoring Roadmap (The Path to Pure WYSIWID)

The current engine is a "Hybrid" model and requires the following changes to align with the pure WYSIWID pattern:

### Phase 1: Decomposing the "God Systems"
*   **`SystemLifecycle`**: Split into `ConceptHealth` (arithmetic), `ConceptVitality` (timers), and `RuleInteraction` (interpretation).
*   **`SystemWorldLevelTransition`**: Split into `ConceptGoalDetector` and `RuleLevelTransition`.

### Phase 2: Purifying State Machines
*   Current HSM states (e.g., `JumpingState`) trigger sounds and animations. These must be refactored to publish **Facts** (e.g., `EventEntityJumped`).
*   `SystemAudioSynchronization` and `SystemAnimationSynchronization` will become the sole authorities for presentation.

### Phase 3: Transition to Commands
*   Update systems to consume **Commands** (e.g., `CommandMove`, `CommandDamage`) rather than having Synchronizations modify `Velocity` or `Health` directly.

### Phase 4: Topic-Based Routing
*   Upgrade the `EventBus` to support hierarchical strings (e.g., `player.collision.enemy`) to allow for more granular, declarative rule-matching.