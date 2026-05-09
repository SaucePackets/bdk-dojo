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
