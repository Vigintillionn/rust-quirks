# Quirk 01: From Lifetimes to Ghosts

> **📖 Read the full blog post:** [Rust Quirks 1: From Lifetimes to Ghosts](https://www.yarne.me/blog/ghostcells)

This crate contains the reference implementation for the **GhostCell** pattern. It demonstrates how to decouple data from mutability permissions using invariant lifetimes (`PhantomData<fn(&'id ()) -> &'id ()>`) and Higher-Ranked Trait Bounds (`for<'id>`).

The result is a 100% safe, mutable, cyclic graph with absolutely zero runtime reference-counting overhead.

## What's inside?

- `src/branded_arena.rs`: A standard memory arena that uses an invariant lifetime brand.
- `src/ghost_cell.rs`: The core implementation of `GhostToken` and `GhostCell`.
- `src/bin/ast_demo.rs`: A runnable example of a cyclic Abstract Syntax Tree that separates memory lifetimes (`'a`) from brand lifetimes (`'id`).
- `src/bin/swap.rs`: A runnable example demonstrating how to mutably borrow two distinctly different nodes at the exact same time to swap their data.
- `tests/ui/`: The compile-fail tests.

## Running the Demos

You can run the proof-of-concept binaries to see the graphs successfully build and mutate:

```bash
cargo run --bin ast_demo
cargo run --bin swap
```

## Running the Compile-Fail Tests

The real magic of this crate is what it prevents you from doing. We use `trybuild` to ensure the compiler stops you from creating double-mutable borrows or mixing up arena indices.

Run the tests to see the compiler enforce our type-state boundaries:

```bash
cargo test
```

> Note: If you modify the code in the tests/ui/ directory and want to update the expected compiler error outputs, run `TRYBUILD=overwrite cargo test`.
