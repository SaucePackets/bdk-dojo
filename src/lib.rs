pub mod amount;
pub mod utxo;

pub use amount::Amount;
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
    }
}
