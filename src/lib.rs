pub mod amount;
pub mod balance;
pub mod utxo;

pub use amount::Amount;
pub use balance::calculate_balance;
pub use utxo::{OutPoint, Utxo};

pub fn dojo_ready() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fresh_repo_is_ready() {
        assert!(dojo_ready());
    }

    #[test]
    fn utxo_stores_outpoint_and_value() {
        let utxo = Utxo {
            outpoint: OutPoint {
                txid: "00".repeat(32),
                vout: 0,
            },
            value: Amount::from_sats(12_345),
        };

        assert_eq!(utxo.outpoint.vout, 0);
        assert_eq!(utxo.value.to_sats(), 12_345);
        assert_eq!(utxo.outpoint.txid, "00".repeat(32));
    }

    #[test]
    fn calculate_balance_empty_wallet_is_zero() {
        let utxos = [];

        assert_eq!(calculate_balance(&utxos), 0);
    }

    #[test]
    fn calculate_balance_sums_all_utxos() {
        let utxos = [
            Utxo {
                outpoint: OutPoint {
                    txid: "11".repeat(32),
                    vout: 0,
                },
                value: Amount::from_sats(21_000),
            },
            Utxo {
                outpoint: OutPoint {
                    txid: "22".repeat(32),
                    vout: 1,
                },
                value: Amount::from_sats(79_000),
            },
        ];

        assert_eq!(calculate_balance(&utxos), 100_000);
    }
}
