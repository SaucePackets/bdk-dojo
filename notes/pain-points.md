# Pain points

## Active

- Wallet state boundaries: keep balance classification in `balance.rs` and let wallet domain objects delegate to it.

## Resolved

- Balance categories: distinguish total sats from confirmed, pending, trusted, and spendable sats.
- OutPoint: understand it as the exact pointer to a transaction output: `txid + vout`.
- `Amount` conversion: use `to_sats()` instead of casting the custom struct.
- Rust module scope for beginner tests: single-module tests can live beside the module; cross-module tests fit better in `src/lib.rs` or `tests/`.
- `cargo test <word>` filters tests by name.
