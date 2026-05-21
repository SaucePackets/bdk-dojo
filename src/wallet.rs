use crate::balance::{BalanceSummary, classify_balance};
use crate::utxo::{OutPoint, Utxo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletState {
    pub utxos: Vec<Utxo>,
    pub tip_height: u32,
    pub checkpoints: Vec<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyncEvent {
    Found(Utxo),
    Confirmed { outpoint: OutPoint, height: u32 },
    Spent(OutPoint),
    Reorged { outpoint: OutPoint },
    TipAdvanced(u32),
}

impl WalletState {
    pub fn new(tip_height: u32) -> Self {
        WalletState {
            utxos: Vec::new(),
            tip_height,
            checkpoints: vec![tip_height],
        }
    }

    pub fn balance(&self) -> BalanceSummary {
        classify_balance(&self.utxos)
    }

    pub fn apply(&mut self, event: SyncEvent) {
        match event {
            SyncEvent::Found(utxo) => {
                self.utxos.push(utxo);
            }
            SyncEvent::Confirmed { outpoint, height } => {
                if let Some(utxo) = self.utxos.iter_mut().find(|u| u.outpoint == outpoint) {
                    utxo.confirmed = true;
                    utxo.seen_at_height = Some(height);
                }
            }
            SyncEvent::Spent(outpoint) => {
                self.utxos.retain(|u| u.outpoint != outpoint);
            }
            SyncEvent::Reorged { outpoint } => {
                if let Some(utxo) = self.utxos.iter_mut().find(|u| u.outpoint == outpoint) {
                    utxo.confirmed = false;
                    utxo.seen_at_height = None;
                }
            }
            SyncEvent::TipAdvanced(height) => {
                self.tip_height = height;
                self.checkpoints.push(height);
            }
        }
    }

    pub fn rollback_to_height(&mut self, height: u32) {
        self.checkpoints.retain(|&h| h <= height);
        for utxo in self.utxos.iter_mut() {
            if utxo.seen_at_height.map_or(false, |h| h > height) {
                utxo.confirmed = false;
                utxo.seen_at_height = None;
            }
        }
        self.tip_height = height;
    }
}
