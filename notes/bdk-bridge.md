# BDK bridge

## 1.1 — Amounts and UTXOs

Toy concept:

- `Amount`
- `OutPoint`
- `Utxo`

Closest BDK concept/API/example:

- BDK and Bitcoin libraries model coins as transaction outputs.
- An outpoint identifies a specific output from a specific transaction.

What the toy hides:

- Real transaction IDs are hashes, not arbitrary strings.
- Real wallet UTXOs include script/pubkey data, confirmation state, chain position, and spendability policy.

What to read next:

- Reinforce UTXO and OutPoint during lesson 1.2 before summing balances.
