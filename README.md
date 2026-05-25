# Rust Type System Quirks

Welcome to the companion repository for the **[Rust Type System Quirks](https://www.yarne.me/blog?tags=Rust+Quirks)** blog series.

## The Series

| #   | Quirk          | Crate                                | Blog Post                                                        | Description                                                                                               |
| --- | -------------- | ------------------------------------ | ---------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| 01  | **GhostCells** | [`01-ghost-cells`](./01-ghost-cells) | [From Lifetimes to Ghosts](https://www.yarne.me/blog/ghostcells) | Using invariant lifetimes and generative closures as zero-cost compile-time proofs to tame cyclic graphs. |

## How to use this repository

This project is structured as a **Cargo Workspace**. Each quirk has its own dedicated crate containing library implementations, runnable binaries, and integration tests.

You can build the entire workspace from this root directory:

```bash
cargo build
```

### Running the interactive examples

Each crate contains runnable examples in its `src/bin/` directory. To run a specific example from the root of the workspace, use the `--bin` flag:

```bash
# Example: Running the AST demo from Quirk 01
cargo run --bin ast_demo
```

### Compile fail tests

Because this series focuses on using the type system, proving that the compiler rejects bad code is just as important as proving it accepts good code.

We use the `trybuild` crate to run "UI tests." These tests intentionally fail to compile, and we assert that the compiler throws the exact error messages we expect.

To run all tests (including the compile-fail tests) across the whole workspace:

```bash
cargo test
```
