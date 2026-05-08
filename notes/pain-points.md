# Pain points

## Active

- OutPoint: understand it as the exact pointer to a transaction output: `txid + vout`.

## Resolved

- Rust module scope for beginner tests: single-module tests can live beside the module; cross-module tests fit better in `src/lib.rs` or `tests/`.
- `cargo test <word>` filters tests by name.
