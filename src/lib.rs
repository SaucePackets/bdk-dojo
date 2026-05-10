pub mod amount;
pub mod balance;
pub mod utxo;
pub mod wallet;

pub use amount::Amount;
pub use balance::{BalanceSummary, calculate_balance, classify_balance};
pub use utxo::{OutPoint, Utxo};
pub use wallet::WalletState;

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
            confirmed: true,
            spendable: true,
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
                confirmed: true,
                spendable: true,
            },
            Utxo {
                outpoint: OutPoint {
                    txid: "22".repeat(32),
                    vout: 1,
                },
                value: Amount::from_sats(79_000),
                confirmed: true,
                spendable: true,
            },
        ];

        assert_eq!(calculate_balance(&utxos), 100_000);
    }

    #[test]
    fn classify_balance_empty_wallet_is_zeroed() {
        assert_eq!(
            classify_balance(&[]),
            BalanceSummary {
                confirmed: 0,
                trusted_pending: 0,
                untrusted_pending: 0,
                total_spendable: 0,
            }
        );
    }

    #[test]
    fn classify_balance_separates_trust_and_spendability() {
        let utxos = [
            Utxo {
                outpoint: OutPoint {
                    txid: "33".repeat(32),
                    vout: 0,
                },
                value: Amount::from_sats(50_000),
                confirmed: true,
                spendable: true,
            },
            Utxo {
                outpoint: OutPoint {
                    txid: "44".repeat(32),
                    vout: 1,
                },
                value: Amount::from_sats(20_000),
                confirmed: false,
                spendable: true,
            },
            Utxo {
                outpoint: OutPoint {
                    txid: "55".repeat(32),
                    vout: 2,
                },
                value: Amount::from_sats(10_000),
                confirmed: false,
                spendable: false,
            },
            Utxo {
                outpoint: OutPoint {
                    txid: "66".repeat(32),
                    vout: 3,
                },
                value: Amount::from_sats(5_000),
                confirmed: true,
                spendable: false,
            },
        ];

        assert_eq!(
            classify_balance(&utxos),
            BalanceSummary {
                confirmed: 50_000,
                trusted_pending: 20_000,
                untrusted_pending: 10_000,
                total_spendable: 70_000,
            }
        );
    }

    #[test]
    fn empty_wallet_balance_is_zeroed() {
        let wallet = WalletState::new(100);

        assert_eq!(wallet.tip_height, 100);
        assert!(wallet.utxos.is_empty());

        assert_eq!(
            wallet.balance(),
            BalanceSummary {
                confirmed: 0,
                trusted_pending: 0,
                untrusted_pending: 0,
                total_spendable: 0,
            }
        );
    }

    #[test]
    fn wallet_balance_delegates_to_classify_balance() {
        let mut wallet = WalletState::new(100);

        wallet.utxos.push(Utxo {
            outpoint: OutPoint {
                txid: "77".repeat(32),
                vout: 0,
            },
            value: Amount::from_sats(50_000),
            confirmed: true,
            spendable: true,
        });

        wallet.utxos.push(Utxo {
            outpoint: OutPoint {
                txid: "88".repeat(32),
                vout: 1,
            },
            value: Amount::from_sats(20_000),
            confirmed: false,
            spendable: true,
        });

        wallet.utxos.push(Utxo {
            outpoint: OutPoint {
                txid: "99".repeat(32),
                vout: 2,
            },
            value: Amount::from_sats(10_000),
            confirmed: false,
            spendable: false,
        });

        assert_eq!(
            wallet.balance(),
            BalanceSummary {
                confirmed: 50_000,
                trusted_pending: 20_000,
                untrusted_pending: 10_000,
                total_spendable: 70_000
            }
        );

        assert_eq!(wallet.balance(), classify_balance(&wallet.utxos));
    }
}
