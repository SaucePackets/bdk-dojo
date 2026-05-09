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

## 1.2 — Total balance

Status: complete

Files changed:

- `src/balance.rs`
- `src/lib.rs`

Tests passed:

- `tests::calculate_balance_empty_wallet_is_zero`
- `tests::calculate_balance_sums_all_utxos`

What was learned:

- `calculate_balance` loops through all UTXOs, adds their sats, and returns the total.
- `Amount` should be converted with `to_sats()`, not cast with `as u64`.
- Total balance is deliberately simpler than trust, confirmation, or spendability classification.

Pain point:

- Resolved: `Amount` is a wallet type; `to_sats()` exposes the primitive sat count.

Next lesson:

- 1.3 — Balance buckets

## 1.3 — Balance buckets

Status: complete

Files changed:

- `src/utxo.rs`
- `src/balance.rs`
- `src/lib.rs`

Tests passed:

- `tests::classify_balance_empty_wallet_is_zeroed`
- `tests::classify_balance_separates_trust_and_spendability`

What was learned:

- `calculate_balance` returns only total sats.
- `classify_balance` returns confirmed, trusted pending, untrusted pending, and total spendable buckets.
- A wallet balance is not just one number once trust and spendability matter.

Pain point:

- Resolved: balance category names and their purpose.

Next lesson:

- 1.4 — Wallet state
