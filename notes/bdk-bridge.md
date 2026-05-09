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

## 1.2 — Total balance

Toy concept:

- `calculate_balance(&[Utxo]) -> u64`

Closest BDK concept/API/example:

- Wallet balance is derived from wallet-owned UTXOs.
- Real BDK exposes richer balance categories; this kata only totals sats.

What the toy hides:

- Confirmation state, trust policy, coinbase maturity, chain position, and spendability rules.

What to read next:

- Lesson 1.3 splits total sats into simple trust and spendability buckets.
