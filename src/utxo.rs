use crate::amount::Amount;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutPoint {
    pub txid: String,
    pub vout: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Utxo {
    pub outpoint: OutPoint,
    pub value: Amount,
    pub confirmed: bool,
    pub spendable: bool,
    pub seen_at_height: Option<u32>,
}
