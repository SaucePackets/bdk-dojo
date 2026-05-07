#[derive(Debug)]
pub struct Utxo {
    pub value: u64,
    pub confirmed: bool,
    pub spendable: bool,
}

#[derive(Debug, PartialEq, Eq)]
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

    for utxo in utxos {
        if utxo.confirmed && utxo.spendable {
            confirmed += utxo.value;
        } else if !utxo.confirmed && utxo.spendable {
            trusted_pending += utxo.value;
        } else if !utxo.confirmed && !utxo.spendable {
            untrusted_pending += utxo.value;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_wallet_balance_from_utxos() {
        let utxos = vec![
            Utxo {
                value: 50_000,
                confirmed: true,
                spendable: true,
            },
            Utxo {
                value: 20_000,
                confirmed: false,
                spendable: true,
            },
            Utxo {
                value: 10_000,
                confirmed: false,
                spendable: false,
            },
        ];

        let summary = calculate_balance(&utxos);

        assert_eq!(
            summary,
            BalanceSummary {
                confirmed: 50_000,
                trusted_pending: 20_000,
                untrusted_pending: 10_000,
                total_spendable: 70_000,
            }
        );
    }
}
