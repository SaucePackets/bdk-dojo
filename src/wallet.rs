use crate::balance::{BalanceSummary, classify_balance};
use crate::utxo::Utxo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletState {
    pub utxos: Vec<Utxo>,
    pub tip_height: u32,
}

impl WalletState {
    pub fn new(tip_height: u32) -> Self {
        WalletState {
            utxos: [],
            tip_height,
        }
    }

    pub fn balance(&self) -> BalanceSummary {
        classify_balance(&self.utxos)
    }
}
