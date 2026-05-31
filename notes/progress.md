# Progress

## 1.1 — Amounts and UTXOs

Completed.

Files changed:

- `src/amount.rs`
- `src/utxo.rs`
- `src/lib.rs`

Tests passed:

```text
3 passed
```

Concept learned:

- Wallets track coins as UTXOs.
- `Amount` tells how many sats the coin is worth.
- `OutPoint` points to the exact transaction output: `txid + vout`.
- `Utxo` combines the location (`OutPoint`) and value (`Amount`).

Rust pain point:

- Test placement and module scope.
- `cargo test <word>` filters tests by name.

Bitcoin/BDK pain point:

- OutPoint needs more practice.

Next lesson:

- 1.2 — Total balance

## 1.2 — Total balance

Completed.

Files changed:

- `src/balance.rs`
- `src/lib.rs`

Tests passed:

```text
5 passed
```

Concept learned:

- `calculate_balance` loops through all UTXOs, adds each `Amount` as sats, and returns the total.
- `Amount::to_sats()` is the safe conversion boundary from wallet type to raw `u64`.
- Plain total balance does not care whether a UTXO is confirmed or spendable.

Rust pain point:

- Resolved: do not cast a custom struct with `as u64`; call the explicit method.

Bitcoin/BDK pain point:

- Balance can mean different things. Lesson 1.2 only means total sats.

Next lesson:

- 1.3 — Balance buckets

## 1.3 — Balance buckets

Completed.

Files changed:

- `src/utxo.rs`
- `src/balance.rs`
- `src/lib.rs`

Tests passed:

```text
7 passed
```

Concept learned:

- `confirmed`, `trusted_pending`, and `untrusted_pending` describe different kinds of sats.
- `total_spendable` is confirmed spendable sats plus trusted pending spendable sats.
- Confirmed but unspendable sats are ignored in this beginner model.

Rust pain point:

- Adding fields to `Utxo` means older tests must construct those fields too.

Bitcoin/BDK pain point:

- Real wallet balance APIs expose categories because trust and spendability affect what the wallet can safely use.

Next lesson:

- 1.4 — Wallet state

## 1.4 — Wallet state

Completed.

Files changed:

- `src/wallet.rs`
- `src/lib.rs`

Tests passed:

```text
9 passed
```

Concept learned:

- `WalletState` owns the UTXO set as the wallet domain object.
- `tip_height` tracks chain tip for upcoming confirmation lessons.
- `WalletState::balance()` delegates to `classify_balance(&self.utxos)` — no duplication.
- `WalletState::new(tip_height)` constructs an empty wallet.

Rust pain point:

- Resolved: constructor uses `Vec::new()`, not an array literal.

Bitcoin/BDK pain point:

- Real BDK wallet state is larger (descriptors, indexed tx graph, persistence, sync state).

Next lesson:

- 2.1 — Confirmation depth

## 2.1 — Confirmation depth

Completed.

Files changed:

- `src/utxo.rs` (added `seen_at_height`)
- `src/chain.rs`
- `src/lib.rs`

Tests passed:

```text
13 passed
```

Concept learned:

- Confirmations = `tip_height - seen_at_height + 1` (inclusive).
- Seen at tip = 1 confirmation, not 0.
- Unseen (`None`) = 0 confirmations.
- Future seen height returns 0 (reorg edge case safety).

Rust pain point:

- `match utxo.seen_at_height` with guard clause `if height <= tip_height` took a moment.

Bitcoin/BDK pain point:

- Real wallets don't compute confirmations one UTXO at a time; they track block height and derive depth per tx.

Next lesson:

- 2.2 — Spendability policy

## 2.2 — Spendability policy

Completed.

Files changed:

- `src/utxo.rs` (added `coinbase`, `locked_until`, `owned`)
- `src/chain.rs` (`is_spendable`, `COINBASE_MATURITY`)

Tests passed:

```text
15 passed
```

Concept learned:

- Three spendability gates: owned, timelock maturity, coinbase maturity.
- `COINBASE_MATURITY = 100` mirrors Core's `COINBASE_MATURITY`.
- Confirmed ≠ spendable (coinbase too young, timelocked).
- `is_spendable` is a pure read function — no mutation.

Rust pain point:

- `is_spendable` signature had to accept `tip_height` as a parameter, not compute it internally.

Bitcoin/BDK pain point:

- BDK's `SpendableTxIn` and `coin_select` layers encode these same rules with much more nuance (CSV/CLTV, RBF signaling, mempool ancestor limits).

Next lesson:

- 2.3 — Sync events

## 2.3 — Sync events

Completed.

Files changed:

- `src/wallet.rs` (`SyncEvent` enum, `apply()` method)
- `src/lib.rs`

Tests passed:

```text
16 passed
```

Concept learned:

- Chain data arrives as a sequence of discrete events: Found, Confirmed, Spent, Reorged, TipAdvanced.
- `apply()` matches the event and mutates wallet state — the wallet is an event-driven state machine.
- `Spent` removes the UTXO; `Reorged` unconfirms without removing (it may reappear).
- `Found` discovers new UTXOs the wallet didn't know about.

Rust pain point:

- `iter_mut().find(|u| ...)` for mutation, `retain(|u| ...)` for removal. Different closure patterns.

Bitcoin/BDK pain point:

- Real BDK sync processes blocks/compact block filters, not simple enum events. The event abstraction is a teaching simplification.

Next lesson:

- 2.4 — Checkpoints and reorgs

## 2.4 — Checkpoints and reorgs

Completed.

Files changed:

- `src/wallet.rs` (`checkpoints` field, `rollback_to_height()`)
- `src/lib.rs`

Tests passed:

```text
17 passed
```

Concept learned:

- `checkpoints` is a growing list of past tip heights — the wallet's rewind points.
- `rollback_to_height()` strips checkpoints above the target and unconfirms UTXOs seen after it.
- Reorgs unconfirm, not delete. UTXOs may confirm again on the winning chain.
- `TipAdvanced` pushes a checkpoint; the history builds with every new block.

Rust pain point:

- `map_or(false, |h| h > height)` — the idiom for conditional unwrap without an `if let`.

Bitcoin/BDK pain point:

- BDK uses `LocalChain` with anchor heights and `CheckPoint` blocks. The toy `Vec<u32>` is a minimal version of that rewind history.

Next lesson:

- 2.5 — Address index and gap limit

## 2.5 — Address index and gap limit

Completed.

Files changed:

- `src/wallet.rs` (`AddressRecord` struct, `addresses` field, `next_unused_address()`)
- `src/lib.rs`

Tests passed:

```text
18 passed
```

Concept learned:

- Wallets derive addresses on demand, not all at once. The gap limit caps consecutive unused address scanning.
- `next_unused_address()` scans for an unused record; if none, derives the next from `self.addresses.len()`.
- The `used` flag is external state (set when a UTXO lands at that address) — the method is a pure scan + derive.
- BIP 44: standard gap limit is 20 consecutive unused before the wallet stops looking.

Rust pain point:

- Resolved: distinguishing `iter()` (read scan) from `.push()` (mutate). Two operations, not one.
- Resolved: `len()` as the next index when starting from 0.

Bitcoin/BDK pain point:

- Real BDK uses `KeychainTxOutIndex` with lookahead and `spk_txout_index` for address tracking. The toy `Vec<AddressRecord>` is a minimal model of derive-on-demand + reuse.

Next lesson:

- 3.1 — Fee rates and vbytes
