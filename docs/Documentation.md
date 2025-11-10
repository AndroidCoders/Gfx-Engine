# Rust Documentation Best Practices

This document outlines the best practices for documenting Rust code in the Gfx-Engine project. By following these guidelines, we can ensure that our code is easy to understand, use, and maintain.

## Key Principles

*   **Use Doc Comments:**
    *   `///` for functions, structs, enums, and other items.
    *   `//!` for modules and the crate itself (in `main.rs` or `lib.rs`) to provide a general overview.
*   **Leverage Markdown:** Use Markdown for formatting, including headings (`#`), lists (`*`), and code blocks (```rust).
*   **Provide Runnable Examples:** The `# Examples` section is crucial. These examples are automatically tested with `cargo test`, ensuring they are always correct.
*   **Document Potential Issues:** Use dedicated sections to explain potential problems:
    *   `# Panics`: For situations that will cause the program to crash.
    *   `# Errors`: For explaining the different kinds of `Result` errors a function can return.
    *   `# Safety`: For `unsafe` code, to explain the responsibilities of the programmer using the code.
*   **Generate and Review:** Use `cargo doc --open` to generate the HTML documentation and review it in a browser.

## Crate-level Documentation

Crate-level documentation provides an overview of your library or application. It's typically placed at the top of your `src/lib.rs` (for libraries) or `src/main.rs` (for binaries) file using `//!` comments.

```rust
//! # My Awesome Crate
//!
//! `my_awesome_crate` is a library for doing amazing things.
//!
//! This crate provides utilities for [briefly describe main features].
//!
//! ## Getting Started
//!
//! To use `my_awesome_crate`, add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! my_awesome_crate = "0.1.0"
//! ```
//!
//! Then, you can use it in your code:
//!
//! ```rust
//! use my_awesome_crate::some_function;
//!
//! fn main() {
//!     some_function();
//! }
//! ```
```

## Module-level Documentation

Module-level documentation describes the purpose of a specific module and how its components interact. Like crate-level docs, it uses `//!` comments, usually at the top of the module file.

```rust
//! This module handles all rendering operations for the game engine.
//!
//! It provides the `Renderer` struct which manages the drawing context,
//! textures, and sprites.
pub mod renderer {
    // ... module content ...
}
```

## Item-level Documentation

For individual items like functions, structs, enums, and traits, use `///` comments directly above the item.

```rust
/// Represents a 2D point in screen coordinates.
///
/// # Examples
///
/// ```rust
/// let p = Point { x: 10, y: 20 };
/// assert_eq!(p.x, 10);
/// ```
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    /// Creates a new `Point` at the specified coordinates.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let p = Point::new(5, 10);
    /// assert_eq!(p.x, 5);
    /// assert_eq!(p.y, 10);
    /// ```
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}
```