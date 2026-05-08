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
