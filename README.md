# Tokio Meetup: Async Rust Presentation

This repository contains the code examples and materials for a presentation on **Asynchronous Rust** and the **Tokio** runtime. It demonstrates how to write, understand, and spawn asynchronous tasks using Rust's `async`/`await` syntax and the Tokio ecosystem.

## Project Structure

The project is structured as a series of runnable binaries that progressively introduce async Rust concepts. These correspond to different stages or slides in the presentation.

* `src/rust_future.rs`: Contains examples of basic `Future` trait implementations and mechanics.
* `src/bin/`: Contains the runnable examples demonstrating various concepts:
  * `v1.rs` - `v5.rs`: Step-by-step progression of an async application, showing the evolution from basic futures to more complex concurrent Tokio patterns.
  * `spawn_tokio.rs`: Demonstrates how to spawn tasks correctly using `tokio::spawn` for concurrent execution.

## Prerequisites

To run these examples, you will need to have [Rust and Cargo installed](https://rustup.rs/).

## Running the Examples

You can run each example individually using Cargo's `--bin` flag. For example, to run the first iteration:

```bash
cargo run --bin v1
```

To run the Tokio spawning example:

```bash
cargo run --bin spawn_tokio
```

Explore `v1` through `v5` by substituting the binary name in the command above to see how the code evolves.

## License

This project is dual-licensed under either the **MIT License** or the **Apache License, Version 2.0**. See the `LICENSE-MIT` and `LICENSE-APACHE` files for more details.