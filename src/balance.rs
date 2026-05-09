use crate::utxo::Utxo;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BalanceSummary {
    pub confirmed: u64,
    pub trusted_pending: u64,
    pub untrusted_pending: u64,
    pub total_spendable: u64,
}

pub fn classify_balance(utxos: &[Utxo]) -> BalanceSummary {
    let mut confirmed = 0;
    let mut trusted_pending = 0;
    let mut untrusted_pending = 0;

    for utxo in utxos {
        if utxo.confirmed && utxo.spendable {
            confirmed += utxo.value.to_sats();
        } else if !utxo.confirmed && utxo.spendable {
            trusted_pending += utxo.value.to_sats();
        } else if !utxo.confirmed && !utxo.spendable {
            untrusted_pending += utxo.value.to_sats();
        }
    }

    let total_spendable = confirmed + trusted_pending;
    BalanceSummary {
        confirmed,
        trusted_pending,
        untrusted_pending,
        total_spendable,
    }
}

pub fn calculate_balance(utxos: &[Utxo]) -> u64 {
    let mut total_balance = 0;
    for utxo in utxos {
        total_balance += utxo.value.to_sats();
    }

    total_balance
}
