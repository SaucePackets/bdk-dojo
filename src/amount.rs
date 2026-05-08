#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Amount {
    sats: u64,
}

impl Amount {
    pub fn from_sats(sats: u64) -> Self {
        Self { sats }
    }

    pub fn to_sats(self) -> u64 {
        self.sats
    }
}

#[test]
fn amount_preserves_sats_exactly()
{
    let amount = Amount::from_sats(50_000);
    assert_eq!(amount.to_sats(), 50_000);
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
