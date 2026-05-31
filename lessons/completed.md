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

## 1.4 — Wallet state

Status: complete

Files changed:

- `src/wallet.rs`
- `src/lib.rs`

Tests passed:

- `tests::empty_wallet_balance_is_zeroed`
- `tests::wallet_balance_delegates_to_classify_balance`

What was learned:

- `WalletState` owns the UTXO set — it is the wallet's domain object.
- `tip_height` tracks the chain tip for confirmation logic later.
- `WalletState::balance()` delegates to `classify_balance(&self.utxos)` instead of duplicating the bucket rules.
- `WalletState::new(tip_height)` creates an empty wallet at a given chain tip.

Pain point:

- Resolved: `utxos` field uses `Vec::new()` in the constructor, not an array literal.

Next lesson:

- 2.1 — Confirmation depth

## 2.1 — Confirmation depth

Status: complete

Files changed:

- `src/utxo.rs` (added `seen_at_height`)
- `src/chain.rs`
- `src/lib.rs` (tests)

Tests passed:

- `tests::unseen_utxo_has_zero_confirmations`
- `tests::utxo_seen_at_tip_has_one_confirmation`
- `tests::older_utxo_counts_confirmations_inclusively`
- `tests::future_seen_height_has_zero_confirmations`

What was learned:

- Confirmations = tip_height - seen_at_height + 1 (inclusive count).
- A UTXO seen at block 100 and confirmed at tip 100 has one confirmation, not zero.
- Unseen UTXOs (`seen_at_height = None`) have zero confirmations.
- A UTXO with a future `seen_at_height` (reorg edge case) also returns zero.

Pain point:

- The +1 in the confirmation formula is easy to get wrong. Inclusive counting matters.

Next lesson:

- 2.2 — Spendability policy

## 2.2 — Spendability policy

Status: complete

Files changed:

- `src/utxo.rs` (added `coinbase`, `locked_until`, `owned`)
- `src/chain.rs` (`is_spendable`, `COINBASE_MATURITY`)
- `src/lib.rs` (tests)

Tests passed:

- `tests::normal_owned_unlocked_utxo_is_spendable`
- `tests::spendability_rejects_immature_coinbase_locked_and_foreign_utxos`

What was learned:

- Spendability is not the same as confirmation. Three checks: ownership, timelock maturity, coinbase maturity (100 blocks).
- `COINBASE_MATURITY = 100` mirrors Bitcoin Core's rule.
- A UTXO can be confirmed but unspendable (coinbase too young, timelocked).
- `is_spendable()` is a pure function — no mutation, just reads.

Pain point:

- `is_spendable` signature changed mid-lesson to accept `tip_height` instead of relying on `seen_at_height` alone. The function needs chain context.

Next lesson:

- 2.3 — Sync events

## 2.3 — Sync events

Status: complete

Files changed:

- `src/wallet.rs` (`SyncEvent` enum, `apply()`)
- `src/lib.rs` (tests, export)

Tests passed:

- `tests::wallet_apply_tracks_found_confirmed_spent_and_reorged_utxos`

What was learned:

- A wallet processes chain data as a sequence of sync events: found, confirmed, spent, reorged, tip advanced.
- `apply()` matches on the event variant and mutates wallet state accordingly.
- `Confirmed` sets `seen_at_height` and flips the confirm flag.
- `Spent` uses `retain()` to remove the UTXO.
- `Reorged` unconfirms without removing — the UTXO might reappear.
- `Found` pushes a new UTXO the wallet didn't know about.

Pain point:

- `iter_mut().find(|u| ...)` and `retain(|u| ...)` took a few tries. Closure syntax is worth revisiting.

Next lesson:

- 2.4 — Checkpoints and reorgs

## 2.4 — Checkpoints and reorgs

Status: complete

Files changed:

- `src/wallet.rs` (`checkpoints` field, `rollback_to_height()`)
- `src/lib.rs` (tests)

Tests passed:

- `tests::rollback_unconfirms_utxos_above_new_tip`

What was learned:

- `checkpoints` stores a history of tip heights so the wallet can rewind.
- `rollback_to_height()` strips checkpoints above the new tip and unconfirms UTXOs confirmed after it.
- A reorg does not delete UTXOs — it unconfirms them. They might confirm again on the new chain.
- `map_or(false, |h| h > height)` is the Rust idiom for "if Some and the value is above height."

Pain point:

- `retain()` on checkpoints and iterating UTXOs for the rollback are two different operations — keeping them straight was the challenge.

Next lesson:

- 2.5 — Address index and gap limit

## 2.5 — Address index and gap limit

Status: complete

Files changed:

- `src/wallet.rs` (`AddressRecord` struct, `addresses` field, `next_unused_address()`)
- `src/lib.rs` (test, export)

Tests passed:

- `tests::next_unused_address_reuses_until_marked_used_then_derives_next`

What was learned:

- Wallets derive addresses on demand, not all at once. The gap limit caps how many consecutive unused addresses to scan before giving up.
- `next_unused_address()` reuses the first unused record; if none exists, derives the next index from `self.addresses.len()`.
- The `used` flag gets flipped externally (by sync logic when a UTXO lands) — the method only reads it.
- BIP 44 standard gap limit is 20. This toy exercise models the derive-on-demand pattern.

Pain point:

- Resolved: understanding that `len()` is the next index when the list is 0-indexed and contiguous.
- Resolved: `iter()` reads; `.push()` mutates. Two separate operations, not one.

Next lesson:

- 3.1 — Fee rates and vbytes
