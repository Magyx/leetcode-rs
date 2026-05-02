# leetcode

A Rust workspace for solving LeetCode problems with an auto-discovery runner, built-in profiling, and clipboard support.

## How it works

Each solution lives in its own file under `src/solutions/` (e.g. `c0001.rs`, `c0002.rs`, …). A `build.rs` script scans that directory at compile time, parses every `impl Solution` block with `syn`, and generates a registry so solutions can be selected and run without any manual wiring.

Common data structures like `ListNode` are provided in `src/prelude.rs` and serialized automatically from JSON input.

## Usage

```bash
# Interactive selector — lists all solutions and prompts for input
cargo run

# Run a solution directly by index or function name
cargo run -- 0 '[2,7,11,15]' '9'

# Copy a solution to the clipboard (strips local scaffolding)
cargo xcopy 0
```

Arguments are passed as JSON values. You can supply a single JSON array or space-separated JSON literals.

## Adding a solution

Create a new file in `src/solutions/`, for example `c0042.rs`:

```rust
// LOCAL
pub struct Solution;
// LOCAL END

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        todo!()
    }
}
```

The `// LOCAL … // LOCAL END` block is stripped when copying to the clipboard so only the LeetCode-compatible code remains. Everything else is picked up automatically on the next build.

## Profiling

Every solution run prints elapsed time and peak memory via a `ProfileScope` guard and a custom global allocator, so you can quickly spot performance regressions.

## Project structure

```
src/
├── main.rs          # CLI: interactive selector + direct invocation
├── prelude.rs       # Shared types (ListNode, …) and JSON conversions
├── profiler.rs      # Timing & memory-tracking allocator
├── utils.rs         # Clipboard helpers, paste-ready code extraction
├── registry.rs      # (generated) solution registry
└── solutions/
    ├── mod.rs        # (generated) module declarations
    ├── c0001.rs      # Two Sum
    ├── c0002.rs      # Add Two Numbers
    └── ...
```

## Requirements

- **Rust nightly** (edition 2024)
- A clipboard utility (`xclip` / `xsel` / `wl-copy`) on Linux for the `copy` command
