# Completed lessons

## 1.1 — Amounts and UTXOs

Status: complete

Files changed:

- `src/amount.rs`
- `src/utxo.rs`
- `src/lib.rs`

Tests passed:

- `amount::tests::amount_preserves_sats_exactly`
- `tests::utxo_stores_outpoint_and_value`
- `tests::fresh_repo_is_ready`

What was learned:

- `Amount` stores sats exactly as `u64`.
- A UTXO is an unspent transaction output: one wallet coin that can later be spent.
- A wallet needs the amount to know coin value and later calculate spend/change.
- `OutPoint` identifies which exact transaction output the UTXO came from.
- Rust module tests need correct scope/imports.

Pain point:

- `OutPoint` is still fuzzy and should be reinforced in lesson 1.2.
- `cargo test output` filtered tests instead of labeling output.

Next lesson:

- 1.2 — Total balance
