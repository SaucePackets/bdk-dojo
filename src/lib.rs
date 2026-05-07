#[derive(Debug)]
pub struct Utxo {
    pub value: u64,
    pub confirmed: bool,
    pub spendable: bool,
}

#[derive(Debug)]
pub struct BalanceSummary {
    pub confirmed: u64,
    pub trusted_pending: u64,
    pub untrusted_pending: u64,
    pub total_spendable: u64,
}

pub fn calculate_balance(utxos: &[Utxo]) -> BalanceSummary {
    let mut confirmed = 0;
    let mut trusted_pending = 0;
    let mut untrusted_pending = 0;
    let mut total_spendable = 0;

    for utxo in utxos {
        if utxo.confirmed == true && utxo.spendable == true {
            confirmed += utxo.value;
        } else if utxo.confirmed == false && utxo.spendable == false {
            trusted_pending += utxo.value;
        } else if utxo.confirmed == false && utxo.spendable == false {
            untrusted_pending += utxo.value;
        }
    }

    total_spendable = confirmed + trusted_pending;

    BalanceSummary {
        confirmed: confirmed,
        trusted_pending: trusted_pending,
        untrusted_pending: untrusted_pending,
        total_spendable: total_spendable,
    }
}
